use crate::units::{Density, Mass, UnitT};

use super::{MaterialID, MaterialManager, MaterialProperties};

#[derive(Debug)]
pub struct CompoundMaterialInstance {
    pub properties: MaterialID,
    pub ratios: Vec<f32>,
    pub density: Density,
}

impl CompoundMaterialInstance {
    pub fn new(
        properties: MaterialID,
        ratios: Vec<f32>,
        material_manager: &MaterialManager,
    ) -> Self {
        let density = calculate_compound_density(&properties, &ratios, material_manager);

        Self {
            properties,
            ratios,
            density,
        }
    }
}

fn calculate_compound_density(
    properties: &MaterialID,
    ratios: &Vec<f32>,
    material_manager: &MaterialManager,
) -> Density {
    let properties = material_manager
        .get_material(*properties)
        .expect("Material instance is referencing a property set that doesn't exist");

    if let MaterialProperties::Compound(material) = properties {
        let mut i = 0;
        let densities: Vec<Density> = material
            .composition
            .iter()
            .filter_map(|v| {
                let component = material_manager.get_material(v.id).unwrap();
                let result = match component {
                    MaterialProperties::Basic(mat) => {
                        let f_mass = mat.density.mass.as_milligrams() as f64;
                        let adjusted_mass = (f_mass * *ratios.get(i).unwrap() as f64) as UnitT;
                        Some(Density::new(
                            Mass::from_milligrams(adjusted_mass),
                            mat.density.volume,
                        ))
                    }
                    _ => None,
                };
                i += 1;
                result
            })
            .collect();

        let total_density: Density = densities.iter().sum();

        total_density / material.composition.len() as UnitT
    } else {
        unreachable!()
    }
}
