use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Component, Resource, Clone, Copy)]
pub struct PlayerInput {
    pub left: u8,
    pub right: u8,
    pub forward: u8,
    pub backward: u8,
    pub jump: u8,
    pub crouch: u8,
    pub primary: u8,
    pub secondary: u8,
}
