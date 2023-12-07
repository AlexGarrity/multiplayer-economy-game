use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Res, ResMut},
    },
    input::{keyboard::KeyCode, Input},
};

use common::network::events::PlayerInput;

use crate::GameState;

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            handle_player_input.run_if(in_state(GameState::Gameplay)),
        );
        app.init_resource::<PlayerInput>();
    }
}

fn handle_player_input(keyboard: Res<Input<KeyCode>>, mut input: ResMut<PlayerInput>) {
    input.left = keyboard.pressed(KeyCode::A) as u8;
    input.right = keyboard.pressed(KeyCode::D) as u8;
    input.forward = keyboard.pressed(KeyCode::W) as u8;
    input.backward = keyboard.pressed(KeyCode::S) as u8;
}
