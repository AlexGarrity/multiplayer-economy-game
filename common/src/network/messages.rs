use crate::input::PlayerInput;
use bevy::ecs::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Position {
    pub client: Entity,
    pub pos: [f32; 3],
}

#[derive(Serialize, Deserialize)]
pub enum NetworkMessages {
    PlayerInput(PlayerInput),
    Position(Position),
    CreateEntity(Entity),
    DestroyEntity(Entity),
}
