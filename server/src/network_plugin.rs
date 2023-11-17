use std::{net::UdpSocket, time::SystemTime};

use bevy::{
    app::{FixedUpdate, Plugin},
    ecs::{
        event::EventReader,
        schedule::{common_conditions::resource_exists, IntoSystemConfigs},
        system::ResMut,
    },
    log::info,
};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use common::network::configuration::{PROTOCOL_ID, SERVER_SOCKET_ADDRESS};

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
        app.add_systems(
            FixedUpdate,
            (send_messages, receive_messages, handle_events)
                .run_if(resource_exists::<RenetServer>()),
        );
    }
}

fn send_messages(mut server: ResMut<RenetServer>) {
    server.broadcast_message(DefaultChannel::ReliableOrdered, "Server Message");
}

fn receive_messages(mut server: ResMut<RenetServer>) {
    for client in server.clients_id() {
        while let Some(message) = server.receive_message(client, DefaultChannel::ReliableOrdered) {
            info!("Got a message from {}", client);
        }
    }
}

fn handle_events(mut server_events: EventReader<ServerEvent>) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                info!("Client connected: {}", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Client Disconnected: {} ({})", client_id, reason);
            }
        }
    }
}
