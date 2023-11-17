use std::fmt::Display;

use crate::units::{HeatCapacity, Temperature};

pub struct ThermalProperties {
    pub heat_capacity: HeatCapacity,
    pub melting_point: Temperature,
    pub boiling_point: Temperature,
}

impl Display for ThermalProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "\n[\n\tSHC:\t{}\n\tMP:\t{}\n\tBP:\t{}\n]",
            self.heat_capacity, self.melting_point, self.boiling_point
        ))
    }
}
