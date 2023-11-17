use bevy::{
    app::{Plugin, PostStartup},
    asset::{self, Assets, UntypedHandle},
    ecs::system::{Res, ResMut, Resource},
    log::warn,
};
use bevy_asset_loader::asset_collection::AssetCollection;
use kdl::KdlNode;

use crate::loaders::KdlAsset;
use crate::units::{Density, Energy, HeatCapacity, Mass, Temperature, Volume};

use super::{
    BasicMaterialProperties, ComposingMaterial, CompoundMaterialProperties, MaterialManager,
    MaterialProperties, ThermalProperties,
};

pub struct MaterialsPlugin;

impl Plugin for MaterialsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<MaterialManager>();
        app.init_resource::<MaterialConfigs>();
        app.add_systems(PostStartup, load_materials);
    }
}

#[derive(AssetCollection, Resource, Default)]
pub struct MaterialConfigs {
    #[asset(path = "data", collection)]
    configs: Vec<UntypedHandle>,
}

fn create_compound_material_from_config(
    config: &KdlNode,
    material_manager: &mut ResMut<MaterialManager>,
) {
    let opt_children = config.children();
    if let Some(children) = opt_children {
        let name = children
            .get("name")
            .unwrap()
            .get(0)
            .unwrap()
            .value()
            .as_string()
            .unwrap();

        let composition = children.get("composition").unwrap().children().unwrap();
        let composing_materials: Vec<ComposingMaterial> =
            composition.nodes().iter().filter_map(|node| {
                let node_name = node.name().to_string();
            if node_name.as_str() != "basic_material" {
                todo!();
            }

            let material_name = node.get("name").unwrap().value().as_string().unwrap();
            let mean = node.get("mean").unwrap().value().as_f64().unwrap() as f32;
            let sd = node.get("sd").unwrap().value().as_f64().unwrap() as f32;

            let material_id = material_manager.get_material_id(material_name);
            match material_id {
                None => {
                    warn!("Tried to create a compound material {} but couldn't find component material {}", name, material_name);
                    None
                }
                Some(id) => {
                    Some(ComposingMaterial {
                        id,
                        mean,
                        sd
                    })
                }
            }
            })
            .collect();

        material_manager.register_material(MaterialProperties::Compound(
            CompoundMaterialProperties {
                name: String::from(name),
                composition: composing_materials,
            },
        ))
    }
}

fn create_basic_material_from_config(
    config: &KdlNode,
    material_manager: &mut ResMut<MaterialManager>,
) {
    let opt_children = config.children();
    if let Some(children) = opt_children {
        let name = children
            .get("name")
            .unwrap()
            .get(0)
            .unwrap()
            .value()
            .as_string()
            .unwrap();

        let density = children
            .get("density")
            .unwrap()
            .get(0)
            .unwrap()
            .value()
            .as_i64()
            .unwrap();

        let thermal_properties = children
            .get("thermal_properties")
            .unwrap()
            .children()
            .unwrap();

        let heat_capacity = thermal_properties
            .get("heat_capacity")
            .unwrap()
            .get(0)
            .unwrap()
            .value()
            .as_i64()
            .unwrap();

        let melting_point = thermal_properties
            .get("melting_point")
            .unwrap()
            .get(0)
            .unwrap()
            .value()
            .as_i64()
            .unwrap();

        let boiling_point = thermal_properties
            .get("boiling_point")
            .unwrap()
            .get(0)
            .unwrap()
            .value()
            .as_i64()
            .unwrap();

        material_manager.register_material(MaterialProperties::Basic(BasicMaterialProperties {
            name: String::from(name),
            density: Density {
                mass: Mass::from_kilograms(density),
                volume: Volume::from_cubic_metres(1),
            },
            thermal_properties: ThermalProperties {
                heat_capacity: HeatCapacity {
                    energy: Energy::from_joules(heat_capacity),
                    mass: Mass::from_kilograms(1),
                },
                melting_point: Temperature::from_kelvin(melting_point),
                boiling_point: Temperature::from_kelvin(boiling_point),
            },
        }))
    }
}

fn load_materials(
    loaded_files: Res<MaterialConfigs>,
    config_assets: Res<Assets<KdlAsset>>,
    mut material_manager: ResMut<MaterialManager>,
) {
    for handle in &loaded_files.configs {
        let opt_asset = config_assets.get(handle);
        if let Some(asset) = opt_asset {
            let document = &asset.0;

            for node in document.nodes() {
                let node_name = node.name().to_string();
                match node_name.as_str() {
                    "define_basic_material" => {
                        create_basic_material_from_config(node, &mut material_manager);
                    }
                    "define_compound_material" => {
                        create_compound_material_from_config(node, &mut material_manager);
                    }
                    _ => {
                        todo!("Command {} is not implemented yet", node_name)
                    }
                }
            }
        }
    }
}

fn test_materials(material_manager: Res<MaterialManager>) {
    let ilmenite = material_manager.get_material_id("Ilmenite").unwrap();
    let instance = material_manager.generate_material_instance(ilmenite);
}
