use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct GetPlayerEntity {
    pub world: u64,
}

impl Display for GetPlayerEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("GetPlayerEntity for World {}", self.world))
    }
}
