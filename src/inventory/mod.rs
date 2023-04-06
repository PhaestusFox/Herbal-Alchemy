use std::collections::HashMap;

use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Serialize, Deserialize};

use crate::plants::{Plant, PlantPart};

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HotBar>();
    }
}

#[derive(Debug, Resource)]
pub struct HotBar(pub [Entity; 10]);

#[derive(Debug, Serialize, Deserialize)]
struct  HotBarData([Item; 10]);

impl FromWorld for HotBar {
    fn from_world(world: &mut World) -> Self {
        let pkv = world.resource::<PkvStore>();
        let items = if let Ok(data) = pkv.get::<HotBarData>("hotbar") {
            data.0
        } else {
            [Item::Empty; 10]
        };
        let mut entitys = [Entity::from_bits(0); 10];
        for i in 0..10 {
            entitys[i] = world.spawn((items[i], ImageBundle::default())).id();
        }
        HotBar(entitys)
    }
}

#[derive(Resource)]
pub struct ToolBarItem(u8);
#[derive(Resource)]
pub struct Inventory(HashMap<IVec2, Entity>);

#[derive(Component, Debug, Serialize, Deserialize, Default, Clone, Copy)]
enum Item {
    #[default]
    Empty,
    Seed(Plant),
    Potion(u64),
    Ingredient(Plant, PlantPart),
    Intimidate(u64),
}