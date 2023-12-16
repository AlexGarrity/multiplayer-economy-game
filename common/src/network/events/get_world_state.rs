use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GetWorldState {
    pub world: u64,
}

impl Display for GetWorldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("GetWorldState for World {}", self.world))
    }
}