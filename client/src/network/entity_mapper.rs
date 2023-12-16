use bevy::{utils::HashMap, ecs::{entity::Entity, system::Resource}};

#[derive(Resource, Default)]
pub struct EntityMapper(pub HashMap<Entity, Entity>);