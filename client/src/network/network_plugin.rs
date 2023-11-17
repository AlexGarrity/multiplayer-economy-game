use std::{net::UdpSocket, time::SystemTime};

use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    log::{info, warn},
    math::Vec2,
    render::color::Color,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
    utils::default,
};
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig, DefaultChannel, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin,
};
use common::{
    input::PlayerInput,
    network::{
        configuration::{CLIENT_SOCKET_ADDRESS, PROTOCOL_ID, SERVER_SOCKET_ADDRESS},
        EntityMapper, NetworkMessages, Position,
    },
};

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let socket = UdpSocket::bind(CLIENT_SOCKET_ADDRESS).unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let auth = ClientAuthentication::Unsecure {
            protocol_id: PROTOCOL_ID,
            client_id: current_time.as_millis() as u64,
            server_addr: SERVER_SOCKET_ADDRESS,
            user_data: None,
        };

        let transport = NetcodeClientTransport::new(current_time, auth, socket).unwrap();
        let client = RenetClient::new(ConnectionConfig::default());

        app.add_plugins(RenetClientPlugin);
        app.add_plugins(NetcodeClientPlugin);
        app.insert_resource(client);
        app.insert_resource(transport);
        app.init_resource::<EntityMapper>();

        app.add_systems(
            Update,
            (send_messages, receive_messages).run_if(client_connected()),
        );
    }
}

fn send_messages(mut client: ResMut<RenetClient>, input: Res<PlayerInput>) {
    let input_message = NetworkMessages::PlayerInput(*input);
    let message = bincode::serialize(&input_message).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, message);
}

fn receive_messages(
    mut client: ResMut<RenetClient>,
    mut mapper: ResMut<EntityMapper>,
    mut positions: Query<(&mut Transform, &mut PlayerInput)>,
    mut commands: Commands,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let deserialised_message = bincode::deserialize(&message);
        if deserialised_message.is_err() {
            warn!("Got a bad message: {:?}", message);
            continue;
        }

        let network_message: NetworkMessages = deserialised_message.unwrap();

        match network_message {
            NetworkMessages::Position(pos) => {
                handle_position_message(&mut mapper, &mut commands, pos, &mut positions);
            }
            _ => {
                todo!()
            }
        }
    }
}

fn handle_position_message(
    mapper: &mut ResMut<EntityMapper>,
    commands: &mut Commands,
    pos: Position,
    positions: &mut Query<(&mut Transform, &mut PlayerInput)>,
) {
    let entity = {
        match mapper.entities.get(&pos.client) {
            Some(e) => *e,
            None => {
                let entity = commands
                    .spawn((
                        SpriteBundle {
                            sprite: Sprite {
                                color: Color::hsl(60.0, 0.5, 0.5),
                                custom_size: Some(Vec2::new(50.0, 50.0)),
                                ..Default::default()
                            },
                            transform: Transform::IDENTITY,
                            ..default()
                        },
                        PlayerInput::default(),
                    ))
                    .id();
                mapper.entities.insert(pos.client, entity);
                entity
            }
        }
    };

    let position = positions.get_mut(entity);
    if let Ok((mut transform, mut input)) = position {
        info!("{:?}", transform.as_ref());
        transform.translation.x = pos.pos[0];
        transform.translation.y = pos.pos[1];
        transform.translation.z = pos.pos[2];
    }
}
