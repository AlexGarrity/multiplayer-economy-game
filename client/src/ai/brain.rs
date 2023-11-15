use bevy::prelude::{Component, Entity, Query, Res, Transform, Vec3};

use crate::units::Volume;

#[derive(Debug, PartialEq, Eq)]
pub enum AiState {
    Gather,
    Mining,
    Return,
    Rest,
    Sell,
}

#[derive(Component)]
pub struct Brain {
    pub id: u32,
    pub state: AiState,
    pub target_entity: Option<Entity>,
    pub target_position: Option<Vec3>,
}
