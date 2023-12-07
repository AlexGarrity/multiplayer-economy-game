use bevy::{
    ecs::{component::Component, entity::Entity, system::Resource},
    utils::HashMap,
};
use bevy_renet::renet::ClientId;

#[derive(Component)]
pub struct ClientMapping {
    pub id: ClientId,
}

#[derive(Resource, Default)]
pub struct ClientEntityMapper {
    pub clients: HashMap<u64, Entity>,
}
