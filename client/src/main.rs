#!allow[cargo::dead_code]

extern crate bevy;
extern crate rand;

mod units;

use bevy::{input::keyboard, prelude::*, utils::HashMap};
use rand::{thread_rng, Rng};
use units::{Density, Mass, Volume};

use crate::units::UnitT;

type ItemID = u32;

const MAX_INVENTORY_VOLUME: Volume = Volume::from_litres(45);
const MAX_INVENTORY_MASS: Mass = Mass::from_kilograms(40);

struct MaterialProperties {
    density: Density,
}

impl MaterialProperties {
    fn new() -> Self {
        MaterialProperties {
            density: Density::new(Mass::from_kilograms(1), Volume::from_cubic_metres(1)),
        }
    }
}

struct Item {
    name: String,
    material: MaterialProperties,
}

impl Item {
    fn new(name: String, material: MaterialProperties) -> Self {
        Item { name, material }
    }
}

#[derive(Resource)]
struct ItemManager {
    items: HashMap<u32, Item>,
}

impl ItemManager {
    fn new() -> Self {
        ItemManager {
            items: HashMap::new(),
        }
    }

    fn register_item(&mut self, item: Item) -> ItemID {
        let id: ItemID = {
            let mut id: ItemID = 0;
            loop {
                if !self.items.contains_key(&id) {
                    break;
                }
                id += 1;
            }
            id
        };

        self.items.insert(id, item);
        id
    }

    fn get_item(&self, item: ItemID) -> Option<&Item> {
        self.items.get(&item)
    }

    fn get_random_item(&self) -> ItemID {
        let keys: Vec<&ItemID> = self.items.keys().collect();
        **(keys.get(thread_rng().gen_range(0..keys.len())).unwrap())
    }
}

impl Default for ItemManager {
    fn default() -> Self {
        let mut item_manager = ItemManager {
            items: HashMap::new(),
        };
        item_manager.register_item(Item::new(
            String::from("Magnetite"),
            MaterialProperties {
                density: Density::new(
                    Mass::from_milligrams(5260),
                    Volume::from_cubic_centimetres(1),
                ),
            },
        ));
        item_manager.register_item(Item::new(
            String::from("Hematite"),
            MaterialProperties {
                density: Density::new(
                    Mass::from_milligrams(5180),
                    Volume::from_cubic_centimetres(1),
                ),
            },
        ));
        item_manager
    }
}

#[derive(Component)]
struct ResourceInventory {
    items: HashMap<ItemID, Volume>,
    mass: Mass,
    volume: Volume,
}

impl ResourceInventory {
    fn new() -> Self {
        ResourceInventory {
            items: Default::default(),
            mass: Mass::from_grams(0),
            volume: Volume::from_litres(0),
        }
    }

    pub fn is_full(&self) -> bool {
        (self.mass >= MAX_INVENTORY_MASS - Mass::from_grams(10))
            || (self.volume >= MAX_INVENTORY_VOLUME - Volume::from_cubic_centimetres(10))
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.volume = Volume::from_litres(0);
        self.mass = Mass::from_kilograms(0);
    }

    pub fn add_items(&mut self, item_id: ItemID, item: &Item, volume: Volume) -> Volume {
        let remaining_volume = MAX_INVENTORY_VOLUME - self.volume;
        let remaining_mass = MAX_INVENTORY_MASS - self.mass;

        let mass = item.material.density * volume;
        let volume_to_add_based_on_mass = {
            if mass <= remaining_mass {
                mass / item.material.density
            } else {
                remaining_mass / item.material.density
            }
        };
        let volume_to_add_based_on_volume = {
            if volume <= remaining_volume {
                volume
            } else {
                remaining_volume
            }
        };

        if volume_to_add_based_on_mass.as_cubic_millimetres() <= 0
            || volume_to_add_based_on_volume.as_cubic_millimetres() <= 0
        {
            return volume;
        }

        let volume_to_add = {
            if volume_to_add_based_on_mass < volume_to_add_based_on_volume {
                volume_to_add_based_on_mass
            } else {
                volume_to_add_based_on_volume
            }
        };
        let mass_to_add = item.material.density * volume_to_add;

        self.items.insert(
            item_id,
            *self.items.get(&item_id).unwrap_or(&Volume::from_litres(0)) + volume,
        );
        self.volume += volume_to_add;
        self.mass += mass_to_add;

        volume - volume_to_add
    }

    pub fn remove_items(&mut self, item_id: ItemID, item: &Item, volume: Volume) -> Option<Volume> {
        let current_volume = {
            let has_current_amount = self.items.get(&item_id);
            has_current_amount?;
            *has_current_amount.unwrap()
        };

        if volume > current_volume {
            self.items.remove(&item_id);
            self.volume -= current_volume;
            self.mass -= item.material.density * current_volume;
            Some(current_volume)
        } else {
            self.items.insert(item_id, current_volume - volume);
            self.volume -= volume;
            self.mass -= item.material.density * volume;
            Some(volume)
        }
    }
}

#[derive(Component, Resource)]
struct ItemTracker(HashMap<ItemID, Volume>);

impl ItemTracker {
    pub fn new() -> Self {
        ItemTracker(HashMap::new())
    }

    pub fn add_resource(&mut self, item: ItemID, amount: Volume) -> Volume {
        let current_amount = *self.0.get(&item).unwrap_or(&Volume::from_litres(0));
        self.0.insert(item, current_amount + amount);
        current_amount + amount
    }
}

impl Default for ItemTracker {
    fn default() -> Self {
        ItemTracker::new()
    }
}

#[derive(Component)]
struct ResourceNode {
    item: ItemID,
    remaining: Volume,
}

impl ResourceNode {
    fn new(item: ItemID, count: Volume) -> Self {
        ResourceNode {
            item,
            remaining: count,
        }
    }
}

#[derive(Default, Resource)]
struct ResourceNodeRegistry {
    nodes: Vec<(Entity, Vec3)>,
}

impl ResourceNodeRegistry {
    fn get_random_node(&self) -> &(Entity, Vec3) {
        let mut random_source = thread_rng();
        self.nodes
            .get(random_source.gen_range(0..self.nodes.len()))
            .unwrap()
    }

    fn register_node(&mut self, node: Entity, position: Vec3) {
        self.nodes.push((node, position))
    }
}

#[derive(Debug, PartialEq, Eq)]
enum AiState {
    Gather,
    Mining,
    Return,
    Rest,
}

#[derive(Component)]
struct Brain {
    pub id: u32,
    pub state: AiState,
    pub state_change_cooldown: f32,
    pub target_entity: Option<Entity>,
    pub target_position: Option<Vec3>,
}

#[derive(Component)]
struct Tool {
    pub cooldown: f32,
}

fn add_players(mut commands: Commands) {
    let mut random_source = thread_rng();

    for i in 0..6 {
        commands.spawn((
            ItemTracker::new(),
            ResourceInventory::new(),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::hsl(i as f32 * 60.0, 0.5, 0.5),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    random_source.gen_range(-100.0..100.0),
                    random_source.gen_range(-100.0..100.0),
                    0.0,
                ),
                ..default()
            },
            Brain {
                id: i,
                state: AiState::Rest,
                state_change_cooldown: 0.0,
                target_entity: None,
                target_position: None,
            },
            Tool { cooldown: 0.0 },
        ));
    }
}

fn create_resource_nodes(
    mut commands: Commands,
    item_manager: Res<ItemManager>,
    mut node_registry: ResMut<ResourceNodeRegistry>,
    mut global_resource_tracker: ResMut<ItemTracker>,
) {
    let mut random_source = thread_rng();

    for _ in 0..10 {
        let item_id = item_manager.get_random_item();
        let count = random_source.gen_range(20..100);
        let position = Vec3::new(
            random_source.gen_range(-500.0..500.0),
            random_source.gen_range(-500.0..500.0),
            5.0,
        );

        global_resource_tracker.add_resource(item_id, Volume::from_cubic_metres(count));
        let entity = commands
            .spawn((
                ResourceNode::new(item_id, Volume::from_cubic_metres(count)),
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::hsl(item_id as f32 * 60.0, count as f32 / 100.0, 0.7),
                        custom_size: Some(Vec2::new(70.0, 70.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(position),
                    ..default()
                },
            ))
            .id();

        node_registry.register_node(entity, position);
    }
}

fn run_ai_logic(
    time: Res<Time<Fixed>>,
    mut ai: Query<(
        &mut Brain,
        &mut ResourceInventory,
        &mut ItemTracker,
        &Transform,
    )>,
    node_registry: Res<ResourceNodeRegistry>,
) {
    for (mut brain, mut inventory, mut tracker, transform) in &mut ai {
        brain.state_change_cooldown -= time.delta_seconds();
        if brain.state_change_cooldown > 0.0 {
            continue;
        }

        let new_state: AiState = match brain.state {
            AiState::Rest => {
                let target_node = node_registry.get_random_node();
                brain.target_position = Some(target_node.1 + Vec3::new(0.0, 0.0, 5.0));
                brain.target_entity = Some(target_node.0);
                for (item_id, volume) in &inventory.items {
                    tracker.add_resource(*item_id, *volume);
                }
                inventory.clear();
                AiState::Gather
            }
            AiState::Mining => {
                if inventory.is_full() {
                    brain.target_entity = None;
                    brain.target_position = Some(Vec3::new(0.0, 0.0, 3.0));
                    AiState::Return
                } else {
                    AiState::Mining
                }
            }
            AiState::Gather => {
                if (brain.target_position.unwrap() - transform.translation).length_squared() < 1.0 {
                    AiState::Mining
                } else {
                    AiState::Gather
                }
            }
            AiState::Return => {
                if (brain.target_position.unwrap() - transform.translation).length_squared() < 1.0 {
                    AiState::Rest
                } else {
                    AiState::Return
                }
            }
        };

        brain.state = new_state;
        brain.state_change_cooldown = 0.5
    }
}

fn do_movement(time: Res<Time<Fixed>>, mut ai: Query<(&mut Transform, &Brain)>) {
    for (mut transform, brain) in &mut ai {
        if brain.state == AiState::Rest || brain.state == AiState::Mining {
            continue;
        }
        match brain.target_position {
            None => continue,
            Some(position) => {
                let direction_vector = position - transform.translation;
                let direction_unit_vector = direction_vector / direction_vector.length();
                let adjusted_vector = {
                    if direction_vector.length_squared() < direction_unit_vector.length_squared() {
                        direction_vector
                    } else {
                        direction_unit_vector
                    }
                };

                transform.translation += adjusted_vector * time.delta_seconds() * 100.0;
            }
        }
    }
}

fn use_tools(
    time: Res<Time<Fixed>>,
    mut ai: Query<(&mut Brain, &mut ResourceInventory, &mut Tool)>,
    mut nodes: Query<&mut ResourceNode>,
    item_manager: Res<ItemManager>,
) {
    for (brain, mut inventory, mut tool) in &mut ai {
        tool.cooldown -= time.delta_seconds();

        if brain.state != AiState::Mining {
            continue;
        }
        if tool.cooldown > 0.0 {
            continue;
        }
        if inventory.is_full() {
            continue;
        }

        let mut node: Mut<'_, ResourceNode> = nodes
            .get_component_mut(brain.target_entity.unwrap())
            .unwrap();

        let amount_to_remove = {
            let target_amount = Volume::from_millilitres(time.delta().as_millis() as UnitT);
            if node.remaining > target_amount {
                target_amount
            } else {
                node.remaining
            }
        };

        let item = item_manager.get_item(node.item).unwrap();
        let amount_not_removed = inventory.add_items(node.item, item, amount_to_remove);
        node.remaining -= amount_to_remove - amount_not_removed;
    }
}

fn controls(keys: Res<Input<KeyCode>>, ai: Query<(&ResourceInventory, &Brain, &ItemTracker)>) {
    if keys.just_released(KeyCode::P) {
        for (inv, brain, tracker) in &ai {
            info!("AI {} Inventory:", brain.id);
            for (item_id, volume) in &inv.items {
                info!("\tID: {}\tVolume: {}ml", item_id, volume.as_millilitres());
            }
            info!("AI {} Tracker:", brain.id);
            for (item_id, quantity) in &tracker.0 {
                info!("\tID: {}\tVolume: {}ml", item_id, quantity.as_millilitres());
            }
            info!("\n");
        }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .init_resource::<ItemManager>()
        .init_resource::<ItemTracker>()
        .init_resource::<ResourceNodeRegistry>()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Startup, add_players)
        .add_systems(Startup, create_resource_nodes)
        .add_systems(FixedUpdate, run_ai_logic)
        .add_systems(FixedUpdate, do_movement)
        .add_systems(FixedUpdate, use_tools)
        .add_systems(Update, controls)
        .run();
}
