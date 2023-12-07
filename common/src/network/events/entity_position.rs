use serde::{Serialize, Deserialize};
use bevy::ecs::entity::Entity;

#[derive(Serialize, Deserialize)]
pub struct EntityPosition {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
    pub z: f32
}