extern crate common;

mod input;
mod network;

use bevy::input::keyboard::KeyboardInput;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::camera;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_renet::client_connected;
use common::loaders::KdlAsset;
use common::loaders::KdlLoader;
use common::network::events::CreateEntity;
use common::network::events::GetPlayerEntity;
use common::network::events::GetWorldState;
use common::network::events::PlayerInput;
use network::EntityMapper;
use network::ReceiveFromServer;
use network::SendToServer;

use crate::input::PlayerControlPlugin;

#[derive(Resource, Default)]
struct PlayerEntity(Option<Entity>);

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
    app.add_plugins(DefaultPlugins.build().set::<LogPlugin>(LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "client=debug,error".into()
    }))
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
    .add_systems(Update, advance_state.run_if(in_state(GameState::Loading)).run_if(client_connected()))
    .add_plugins(PlayerControlPlugin)
    .insert_resource(PlayerEntity::default());

    app.add_systems(Update, create_entity_system)
        .add_systems(
            PreUpdate,
            send_user_input.run_if(resource_changed::<PlayerInput>()),
        )
        .add_systems(
            Update,
            set_player_entity.run_if(in_state(GameState::Gameplay)),
        )
        .add_systems(
            PostUpdate,
            track_camera.run_if(in_state(GameState::Gameplay)),
        )
        .add_systems(
            OnEnter(GameState::Gameplay),
            (get_world_state, get_player_entity),
        )
        .add_systems(Update, print_player_entity);

    app.run();
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn print_player_entity(player_entity_resource: Res<PlayerEntity>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_released(KeyCode::X) {
        info!("Player entity is {:?}", player_entity_resource.0);
    }
}

fn get_world_state(mut get_world_state_event_writer: EventWriter<SendToServer<GetWorldState>>) {
    get_world_state_event_writer.send(SendToServer {
        message: GetWorldState { world: 0 },
    });
}

fn get_player_entity(
    mut get_player_entity_event_writer: EventWriter<SendToServer<GetPlayerEntity>>,
) {
    get_player_entity_event_writer.send(SendToServer {
        message: GetPlayerEntity { world: 0 },
    });
}

fn set_player_entity(
    mut player_entity_event_reader: EventReader<
        ReceiveFromServer<common::network::events::PlayerEntity>,
    >,
    mut player_entity_resource: ResMut<PlayerEntity>,
    mapper: Res<EntityMapper>,
) {
    for event in player_entity_event_reader.read() {
        info!("Got player entity {:?}", event.message.entity);
        player_entity_resource.0 = mapper.0.get(&event.message.entity).copied();
    }
}

fn create_entity_system(
    mut create_entity_event_reader: EventReader<ReceiveFromServer<CreateEntity>>,
    mut commands: Commands,
    mut mapper: ResMut<EntityMapper>,
) {
    for create_entity_event in create_entity_event_reader.read() {
        info!("Creating entity {:?}", create_entity_event.message.entity);
        let new_entity = commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.1, 0.3, 0.7, 1.0),
                    custom_size: Some(Vec2 { x: 25.0, y: 25.0 }),
                    ..default()
                },
                transform: Transform::IDENTITY,
                ..default()
            })
            .id();

        mapper
            .0
            .insert(create_entity_event.message.entity, new_entity);
    }
}

fn send_user_input(
    input: Res<PlayerInput>,
    mut input_event_writer: EventWriter<SendToServer<PlayerInput>>,
) {
    input_event_writer.send(SendToServer {
        message: input.clone(),
    });
}

fn track_camera(
    player_entity_resource: Res<PlayerEntity>,
    mut camera_transforms: Query<&mut Transform, With<Camera2d>>,
    entities: Query<(Entity, &Transform), Without<Camera2d>>,
) {
    if let Some(player_entity) = player_entity_resource.0 {
        let player_entity_transform = entities.get_component::<Transform>(player_entity);
        match player_entity_transform {
            Ok(player_transform) => {
                for mut camera_transform in &mut camera_transforms {
                    camera_transform.translation.x = player_transform.translation.x;
                    camera_transform.translation.y = player_transform.translation.y;
                    camera_transform.translation.z = player_transform.translation.z - 1000.0;
                }
            }
            Err(_) => {
                warn!("Failed to track camera as the specified player entity doesn't exist");
            }
        }
    }
}
