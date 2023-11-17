use std::{collections::HashMap, fmt::Debug, fmt::Display};

use bevy::ecs::system::Resource;
use rand::{distributions::Standard, thread_rng, Rng};
use rand_distr::{Distribution, Normal};

use super::{
    BasicMaterialInstance, CompoundMaterialInstance, MaterialInstance, MaterialProperties,
};

#[derive(Default, Hash, PartialEq, Eq, Clone, Copy)]
pub struct MaterialID(u64);

impl Display for MaterialID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("MATERIAL_ID({:04x})", self.0 >> 48))
    }
}

impl Debug for MaterialID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("MATERIAL_ID({:016x})", self.0))
    }
}

impl Distribution<MaterialID> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> MaterialID {
        MaterialID(rng.gen())
    }
}

#[derive(Default, Resource)]
pub struct MaterialManager {
    materials: HashMap<MaterialID, MaterialProperties>,
}

impl MaterialManager {
    pub fn register_material(&mut self, material: MaterialProperties) {
        let id = {
            let mut id: MaterialID;
            let mut random = thread_rng();
            loop {
                id = random.gen::<MaterialID>();
                if !self.materials.contains_key(&id) {
                    break;
                }
            }
            id
        };

        self.materials.insert(id, material);
    }

    pub fn get_material(&self, id: MaterialID) -> Option<&MaterialProperties> {
        self.materials.get(&id)
    }

    pub fn get_material_id(&self, name: &str) -> Option<MaterialID> {
        for (id, material) in &self.materials {
            match material {
                MaterialProperties::Basic(mat) => {
                    if mat.name == name {
                        return Some(*id);
                    }
                }
                MaterialProperties::Compound(mat) => {
                    if mat.name == name {
                        return Some(*id);
                    }
                }
            }
        }

        None
    }

    pub fn generate_material_instance(&self, id: MaterialID) -> Option<MaterialInstance> {
        if let Some(material) = self.materials.get(&id) {
            match material {
                MaterialProperties::Basic(_) => {
                    Some(MaterialInstance::Basic(BasicMaterialInstance(id)))
                }
                MaterialProperties::Compound(mat) => {
                    let mut random = thread_rng();
                    let ratios: Vec<f32> = mat
                        .composition
                        .iter()
                        .map(|component| {
                            let distribution = Normal::new(component.mean, component.sd).unwrap();
                            distribution.sample(&mut random)
                        })
                        .collect();

                    let ratio_sum: f32 = ratios.iter().sum();
                    let ratios = ratios.iter().map(|v| v / ratio_sum).collect();

                    Some(MaterialInstance::Compound(CompoundMaterialInstance::new(
                        id, ratios, self,
                    )))
                }
            }
        } else {
            None
        }
    }
}

impl Display for MaterialManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = f.write_str("Material Manager Contents: \n");
        for (id, material) in &self.materials {
            let _ = f.write_fmt(format_args!("{}:\n", id));
            match material {
                MaterialProperties::Basic(mat) => {
                    let _ = f.write_fmt(format_args!("{}", mat));
                }
                MaterialProperties::Compound(mat) => {
                    let _ = f.write_fmt(format_args!("{}", mat));
                }
            }
            let _ = f.write_str("\n");
        }
        Ok(())
    }
}
