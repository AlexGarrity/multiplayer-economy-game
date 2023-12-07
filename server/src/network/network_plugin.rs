use std::{collections::VecDeque, net::UdpSocket, time::SystemTime};

use bevy::{
    app::{App, FixedUpdate, Plugin, Update},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::{
            common_conditions::{in_state, resource_exists},
            IntoSystemConfigs,
        },
        system::{Commands, ResMut},
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

use common::network::{
    configuration::{PROTOCOL_ID, SERVER_SOCKET_ADDRESS},
    events::{EntityPosition, PlayerInput},
};
use serde::{Deserialize, Serialize};

use crate::{network::ClientMapping, ServerState};

use super::{ClientEntityMapper, ReceiveFromClient, SendToClient};

pub struct NetworkPlugin;

enum NetworkEventDirection {
    Send,
    Receive,
    Both,
}

trait NetworkEventAdder {
    fn register_network_event<T>(&mut self, direction: NetworkEventDirection) -> &mut Self
    where
        T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static;
}

impl NetworkEventAdder for App {
    fn register_network_event<T>(&mut self, direction: NetworkEventDirection) -> &mut Self
    where
        T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static,
    {
        match direction {
            NetworkEventDirection::Send => {
                self.add_event::<SendToClient<T>>();
                self.add_systems(
                    Update,
                    (send_messages::<T>)
                        .run_if(resource_exists::<RenetServer>())
                        .run_if(in_state(ServerState::Running)),
                );
            }
            NetworkEventDirection::Receive => {
                self.add_event::<ReceiveFromClient<T>>();
                self.add_systems(
                    Update,
                    (receive_messages::<T>)
                        .run_if(resource_exists::<RenetServer>())
                        .run_if(in_state(ServerState::Running)),
                );
            }
            NetworkEventDirection::Both => {
                self.add_event::<SendToClient<T>>();
                self.add_event::<ReceiveFromClient<T>>();
                self.add_systems(
                    Update,
                    (send_messages::<T>, receive_messages::<T>)
                        .run_if(resource_exists::<RenetServer>())
                        .run_if(in_state(ServerState::Running)),
                );
            }
        };
        self
    }
}

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
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

        app
            .register_network_event::<PlayerInput>(NetworkEventDirection::Receive)
            .register_network_event::<EntityPosition>(NetworkEventDirection::Send);

        app.add_systems(
            FixedUpdate,
            handle_events
                .run_if(resource_exists::<RenetServer>())
                .run_if(in_state(ServerState::Running)),
        );
    }
}

fn send_messages<T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static>(
    mut server: ResMut<RenetServer>,
    mut reader: EventReader<SendToClient<T>>,
) {
    reader.read().for_each(|event| {
        let serialisation_result = bincode::serialize(&event.message);
        match serialisation_result {
            Ok(serialised_message) => match event.client {
                Some(receiver) => {
                    server.send_message(
                        receiver,
                        DefaultChannel::ReliableOrdered,
                        serialised_message,
                    );
                }
                None => {
                    server.broadcast_message(DefaultChannel::ReliableOrdered, serialised_message);
                }
            },
            Err(serialisation_error) => {
                warn!(
                    "Tried to serialise a message but failed ({})",
                    serialisation_error
                );
            }
        }
    })
}

fn receive_messages<T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static>(
    mut server: ResMut<RenetServer>,
    mut writer: EventWriter<ReceiveFromClient<T>>,
) {
    for client in server.clients_id() {
        while let Some(message) = server.receive_message(client, DefaultChannel::ReliableOrdered) {
            let deserialisation_result = bincode::deserialize::<T>(&message);
            match deserialisation_result {
                Ok(message) => {
                    writer.send(ReceiveFromClient { client, message });
                }
                Err(deserialisation_error) => {
                    warn!(
                        "Failed to deserialise a message from {} ({})",
                        client, deserialisation_error
                    );
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
            ServerEvent::ClientConnected { client_id: client } => {
                info!("Client connected: {}", client);

                let entity = commands
                    .spawn((ClientMapping { id: *client }, Transform::IDENTITY))
                    .id();
                mapper.clients.insert(client.raw(), entity);
            }
            ServerEvent::ClientDisconnected {
                client_id: client,
                reason,
            } => {
                info!("Client Disconnected: {} ({})", client, reason);
                match mapper.clients.get(&client.raw()) {
                    Some(entity) => {
                        commands.entity(*entity).despawn();
                        mapper.clients.remove(&client.raw());
                    }
                    None => {
                        warn!("Client disconnected but lacked an entity");
                    }
                }
            }
        }
    }
}
