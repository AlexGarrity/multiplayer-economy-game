use super::{BasicMaterialProperties, CompoundMaterialProperties};

pub enum MaterialProperties {
    Basic(BasicMaterialProperties),
    Compound(CompoundMaterialProperties),
}
