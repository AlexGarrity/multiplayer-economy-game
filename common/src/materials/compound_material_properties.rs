use std::fmt::Display;

use super::material_manager::MaterialID;

pub struct ComposingMaterial {
    pub id: MaterialID,
    pub mean: f32,
    pub sd: f32,
}

impl Display for ComposingMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "[ ID {}\tMean: {:.2}\tSD: {:.2}]",
            self.id, self.mean, self.sd
        ))
    }
}

pub struct CompoundMaterialProperties {
    pub name: String,
    pub composition: Vec<ComposingMaterial>,
}

impl Display for CompoundMaterialProperties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_fmt(format_args!("Name: {}\nComposition: [\n", self.name));
        for component in &self.composition {
            let _ = f.write_fmt(format_args!("\t{}\n", component));
        }
        f.write_str("]\n")
    }
}
