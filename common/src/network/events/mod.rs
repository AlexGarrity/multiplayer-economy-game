mod entity_position;
mod player_input;
mod create_entity;
mod destroy_entity;
mod get_world_state;
mod get_player_entity;
mod player_entity;

pub use entity_position::EntityPosition;
pub use player_input::PlayerInput;
pub use create_entity::CreateEntity;
pub use destroy_entity::DestroyEntity;
pub use get_world_state::GetWorldState;
pub use get_player_entity::GetPlayerEntity;
pub use player_entity::PlayerEntity;

pub enum Events {
    PlayerInput(PlayerInput),
    EntityPosition(EntityPosition),

    CreateEntity(CreateEntity),
    DestroyEntity(DestroyEntity),

    GetWorldState(GetWorldState),
    
    GetPlayerEntity(GetPlayerEntity),
    PlayerEntity(PlayerEntity),
}