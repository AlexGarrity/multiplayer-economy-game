extern crate common;

mod input;
mod network;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common::input::PlayerInput;
use common::loaders::KdlAsset;
use common::loaders::KdlLoader;

use crate::input::PlayerControlPlugin;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum GameState {
    #[default]
    LoadConfigs,
    Loading,
    Gameplay,
}

pub fn advance_state(mut state: ResMut<NextState<GameState>>) {
    state.set(GameState::Gameplay);
}

fn main() {
    info!("Client v{} using common v{}", version(), common::version());

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins((common::materials::MaterialsPlugin, network::NetworkPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<GameState>()
        .init_asset::<KdlAsset>()
        .init_asset_loader::<KdlLoader>()
        .add_loading_state(
            LoadingState::new(GameState::LoadConfigs).continue_to_state(GameState::Loading),
        )
        .add_collection_to_loading_state::<_, common::materials::MaterialConfigs>(
            GameState::LoadConfigs,
        )
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Loading), advance_state)
        .add_plugins(PlayerControlPlugin);

    app.run();
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
