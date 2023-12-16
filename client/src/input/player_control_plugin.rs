use bevy::{
    app::{Plugin, Update},
    ecs::{
        schedule::{common_conditions::in_state, IntoSystemConfigs},
        system::{Res, ResMut},
    },
    input::{keyboard::KeyCode, Input}, prelude::default,
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
    let new_input = PlayerInput {
        left: keyboard.pressed(KeyCode::A) as u8,
        right: keyboard.pressed(KeyCode::D) as u8,
        forward: keyboard.pressed(KeyCode::W) as u8,
        backward: keyboard.pressed(KeyCode::S) as u8,
        ..default()
    };

    if input.left != new_input.left {
        input.left = new_input.left;
    }
    if input.right != new_input.right {
        input.right = new_input.right;
    }
    if input.forward != new_input.forward {
        input.forward = new_input.forward;
    }
    if input.backward != new_input.backward {
        input.backward = new_input.backward;
    }
}
