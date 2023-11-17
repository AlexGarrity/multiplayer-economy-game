extern crate bevy;
extern crate bevy_asset_loader;
extern crate bevy_renet;
extern crate common;

mod network_plugin;

use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy::{log::LogPlugin, prelude::*, time::common_conditions::on_timer};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_renet::{
    renet::{
        transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
        ConnectionConfig, DefaultChannel, RenetServer, ServerEvent,
    },
    transport::NetcodeServerPlugin,
    RenetServerPlugin,
};
use common::network::configuration::{CLIENT_SOCKET_ADDRESS, PROTOCOL_ID, SERVER_SOCKET_ADDRESS};
use network_plugin::NetworkPlugin;

#[derive(Default, States, Clone, Debug, Hash, PartialEq, Eq)]
enum ServerState {
    #[default]
    LoadingData,
    GeneratingAssets,
    Running,
}

fn print_version() {
    info!(
        "Server v{} running common v{}",
        version(),
        common::version()
    );
}

fn main() {
    let mut app = App::new();
    app.add_state::<ServerState>()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .add_loading_state(
            LoadingState::new(ServerState::LoadingData)
                .continue_to_state(ServerState::GeneratingAssets),
        )
        .add_systems(OnEnter(ServerState::LoadingData), print_version)
        .add_plugins(NetworkPlugin);

    app.run();
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
