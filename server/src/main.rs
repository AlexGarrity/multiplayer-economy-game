extern crate bevy;
extern crate bevy_asset_loader;
extern crate bevy_renet;
extern crate common;

mod network;

use std::time::Duration;

use bevy::{log::LogPlugin, prelude::*, time::common_conditions::on_timer};
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_renet::client_connected;
use common::network::events::{
    CreateEntity, EntityPosition, GetPlayerEntity, GetWorldState, PlayerEntity, PlayerInput,
};
use network::{ClientEntityMapper, NetworkPlugin, ReceiveFromClient, SendToClient};

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
        .add_plugins((
            MinimalPlugins,
            LogPlugin {
                level: bevy::log::Level::DEBUG,
                filter: "server=debug,error".into()
            },
        ))
        .insert_resource(Time::<Fixed>::from_seconds(0.1))
        .add_loading_state(
            LoadingState::new(ServerState::LoadingData)
                .continue_to_state(ServerState::GeneratingAssets),
        )
        .add_systems(OnEnter(ServerState::LoadingData), print_version)
        .add_systems(OnEnter(ServerState::GeneratingAssets), generate_assets)
        .add_systems(
            FixedUpdate,
            (input_system).run_if(in_state(ServerState::Running)),
        )
        .add_systems(
            Update,
            (send_player_entity, send_world_state, send_positions, update_input)
                .run_if(in_state(ServerState::Running)),
        )
        .add_plugins(NetworkPlugin);

    app.run();
}

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn send_positions(
    query: Query<(Entity, &Transform), Changed<Transform>>,
    mut entity_position_event_writer: EventWriter<SendToClient<EntityPosition>>,
) {
    let events = query.iter().map(|(entity, transform)| SendToClient {
        client: None,
        message: EntityPosition {
            entity,
            x: transform.translation.x,
            y: transform.translation.y,
            z: transform.translation.z,
        },
    });
    entity_position_event_writer.send_batch(events);
}

fn input_system(time: Res<Time<Fixed>>, mut query: Query<(Entity, &PlayerInput, &mut Transform)>) {
    for (entity, input, mut transform) in &mut query {
        if !input.any() {
            continue;
        }

        let delta_x = {
            let mut x = 0.0;
            if input.left > 0 {
                x -= time.delta_seconds() * 50.0;
            }
            if input.right > 0 {
                x += time.delta_seconds() * 50.0
            }
            x
        };
        let delta_y = {
            let mut y = 0.0;
            if input.backward > 0 {
                y -= time.delta_seconds() * 50.0;
            }
            if input.forward > 0 {
                y += time.delta_seconds() * 50.0
            }
            y
        };

        if delta_x.abs() != 0.0 || delta_y.abs() != 0.0 {
            debug!("New position for {:?} is {:?}", entity, transform.translation);
            transform.translation.x += delta_x;
            transform.translation.y += delta_y;
        }
    }
}

fn send_player_entity(
    mut get_player_entity_events: EventReader<ReceiveFromClient<GetPlayerEntity>>,
    mut send_player_entity_events: EventWriter<SendToClient<PlayerEntity>>,
    mapper: Res<ClientEntityMapper>,
) {
    for event in get_player_entity_events.read() {
        info!("Sending player entity to {}", event.client);
        let server_entity = mapper.clients.get(&event.client.raw());
        match server_entity {
            Some(entity) => send_player_entity_events.send(SendToClient {
                client: Some(event.client),
                message: PlayerEntity {
                    entity: entity.clone(),
                },
            }),
            None => {
                warn!("No entity mapped to client");
            }
        }
    }
}

fn send_world_state(
    mut get_world_state_events: EventReader<ReceiveFromClient<GetWorldState>>,
    mut create_entity_events: EventWriter<SendToClient<CreateEntity>>,
    entities: Query<Entity, With<Transform>>,
) {
    for event in get_world_state_events.read() {
        for entity in &entities {
            create_entity_events.send(SendToClient {
                client: Some(event.client),
                message: CreateEntity { entity },
            });
        }
    }
}

fn update_input(mut input_events: EventReader<ReceiveFromClient<PlayerInput>>, mut query: Query<&mut PlayerInput>, mapper: Res<ClientEntityMapper>) {
    for event in input_events.read() {
        match mapper.clients.get(&event.client.raw()) {
            Some (entity) => {
                match query.get_component_mut::<PlayerInput>(*entity) {
                    Ok (mut input) => {
                        input.set_if_neq(event.message);
                    },
                    Err(error) => {
                        warn!("Failed to get component PlayerInput on entity {:?} ({})", entity, error);
                    }
                }
            },
            None => {
                warn!("Tried to update player input for client {} but they don't have an entity associated", event.client);
            },
        }
    }
}