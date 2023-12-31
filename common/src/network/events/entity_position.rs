use std::fmt::Display;

use serde::{Serialize, Deserialize};
use bevy::ecs::entity::Entity;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct EntityPosition {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Display for EntityPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("EntityPosition ({:?}) = {{ {}, {}, {} }}", self.entity, self.x, self.y, self.z))
    }
}