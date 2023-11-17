use std::fmt::Display;

use crate::units::Density;

use super::ThermalProperties;

pub struct BasicMaterialProperties {
    pub name: String,
    pub density: Density,
    pub thermal_properties: ThermalProperties,
}

impl Display for BasicMaterialProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Name:\t\t{}\nDensity:\t{}\nThermals:\t{}\n",
            self.name.as_str(),
            self.density,
            self.thermal_properties
        ))
    }
}
