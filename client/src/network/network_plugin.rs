use std::{net::UdpSocket, time::SystemTime};

use bevy::{
    app::{Plugin, Update, App},
    ecs::{
        schedule::{IntoSystemConfigs, common_conditions::{resource_exists, in_state}},
        system::ResMut, event::{EventReader, EventWriter},
    },
    log::warn,
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
use common::
    network::{
        configuration::{CLIENT_SOCKET_ADDRESS, PROTOCOL_ID, SERVER_SOCKET_ADDRESS}, events::{PlayerInput, EntityPosition}}
    
;
use serde::{Serialize, Deserialize};

use crate::GameState;

use super::{event_types::{ReceiveFromServer, SendToServer}, entity_mapper::EntityMapper};

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
                self.add_event::<SendToServer<T>>();
                self.add_systems(
                    Update,
                    (send_messages::<T>)
                        .run_if(resource_exists::<RenetClient>())
                        .run_if(in_state(GameState::Gameplay)),
                );
            }
            NetworkEventDirection::Receive => {
                self.add_event::<ReceiveFromServer<T>>();
                self.add_systems(
                    Update,
                    (receive_messages::<T>)
                        .run_if(resource_exists::<RenetClient>())
                        .run_if(in_state(GameState::Gameplay)),
                );
            }
            NetworkEventDirection::Both => {
                self.add_event::<SendToServer<T>>();
                self.add_event::<ReceiveFromServer<T>>();
                self.add_systems(
                    Update,
                    (send_messages::<T>, receive_messages::<T>)
                        .run_if(resource_exists::<RenetClient>())
                        .run_if(in_state(GameState::Gameplay)),
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

        app
            .register_network_event::<PlayerInput>(NetworkEventDirection::Send)
            .register_network_event::<EntityPosition>(NetworkEventDirection::Receive);
    }
}

fn send_messages<T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static>(
    mut client: ResMut<RenetClient>,
    mut reader: EventReader<SendToServer<T>>
) {
    reader.read().for_each(|event| {
        let serialisation_result = bincode::serialize(&event.message);
        match serialisation_result {
            Ok (serialised_message) => {
                client.send_message(DefaultChannel::ReliableOrdered, serialised_message);
            },
            Err (serialisation_error) => {
                warn!("Tried to serialise a message but failed ({})", serialisation_error);
            }
        }
    })
}

fn receive_messages<T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static>(
    mut client: ResMut<RenetClient>,
    mut writer: EventWriter<ReceiveFromServer<T>>
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let deserialisation_result = bincode::deserialize::<T>(&message);
        match deserialisation_result {
            Ok(deserialised_message) => {
                writer.send(ReceiveFromServer { message: deserialised_message });
            },
            Err(deserialisation_error) => {
                warn!("Failed to deserialise a message from the server ({})", deserialisation_error);
            }
        }
    }
}