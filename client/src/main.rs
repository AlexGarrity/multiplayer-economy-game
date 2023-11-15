extern crate bevy;
extern crate kdl;
extern crate rand;

mod ai;
mod loaders;
mod materials;
mod player;
mod units;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::LoadingState;
use bevy_asset_loader::loading_state::LoadingStateAppExt;
use kdl::KdlNode;
use loaders::KdlAsset;
use loaders::KdlLoader;
use materials::BasicMaterialProperties;
use materials::ComposingMaterial;
use materials::CompoundMaterialProperties;
use materials::MaterialInstance;
use materials::MaterialManager;
use materials::ThermalProperties;
use units::Density;
use units::Energy;
use units::HeatCapacity;
use units::Mass;
use units::Temperature;
use units::Volume;

use crate::materials::CompoundMaterialInstance;

#[derive(Component)]
struct Tool {
    pub cooldown: f32,
}

fn controls(
    time: Res<Time<Fixed>>,
    keys: Res<Input<KeyCode>>,
    mut cameras: Query<(&mut Transform, &mut OrthographicProjection)>,
) {
    let horizontal = {
        let mut x = 0.0;
        if keys.pressed(KeyCode::A) {
            x -= 1.0;
        }
        if keys.pressed(KeyCode::D) {
            x += 1.0;
        }
        x * 200.0 * time.delta_seconds()
    };

    let vertical = {
        let mut y = 0.0;
        if keys.pressed(KeyCode::S) {
            y -= 1.0;
        }
        if keys.pressed(KeyCode::W) {
            y += 1.0;
        }
        y * 200.0 * time.delta_seconds()
    };

    let zoom = {
        let mut zoom = 0.0;
        if keys.pressed(KeyCode::Z) {
            zoom += 1.0
        }
        if keys.pressed(KeyCode::X) {
            zoom -= 1.0
        }
        zoom * time.delta_seconds()
    };

    for (mut transform, mut projection) in &mut cameras {
        transform.translation += Vec3::new(horizontal, vertical, 0.0);
        projection.scale *= zoom + 1.0;
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
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

        material_manager.register_material(materials::MaterialProperties::Compound(
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

        material_manager.register_material(materials::MaterialProperties::Basic(
            BasicMaterialProperties {
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
            },
        ))
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

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
enum GameState {
    #[default]
    LoadConfigs,
    Loading,
    Gameplay,
}

#[derive(AssetCollection, Resource)]
struct MaterialConfigs {
    #[asset(path = "data", collection)]
    configs: Vec<UntypedHandle>,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_asset::<KdlAsset>()
        .init_resource::<MaterialManager>()
        .init_asset_loader::<KdlLoader>()
        .add_loading_state(
            LoadingState::new(GameState::LoadConfigs).continue_to_state(GameState::Loading),
        )
        .add_collection_to_loading_state::<_, MaterialConfigs>(GameState::LoadConfigs)
        .add_systems(OnExit(GameState::LoadConfigs), load_materials)
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Loading), test_materials);

    app.run();
}
