use bevy::prelude::Component;

#[derive(Component)]
pub struct PhysicalCharacteristics {
    pub move_speed: f32,
    pub mining_cooldown: f32,
}
