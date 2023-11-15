use super::{BasicMaterialInstance, CompoundMaterialInstance};

#[derive(Debug)]
pub enum MaterialInstance {
    Basic(BasicMaterialInstance),
    Compound(CompoundMaterialInstance),
}
