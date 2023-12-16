use std::fmt::Display;

use bevy::ecs::{component::Component, system::Resource};
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Component, Resource, Clone, Copy, PartialEq, Eq)]
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

impl Display for PlayerInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Input {:03} {:03} {:03} {:03} {:03} {:03} {:03} {:03}",
            self.left,
            self.right,
            self.forward,
            self.backward,
            self.jump,
            self.crouch,
            self.primary,
            self.secondary
        ))
    }
}

impl PlayerInput {
    pub fn any(self) -> bool {
        self.left != 0 || self.right != 0 || self.forward != 0 || self.backward != 0 || self.jump != 0 || self.crouch != 0 || self.primary != 0 || self.secondary != 0
    }
}