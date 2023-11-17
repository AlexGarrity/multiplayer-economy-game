extern crate common;

mod network;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use common::loaders::KdlAsset;
use common::loaders::KdlLoader;

#[derive(Component)]
struct Tool {
    pub cooldown: f32,
}

fn controls(
    time: Res<Time<Fixed>>,
    keys: Res<Input<KeyCode>>,
    mut cameras: Query<(&mut Transform, &mut OrthographicProjection)>,
) {
    let horizontal = {
        let mut x = 0.0;
        if keys.pressed(KeyCode::A) {
            x -= 1.0;
        }
        if keys.pressed(KeyCode::D) {
            x += 1.0;
        }
        x * 200.0 * time.delta_seconds()
    };

    let vertical = {
        let mut y = 0.0;
        if keys.pressed(KeyCode::S) {
            y -= 1.0;
        }
        if keys.pressed(KeyCode::W) {
            y += 1.0;
        }
        y * 200.0 * time.delta_seconds()
    };

    let zoom = {
        let mut zoom = 0.0;
        if keys.pressed(KeyCode::Z) {
            zoom += 1.0
        }
        if keys.pressed(KeyCode::X) {
            zoom -= 1.0
        }
        zoom * time.delta_seconds()
    };

    for (mut transform, mut projection) in &mut cameras {
        transform.translation += Vec3::new(horizontal, vertical, 0.0);
        projection.scale *= zoom + 1.0;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    LoadConfigs,
    Loading,
    Gameplay,
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
        .add_systems(Startup, setup);

    app.run();
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
