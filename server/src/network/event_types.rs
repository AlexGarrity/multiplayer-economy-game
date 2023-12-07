use bevy::ecs::event::Event;
use bevy_renet::renet::ClientId;
use serde::{Serialize, Deserialize};

#[derive(Event)]
pub struct SendToClient<T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static> {
    pub client: Option<ClientId>,
    pub message: T
}

#[derive(Event)]
pub struct ReceiveFromClient<T: Serialize + for<'a> Deserialize<'a> + Sync + Send + 'static> {
    pub client: ClientId,
    pub message: T
}