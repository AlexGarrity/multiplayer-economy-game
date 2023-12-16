use std::{net::UdpSocket, time::SystemTime, fmt::Display, any};

use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::{EventReader, EventWriter},
        schedule::{
            common_conditions::{in_state, resource_exists},
            IntoSystemConfigs,
        },
        system::ResMut,
    },
    log::{warn, debug},
};
use bevy_renet::{
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport},
        ConnectionConfig, DefaultChannel, RenetClient,
    },
    transport::NetcodeClientPlugin,
    RenetClientPlugin, client_connected,
};
use common::network::{
    configuration::{CLIENT_SOCKET_ADDRESS, PROTOCOL_ID, SERVER_SOCKET_ADDRESS},
    events::{
        CreateEntity, DestroyEntity, EntityPosition, GetPlayerEntity, GetWorldState, PlayerEntity,
        PlayerInput,
    },
};
use serde::{Deserialize, Serialize};

use crate::GameState;

use super::{
    entity_mapper::EntityMapper,
    event_types::{ReceiveFromServer, SendToServer},
};

enum NetworkEventDirection {
    Send,
    Receive,
    Both,
}

trait NetworkEventAdder {
    fn register_network_event<T>(&mut self, direction: NetworkEventDirection) -> &mut Self
    where
        T: Display + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static;
}

impl NetworkEventAdder for App {
    fn register_network_event<T>(&mut self, direction: NetworkEventDirection) -> &mut Self
    where
        T: Display + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static
    {
        match direction {
            NetworkEventDirection::Send => {
                self.add_event::<SendToServer<T>>();
                self.add_systems(
                    Update,
                    (send_messages::<T>)
                        .run_if(client_connected())
                        .run_if(in_state(GameState::Gameplay)),
                );
            }
            NetworkEventDirection::Receive => {
                self.add_event::<ReceiveFromServer<T>>();
                self.add_systems(
                    Update,
                    (receive_messages::<T>).run_if(client_connected()),
                );
            }
            NetworkEventDirection::Both => {
                self.add_event::<SendToServer<T>>();
                self.add_event::<ReceiveFromServer<T>>();
                self.add_systems(
                    Update,
                    (send_messages::<T>, receive_messages::<T>)
                        .run_if(client_connected()),
                );
            }
        };
        self
    }
}

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
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

        app.add_plugins((RenetClientPlugin, NetcodeClientPlugin));
        app.insert_resource(client);
        app.insert_resource(transport);
        app.init_resource::<EntityMapper>();

        app.register_network_event::<PlayerInput>(NetworkEventDirection::Send)
            .register_network_event::<EntityPosition>(NetworkEventDirection::Receive)
            .register_network_event::<CreateEntity>(NetworkEventDirection::Receive)
            .register_network_event::<DestroyEntity>(NetworkEventDirection::Receive)
            .register_network_event::<GetWorldState>(NetworkEventDirection::Send)
            .register_network_event::<GetPlayerEntity>(NetworkEventDirection::Send)
            .register_network_event::<PlayerEntity>(NetworkEventDirection::Receive);
    }
}

fn send_messages<T: Display + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static>(
    mut client: ResMut<RenetClient>,
    mut reader: EventReader<SendToServer<T>>,
) {
    reader.read().for_each(move |event| {       
        let serialisation_result = bincode::serde::encode_to_vec(&event.message, bincode::config::standard());
        match serialisation_result {
            Ok(serialised_message) => {
                debug!("Sent a message ({}) (encoded as {:#?})", event.message, serialised_message);
                client.send_message(DefaultChannel::ReliableOrdered, serialised_message);
            }
            Err(serialisation_error) => {
                warn!(
                    "Tried to serialise a message but failed ({})",
                    serialisation_error
                );
            }
        }
    })
}

fn receive_messages<T: Display + Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static>(
    mut client: ResMut<RenetClient>,
    mut writer: EventWriter<ReceiveFromServer<T>>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let deserialisation_result = bincode::serde::decode_from_slice(&message, bincode::config::standard());
        match deserialisation_result {
            Ok((deserialised_message, _)) => {
                debug!("Received a message ({})", deserialised_message);
                writer.send(ReceiveFromServer {
                    message: deserialised_message,
                });
            }
            Err(deserialisation_error) => {
                warn!(
                    "Failed to deserialise a message from the server ({})",
                    deserialisation_error
                );
            }
        }
    }
}
