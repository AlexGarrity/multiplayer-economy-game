extern crate bevy;
extern crate bevy_asset_loader;
extern crate bevy_renet;
extern crate common;

mod network;

use bevy::{log::LogPlugin, prelude::*};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use common::network::events::PlayerInput;
use network::NetworkPlugin;

#[derive(Default, States, Clone, Debug, Hash, PartialEq, Eq)]
pub enum ServerState {
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

pub fn generate_assets(mut state: ResMut<NextState<ServerState>>) {
    state.set(ServerState::Running);
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
        .add_systems(OnEnter(ServerState::GeneratingAssets), generate_assets)
        .add_systems(
            FixedUpdate,
            input_system.run_if(in_state(ServerState::Running)),
        )
        .add_plugins(NetworkPlugin);

    app.run();
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn input_system(time: Res<Time<Fixed>>, mut query: Query<(&PlayerInput, &mut Transform)>) {
    for (input, mut transform) in &mut query {
        transform.translation.x += {
            let mut x = 0.0;
            if input.left > 0 {
                x -= time.delta_seconds() * 50.0;
            }
            if input.right > 0 {
                x += time.delta_seconds() * 50.0
            }
            x
        };
        transform.translation.y += {
            let mut y = 0.0;
            if input.backward > 0 {
                y -= time.delta_seconds() * 50.0;
            }
            if input.forward > 0 {
                y += time.delta_seconds() * 50.0
            }
            y
        };

        info!("{:?}", transform);
    }
}
