mod basic_material_instance;
mod basic_material_properties;
mod compound_material_instance;
mod compound_material_properties;
mod material_instance;
mod material_manager;
mod material_properties;
mod thermal_properties;

pub use basic_material_instance::BasicMaterialInstance;
pub use basic_material_properties::BasicMaterialProperties;
pub use compound_material_instance::CompoundMaterialInstance;
pub use compound_material_properties::ComposingMaterial;
pub use compound_material_properties::CompoundMaterialProperties;
pub use material_instance::MaterialInstance;
pub use material_manager::MaterialID;
pub use material_manager::MaterialManager;
pub use material_properties::MaterialProperties;
pub use thermal_properties::ThermalProperties;
