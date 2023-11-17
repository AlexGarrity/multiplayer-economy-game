use bevy::{
    ecs::{entity::Entity, system::Resource},
    utils::HashMap,
};

#[derive(Default, Resource)]
pub struct EntityMapper {
    pub entities: HashMap<Entity, Entity>,
}
