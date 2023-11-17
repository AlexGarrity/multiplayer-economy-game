use std::{net::UdpSocket, time::SystemTime};

use bevy::{
    app::{FixedUpdate, Plugin, Update},
    ecs::{
        entity::Entity,
        event::EventReader,
        schedule::{
            common_conditions::{in_state, resource_exists},
            IntoSystemConfigs,
        },
        system::{Commands, Query, Res, ResMut},
    },
    log::{info, warn},
    transform::components::Transform,
};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use common::{
    input::PlayerInput,
    network::{
        configuration::{PROTOCOL_ID, SERVER_SOCKET_ADDRESS},
        EntityMapper, NetworkMessages, Position,
    },
};

use crate::{
    client_entity_mapper::{ClientEntityMapper, ClientMapping},
    ServerState,
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let socket = UdpSocket::bind(SERVER_SOCKET_ADDRESS).unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let server_config = ServerConfig {
            current_time,
            max_clients: 64,
            protocol_id: PROTOCOL_ID,
            public_addresses: vec![SERVER_SOCKET_ADDRESS],
            authentication: ServerAuthentication::Unsecure,
        };

        let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
        let server = RenetServer::new(ConnectionConfig::default());

        app.add_plugins(RenetServerPlugin);
        app.add_plugins(NetcodeServerPlugin);
        app.insert_resource(server);
        app.insert_resource(transport);
        app.init_resource::<ClientEntityMapper>();

        app.add_systems(
            Update,
            (send_messages, receive_messages)
                .run_if(resource_exists::<RenetServer>())
                .run_if(in_state(ServerState::Running)),
        );
        app.add_systems(
            FixedUpdate,
            handle_events
                .run_if(resource_exists::<RenetServer>())
                .run_if(in_state(ServerState::Running)),
        );
    }
}

fn send_messages(
    mut server: ResMut<RenetServer>,
    query: Query<(Entity, &Transform, &PlayerInput)>,
) {
    for (entity, transform, input) in &query {
        let message = NetworkMessages::Position(Position {
            client: entity,
            pos: [
                transform.translation.x,
                transform.translation.y,
                transform.translation.z,
            ],
        });
        let coded_message = bincode::serialize(&message);
        if let Ok(m) = coded_message {
            server.broadcast_message(DefaultChannel::ReliableOrdered, m);
        }
    }
}

fn receive_messages(
    mut server: ResMut<RenetServer>,
    mapper: Res<ClientEntityMapper>,
    mut player_inputs: Query<&mut PlayerInput>,
) {
    for client in server.clients_id() {
        while let Some(message) = server.receive_message(client, DefaultChannel::ReliableOrdered) {
            let parsed_message = bincode::deserialize(&message);
            if parsed_message.is_err() {
                warn!("Got a bad message from {}: {:?}", client, message);
                continue;
            }
            match parsed_message.unwrap() {
                NetworkMessages::PlayerInput(input) => match mapper.clients.get(&client) {
                    Some(entity) => match player_inputs.get_mut(*entity) {
                        Ok(mut player_input) => {
                            player_input.left = input.left;
                            player_input.right = input.right;
                            player_input.forward = input.forward;
                            player_input.backward = input.backward;
                        }
                        Err(error) => {
                            warn!("{}", error)
                        }
                    },
                    None => {
                        warn!("Received a message from a client that has no entity")
                    }
                },
                _ => {
                    todo!()
                }
            }
        }
    }
}

fn handle_events(
    mut server_events: EventReader<ServerEvent>,
    mut mapper: ResMut<ClientEntityMapper>,
    mut commands: Commands,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client connected: {}", client_id);

                let entity = commands
                    .spawn((
                        ClientMapping { id: *client_id },
                        Transform::IDENTITY,
                        PlayerInput::default(),
                    ))
                    .id();
                mapper.clients.insert(*client_id, entity);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client Disconnected: {} ({})", client_id, reason);
                match mapper.clients.get(client_id) {
                    Some(entity) => {
                        commands.entity(*entity).despawn();
                        mapper.clients.remove(client_id);
                    }
                    None => {
                        warn!("Client disconnected but lacked an entity");
                    }
                }
            }
        }
    }
}
