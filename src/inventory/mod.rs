use crate::{
    crafting::{potions::PotionEffect, Process},
    prelude::*,
};
use bevy::{ecs::world::EntityMut, prelude::*, utils::HashMap};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use crate::plants::{Plant, PlantPart};

const INVENTORY_SIZE: usize = 98;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Inventory>()
            .add_event::<InventoryEvent>()
            .register_type::<Slot>()
            .register_type::<Item>()
            .add_systems(
                (
                    highlight_selected_item,
                    select_item,
                    item_events,
                    // update_icon,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(save_inventory.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Debug, Resource)]
pub struct HotBar(pub [Entity; 10]);

#[derive(Debug, Serialize, Deserialize)]
struct HotBarData([Item; 10]);

impl FromWorld for HotBar {
    fn from_world(world: &mut World) -> Self {
        let pkv = world.resource::<PkvStore>();
        let items = if let Ok(data) = pkv.get::<HotBarData>("hotbar") {
            data.0
        } else {
            [Item::Empty; 10]
        };
        let mut entitys = [Entity::from_bits(0); 10];
        let mut hotbar_slot = Slot::iter_all();
        for i in 0..10 {
            entitys[i] = world
                .spawn((
                    items[i],
                    ButtonBundle {
                        style: Style {
                            size: Size::all(Val::Percent(100.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    hotbar_slot.next().unwrap(),
                ))
                .id();
        }
        HotBar(entitys)
    }
}

#[derive(Resource)]
pub struct ToolBarItem(u8);

#[derive(Serialize, Deserialize)]
struct InventoryData(Vec<Item>);

impl FromWorld for Inventory {
    fn from_world(world: &mut World) -> Self {
        let pkv = world.resource::<PkvStore>();
        match pkv.get::<Inventory>("inventory") {
            Ok(inv) => inv,
            Err(e) => {
                world.send_event(PlayerMessage::error(format!("Failed to load inventory {:?}", e)));
                Inventory(default())
            },
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct SelectedSlot;

fn highlight_selected_item(
    mut selected: Query<&mut BackgroundColor, Added<SelectedSlot>>,
    mut colors: Query<&mut BackgroundColor, Without<SelectedSlot>>,
    mut removed: RemovedComponents<SelectedSlot>,
) {
    for mut selected in &mut selected {
        selected.0 = Color::GRAY;
    }
    for removed in removed.iter() {
        if let Ok(mut color) = colors.get_mut(removed) {
            color.0 = Color::WHITE;
        }
    }
}

fn select_item(
    buttons: Query<(Entity, &Interaction, &Slot), (Changed<Interaction>, With<Slot>)>,
    selected: Query<(Entity, &Slot), With<SelectedSlot>>,
    mut commands: Commands,
    mut events: EventWriter<InventoryEvent>,
) {
    for (entity, interaction, slot) in &buttons {
        if let Interaction::Clicked = interaction {
            if let Ok((e, selected)) = selected.get_single() {
                if e != entity {
                    events.send(InventoryEvent::MoveItem(*selected, *slot));
                }
            } else {
                commands.entity(entity).insert(SelectedSlot);
            }
            for (entity, _) in &selected {
                commands.entity(entity).remove::<SelectedSlot>();
            }
        }
    }
}

#[derive(
    Component,
    Reflect,
    Debug,
    Serialize,
    Deserialize,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    FromReflect,
)]
pub enum Item {
    #[default]
    Empty,
    Potion(u8),
    Ingredient(Plant, PlantPart),
    Intimidate(u8, u8),
}

impl UiItem for Item {
    fn icon_path(&self) -> String {
        match self {
            Item::Empty => "icons/empty.png",
            Item::Potion(0) => "icons/potion_5.png",
            Item::Potion(255) => "icons/potion_d.png",
            Item::Potion(id) => match id % 8 {
                0 => "icons/potion_0.png",
                1 => "icons/potion_1.png",
                2 => "icons/potion_2.png",
                3 => "icons/potion_3.png",
                4 => "icons/potion_4.png",
                5 => "icons/potion_5.png",
                6 => "icons/potion_6.png",
                7 => "icons/potion_7.png",
                _ => unreachable!(),
            },
            Item::Ingredient(plant, part) => match plant {
                Plant::Palm => match part {
                    PlantPart::Leaf => "icons/palm/leaf.png",
                    PlantPart::Seed => "icons/palm/seed.png",
                    PlantPart::Fruit => "icons/palm/fruit.png",
                    PlantPart::Bark => "icons/palm/bark.png",
                    PlantPart::Stem => "icons/palm/wood.png",
                    PlantPart::Root => "icons/palm/root.png",
                    _ => {
                        error!("Item {:?}, dose not have an icon yet", self);
                        "icons/null.png"
                    }
                },
            },
            Item::Intimidate(_, effect) => {
                if effect & 0xf > (effect >> 4) & 0xf {
                    "icons/ash.png"
                } else {
                    "icons/cube.png"
                }
            }
        }.into()
    }

    fn background_color(&self) -> Color {
        match self {
            Item::Potion(0) => Color::BLUE,
            Item::Potion(255) => Color::WHITE,
            Item::Potion(id) | Item::Intimidate(id, _) => Color::rgb_u8(
                (id << 4) & 0b11110000,
                (id << 2) & 0b11110000,
                id & 0b11110000 as u8,
            ),
            _ => Color::WHITE,
        }
    }
}

impl Item {
    fn get_icon(&self, asset: &ItemIcons) -> Handle<Image> {
        match self {
            Item::Ingredient(plant, part) => match plant {
                Plant::Palm => match part {
                    PlantPart::Leaf => asset.palm_leaf.clone(),
                    PlantPart::Seed => asset.palm_seed.clone(),
                    PlantPart::Fruit => asset.palm_fruit.clone(),
                    PlantPart::Bark => asset.palm_bark.clone(),
                    PlantPart::Stem => asset.palm_wood.clone(),
                    PlantPart::Root => asset.palm_root.clone(),
                    _ => {
                        error!("Item {:?}, dose not have an icon yet", self);
                        asset.null.clone()
                    }
                },
            },
            Item::Empty => asset.empty.clone(),
            Item::Potion(0) => asset.potion_5.clone(),
            Item::Potion(255) => asset.potion_d.clone(),
            Item::Potion(id) => match id % 8 {
                0 => asset.potion_0.clone(),
                1 => asset.potion_1.clone(),
                2 => asset.potion_2.clone(),
                3 => asset.potion_3.clone(),
                4 => asset.potion_4.clone(),
                5 => asset.potion_5.clone(),
                6 => asset.potion_6.clone(),
                7 => asset.potion_7.clone(),
                _ => unreachable!(),
            },
            Item::Intimidate(_, effect) => {
                if effect & 0xf > (effect >> 4) & 0xf {
                    asset.ash.clone()
                } else {
                    asset.cube.clone()
                }
            }
        }
    }
    pub fn brew(&mut self, other: Item) -> Result<u8, crate::crafting::CraftingError> {
        use crate::crafting::CraftingError;
        match self {
            Item::Empty => return Err(CraftingError::NoItem),
            Item::Potion(_) => {}
            Item::Ingredient(_, part) => *self = Item::Potion(*part as u8),
            Item::Intimidate(part, _) => *self = Item::Potion(*part),
        }
        let Item::Potion(val) = self else {return Err(CraftingError::Bug);};
        let other = match other {
            Item::Empty => return Err(CraftingError::NoItem),
            Item::Potion(val) => val,
            Item::Ingredient(_, part) => part as u8,
            Item::Intimidate(part, _) => part,
        };
        *val |= other;
        Ok(*val)
    }

    pub fn taste(&self) -> String {
        match self {
            Item::Empty => String::from("You lick an empty part of the ui. you are suddently aware you are in a video game;\n you decide to report this bug you just found to you god Phox;"),
            Item::Potion(val) => {
                use crate::crafting::potions::PotionEffect::*;
                if *val == 0 {
                    return String::from("It's Just Water");
                }
                if *val == 255 {
                    return format!("That Potion has the effect {}; good thing you are an immortal wizard", InstantDeath);
                }
                let effects = PotionEffect::get_potion_effects(*val);
                if effects.len() == 0 {
                    return String::from("Is is no Potion is has no effects its just a nice pot of soup");
                }
                let mut res = String::from("The Potion Has the following effects:");
                for effect in effects {
                    res.push_str(&format!("\n{},", effect));
                }
                res.pop();
                res
            },
            Item::Ingredient(Plant::Palm, PlantPart::Fruit) => {
                String::from("You Bite into the coconut it is tasty")
            },
            Item::Ingredient(Plant::Palm, PlantPart::Seed) => {
                String::from("You Bite into the coconut it is not yet ripe")
            },
            Item::Ingredient(Plant::Palm, _) => {
                String::from("You Bite into a pice of palm tree and ask you self what are you doing with your life")
            },
            #[allow(unreachable_patterns)]
            Item::Ingredient(_, _) => String::from("you tentatively lick the mystiriuse plant, as soon as you toung toches it you are suddently aware you are in a video game;\n you decide to report this bug you just found to you god Phox;"),
            Item::Intimidate(val, _) => format!("you lick the item; you get the feeling if you add this to a potion it would have the effect\n\n{:08b}\n\n by looking at it you can tell it can still have the following processes applied \n [{:?}]", val, self.can_do_process()),
        }
    }

    pub fn tool_tip_text(&self) -> String {
        match self {
            Item::Empty => String::from("An Empty Inventory Slot"),
            Item::Potion(0) => String::from("A bottle of water"),
            Item::Potion(id) => {
                let effects = PotionEffect::get_potion_effects(*id);
                let mut rep = String::from("Its a Potion of ");
                match effects.len() {
                    0 => rep.push_str("Soup"),
                    _ => {
                        for effect in effects {
                            rep.push_str(&format!("{},", effect));
                        }
                        rep.pop();
                    }
                }
                rep
            }
            Item::Ingredient(plant, part) => plant.tool_tip_text(*part),
            Item::Intimidate(id, _) => {
                format!(
                    "Someting you cooked up in the Lab: {:08b}\n you can still:\n {:?}",
                    id,
                    self.can_do_process()
                )
            }
        }
    }

    pub fn can_do_process(&self) -> Vec<Process> {
        match self {
            Item::Empty => vec![],
            Item::Potion(_) => vec![Process::Taste],
            Item::Ingredient(_, _) => <Process as strum::IntoEnumIterator>::iter().collect(),
            Item::Intimidate(_, mech) => Process::can_do(*mech),
        }
    }
}

pub enum InventoryEvent {
    AddItem(Item),
    RemoveItem(Slot),
    MoveItem(Slot, Slot),
}

#[derive(
    Debug, Reflect, Serialize, Deserialize, Hash, Default, Component, PartialEq, Eq, Clone, Copy,
)]
pub enum Slot {
    #[default]
    HotBar0,
    HotBar1,
    HotBar2,
    HotBar3,
    HotBar4,
    HotBar5,
    HotBar6,
    HotBar7,
    HotBar8,
    HotBar9,
    Inventory(usize),
    Shop,
}

impl Slot {
    pub fn iter_all() -> SlotIter {
        SlotIter{next: Some(Slot::HotBar0), last: Slot::Inventory(INVENTORY_SIZE)}
    }
    pub fn iter_hotbar() -> SlotIter {
        SlotIter{next: Some(Slot::HotBar0), last: Slot::HotBar9}
    }
    pub fn iter_inventory() -> SlotIter {
        SlotIter{next: Some(Slot::Inventory(0)), last: Slot::Inventory(INVENTORY_SIZE)}
    }
}

pub struct SlotIter{next: Option<Slot>, last: Slot}

impl Iterator for SlotIter {
    type Item = Slot;
    fn next(&mut self) -> Option<Self::Item> {
        let next = match self.next {
            Some(Slot::HotBar0) => {
                self.next = Some(Slot::HotBar1);
                Slot::HotBar0
            }
            Some(Slot::HotBar1) => {
                self.next = Some(Slot::HotBar2);
                Slot::HotBar1
            }
            Some(Slot::HotBar2) => {
                self.next = Some(Slot::HotBar3);
                Slot::HotBar2
            }
            Some(Slot::HotBar3) => {
                self.next = Some(Slot::HotBar4);
                Slot::HotBar3
            }
            Some(Slot::HotBar4) => {
                self.next = Some(Slot::HotBar5);
                Slot::HotBar4
            }
            Some(Slot::HotBar5) => {
                self.next = Some(Slot::HotBar6);
                Slot::HotBar5
            }
            Some(Slot::HotBar6) => {
                self.next = Some(Slot::HotBar7);
                Slot::HotBar6
            }
            Some(Slot::HotBar7) => {
                self.next = Some(Slot::HotBar8);
                Slot::HotBar7
            }
            Some(Slot::HotBar8) => {
                self.next = Some(Slot::HotBar9);
                Slot::HotBar8
            }
            Some(Slot::HotBar9) => {
                self.next = Some(Slot::Inventory(0));
                Slot::HotBar9
            }
            Some(Slot::Inventory(i)) => {
                self.next = if i == INVENTORY_SIZE - 1 {
                    Some(Slot::Shop)
                } else {
                    Some(Slot::Inventory(i + 1))
                };
                Slot::Inventory(i)
            }
            Some(Slot::Shop) => {
                self.next = None;
                Slot::Shop
            }
            None => {return None;},
        };
        if next == self.last {
            self.next = None
        }
        Some(next)
    }
}

// pub fn spawn_inventory_tab(
//     mut entity: EntityMut,
//     nine_patch: &Handle<bevy_ninepatch::NinePatchBuilder>,
//     assets: &UiAssets,
//     inventory: &Inventory,
// ) -> Entity {
//     entity
//         .insert(NodeBundle {
//             style: crate::tabs::MAIN_WINDOW_STYLE.clone(),
//             visibility: Visibility::Hidden,
//             ..Default::default()
//         })
//         .with_children(|p| {
//             for slot in Slot::iter_all().skip(10) {
//                 p.spawn(bevy_ninepatch::NinePatchBundle {
//                     style: Style {
//                         margin: UiRect::all(Val::Auto),
//                         justify_content: JustifyContent::Center,
//                         align_items: AlignItems::Center,
//                         size: Size::new(Val::Auto, Val::Auto),
//                         min_size: Size::new(Val::Auto, Val::Px(80.)),
//                         aspect_ratio: Some(1.),
//                         ..Default::default()
//                     },
//                     nine_patch_data: bevy_ninepatch::NinePatchData::with_single_content(
//                         assets.outline.clone(),
//                         nine_patch.clone(),
//                         inventory.0[slot.inventor_slot().unwrap()],
//                     ),
//                     ..Default::default()
//                 });
//             }
//         })
//         .id()
// }

fn item_events(
    selected: Query<(Entity, &Slot), With<SelectedSlot>>,
    mut commands: Commands,
    mut events: EventReader<InventoryEvent>,
    mut inventory: ResMut<Inventory>,
) {
    for event in events.iter() {
        match event {
            InventoryEvent::AddItem(item) => {
                for slot in Slot::iter_all() {
                    if !inventory.0.contains_key(&slot) {
                        inventory.0.insert(slot, *item);
                        break;
                    }
                }
            }
            InventoryEvent::RemoveItem(slot) => {
                if let Ok((item, current_slot)) = selected.get_single() {
                    if slot == current_slot {
                        commands.entity(item).remove::<SelectedSlot>();
                    }
                }
                inventory.0.remove(slot);
            }
            InventoryEvent::MoveItem(from, to) => {
                let old = inventory.0.remove(from);
                let new = inventory.0.remove(to);
                if let Some(old) = old {
                    inventory.0.insert(*to, old);
                }
                if let Some(old) = new {
                    inventory.0.insert(*from, old);
                }
            }
        }
    }
}

struct SaveTime(Timer);
impl Default for SaveTime {
    fn default() -> Self {
        SaveTime(Timer::from_seconds(60., TimerMode::Repeating))
    }
}

fn save_inventory(
    time: Res<Time>,
    mut store: ResMut<PkvStore>,
    mut next_save: Local<SaveTime>,
    inventory: Res<Inventory>,
) {
    next_save.0.tick(time.delta());
    if next_save.0.finished() {
        let _ = store.set("inventory", inventory.as_ref());
    }
}

#[derive(Debug, Resource)]
pub struct Inventory(pub HashMap<Slot, Item>);

impl Inventory {
    pub fn get(&self, slot: &Slot) -> Option<Item> {
        self.0.get(slot).cloned()
    }

    pub fn get_item_icon(&self, slot: Slot) -> String {
        if let Some(item) = self.get(&slot) {
            item.icon_path()
        } else {
            "icons/empty.png".into()
        }
    }
}

impl Serialize for Inventory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Inventory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        Ok(Inventory(HashMap::<Slot, Item>::deserialize(deserializer)?))
    }
}