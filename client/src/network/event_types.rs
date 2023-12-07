use bevy::ecs::event::Event;
use bevy_renet::renet::ClientId;
use serde::{Serialize, Deserialize};

#[derive(Event)]
pub struct SendToServer<T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static> {
    pub message: T
}

#[derive(Event)]
pub struct ReceiveFromServer<T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static> {
    pub message: T
}