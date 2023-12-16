use std::fmt::Display;

use serde::{Serialize, Deserialize};
use bevy::ecs::entity::Entity;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct CreateEntity {
    pub entity: Entity,
}

impl Display for CreateEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("CreateEntity: {:?}", self.entity))
    }
}