use std::{
    net::{SocketAddr, SocketAddrV4, UdpSocket},
    thread::current,
    time::SystemTime,
};

use bevy::{
    app::{Plugin, Update},
    ecs::{schedule::IntoSystemConfigs, system::ResMut},
    log::info,
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
use common::network::configuration::{CLIENT_SOCKET_ADDRESS, PROTOCOL_ID, SERVER_SOCKET_ADDRESS};

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

        app.add_systems(
            Update,
            (send_messages, receive_messages).run_if(client_connected()),
        );
    }
}

fn send_messages(mut client: ResMut<RenetClient>) {
    client.send_message(DefaultChannel::ReliableOrdered, "Test message");
}

fn receive_messages(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        info!("{:?}", message);
    }
}
