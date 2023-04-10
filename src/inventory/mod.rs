use crate::{
    crafting::{potions::PotionEffect, Process},
    prelude::*,
    toolbar::BarEntitys,
};
use bevy::{ecs::world::EntityMut, prelude::*};
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use crate::plants::{Plant, PlantPart};

const INVENTORY_SIZE: usize = 98;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HotBar>()
            .init_resource::<Inventory>()
            .add_event::<InventoryEvent>()
            .register_type::<Slot>()
            .register_type::<Item>()
            .add_systems(
                (
                    highlight_selected_item,
                    select_item,
                    item_events,
                    update_icon,
                )
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(show_inventory.in_schedule(OnEnter(Tab::Inventory)))
            .add_system(hide_inventory.in_schedule(OnExit(Tab::Inventory)))
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
        let mut hotbar_slot = Slot::iter();
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

#[derive(Resource)]
pub struct Inventory([Entity; INVENTORY_SIZE]);
#[derive(Serialize, Deserialize)]
struct InventoryData(Vec<Item>);

impl FromWorld for Inventory {
    fn from_world(world: &mut World) -> Self {
        let pkv = world.resource::<PkvStore>();
        let items = if let Ok(data) = pkv.get::<InventoryData>("inventory") {
            data.0
        } else {
            vec![Item::Empty; INVENTORY_SIZE]
        };
        let mut entitys = [Entity::from_bits(0); INVENTORY_SIZE];
        for i in 0..INVENTORY_SIZE {
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
                    Slot::Inventory(i),
                ))
                .id();
        }
        Inventory(entitys)
    }
}

#[derive(Debug, Default, Component)]
pub struct SelectedSlot;

fn highlight_selected_item(
    mut selected: Query<&mut BackgroundColor, Added<SelectedSlot>>,
    mut colors: Query<(&mut BackgroundColor, &Item), Without<SelectedSlot>>,
    mut removed: RemovedComponents<SelectedSlot>,
) {
    for mut selected in &mut selected {
        selected.0 = Color::GRAY;
    }
    for removed in removed.iter() {
        if let Ok((mut color, item)) = colors.get_mut(removed) {
            color.0 = item.get_bg_color();
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

impl Item {
    fn get_bg_color(&self) -> Color {
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
}

impl Slot {
    pub fn iter() -> SlotIter {
        SlotIter(Some(Slot::HotBar0))
    }
    fn hot_bar_slot(&self) -> Option<usize> {
        match self {
            Slot::HotBar0 => Some(0),
            Slot::HotBar1 => Some(1),
            Slot::HotBar2 => Some(2),
            Slot::HotBar3 => Some(3),
            Slot::HotBar4 => Some(4),
            Slot::HotBar5 => Some(5),
            Slot::HotBar6 => Some(6),
            Slot::HotBar7 => Some(7),
            Slot::HotBar8 => Some(8),
            Slot::HotBar9 => Some(9),
            Slot::Inventory(_) => None,
        }
    }
    fn inventor_slot(&self) -> Option<usize> {
        match self {
            Slot::Inventory(i) => Some(*i as usize),
            _ => None,
        }
    }
}

pub struct SlotIter(Option<Slot>);

impl Iterator for SlotIter {
    type Item = Slot;
    fn next(&mut self) -> Option<Self::Item> {
        match self.0 {
            Some(Slot::HotBar0) => {
                self.0 = Some(Slot::HotBar1);
                Some(Slot::HotBar0)
            }
            Some(Slot::HotBar1) => {
                self.0 = Some(Slot::HotBar2);
                Some(Slot::HotBar1)
            }
            Some(Slot::HotBar2) => {
                self.0 = Some(Slot::HotBar3);
                Some(Slot::HotBar2)
            }
            Some(Slot::HotBar3) => {
                self.0 = Some(Slot::HotBar4);
                Some(Slot::HotBar3)
            }
            Some(Slot::HotBar4) => {
                self.0 = Some(Slot::HotBar5);
                Some(Slot::HotBar4)
            }
            Some(Slot::HotBar5) => {
                self.0 = Some(Slot::HotBar6);
                Some(Slot::HotBar5)
            }
            Some(Slot::HotBar6) => {
                self.0 = Some(Slot::HotBar7);
                Some(Slot::HotBar6)
            }
            Some(Slot::HotBar7) => {
                self.0 = Some(Slot::HotBar8);
                Some(Slot::HotBar7)
            }
            Some(Slot::HotBar8) => {
                self.0 = Some(Slot::HotBar9);
                Some(Slot::HotBar8)
            }
            Some(Slot::HotBar9) => {
                self.0 = Some(Slot::Inventory(0));
                Some(Slot::HotBar9)
            }
            Some(Slot::Inventory(i)) => {
                self.0 = if i == INVENTORY_SIZE - 1 {
                    None
                } else {
                    Some(Slot::Inventory(i + 1))
                };
                Some(Slot::Inventory(i))
            }
            None => None,
        }
    }
}

pub fn spawn_inventory_tab(
    mut entity: EntityMut,
    nine_patch: &Handle<bevy_ninepatch::NinePatchBuilder>,
    assets: &UiAssets,
    inventory: &Inventory,
) -> Entity {
    entity
        .insert(NodeBundle {
            style: crate::tabs::MAIN_WINDOW_STYLE.clone(),
            visibility: Visibility::Hidden,
            ..Default::default()
        })
        .with_children(|p| {
            for slot in Slot::iter().skip(10) {
                p.spawn(bevy_ninepatch::NinePatchBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Auto, Val::Auto),
                        min_size: Size::new(Val::Auto, Val::Px(80.)),
                        aspect_ratio: Some(1.),
                        ..Default::default()
                    },
                    nine_patch_data: bevy_ninepatch::NinePatchData::with_single_content(
                        assets.outline.clone(),
                        nine_patch.clone(),
                        inventory.0[slot.inventor_slot().unwrap()],
                    ),
                    ..Default::default()
                });
            }
        })
        .id()
}

fn hide_inventory(tabs: Res<BarEntitys>, mut visibility: Query<&mut Visibility>) {
    if let Ok(mut entity) = visibility.get_mut(tabs.inventory_tab) {
        *entity = Visibility::Hidden;
    }
}

fn show_inventory(tabs: Res<BarEntitys>, mut visibility: Query<&mut Visibility>) {
    if let Ok(mut entity) = visibility.get_mut(tabs.inventory_tab) {
        *entity = Visibility::Visible;
    }
}

fn item_events(
    mut events: EventReader<InventoryEvent>,
    mut slots: Query<&mut Item>,
    hotbar: Res<HotBar>,
    inventory: Res<Inventory>,
) {
    'events: for event in events.iter() {
        match event {
            InventoryEvent::AddItem(item) => {
                for entity in hotbar.0 {
                    if let Ok(mut slot_item) = slots.get_mut(entity) {
                        if Item::Empty == *slot_item {
                            *slot_item = *item;
                            continue 'events;
                        }
                    }
                }
                for entity in inventory.0 {
                    if let Ok(mut slot_item) = slots.get_mut(entity) {
                        if Item::Empty == *slot_item {
                            *slot_item = *item;
                            continue 'events;
                        }
                    }
                }
            }
            InventoryEvent::RemoveItem(slot) => {
                let id = match (slot.hot_bar_slot(), slot.inventor_slot()) {
                    (Some(id), None) => hotbar.0[id],
                    (None, Some(id)) => inventory.0[id],
                    _ => unreachable!(),
                };
                if let Ok(mut item) = slots.get_mut(id) {
                    *item = Item::Empty;
                }
            }
            InventoryEvent::MoveItem(from, to) => {
                let old = match (from.hot_bar_slot(), from.inventor_slot()) {
                    (None, Some(i)) => inventory.0[i],
                    (Some(i), None) => hotbar.0[i],
                    _ => unreachable!(),
                };
                let new = match (to.hot_bar_slot(), to.inventor_slot()) {
                    (None, Some(i)) => inventory.0[i],
                    (Some(i), None) => hotbar.0[i],
                    _ => unreachable!(),
                };
                let Ok([mut old_item, mut new_item]) = slots.get_many_mut([old, new]) else {
                    error!("All Slots Should have Item and Size");
                    continue;
                };
                old_item.set_changed();
                new_item.set_changed();
                std::mem::swap(&mut *old_item, &mut *new_item);
            }
        }
    }
}

fn update_icon(
    mut items: Query<(&mut UiImage, &Item, &mut BackgroundColor), Changed<Item>>,
    item_icons: Res<ItemIcons>,
) {
    for (mut image, item, mut bg) in &mut items {
        *image = item.get_icon(&item_icons).into();
        bg.0 = item.get_bg_color();
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
    items: Query<(&Item, &Slot)>,
    mut next_save: Local<SaveTime>,
) {
    next_save.0.tick(time.delta());
    if next_save.0.finished() {
        let mut main = InventoryData(vec![Item::Empty; INVENTORY_SIZE]);
        let mut hot = HotBarData([Item::Empty; 10]);
        for (item, slot) in &items {
            match (slot.hot_bar_slot(), slot.inventor_slot()) {
                (Some(id), None) => {
                    hot.0[id] = *item;
                }
                (None, Some(id)) => {
                    main.0[id] = *item;
                }
                _ => unreachable!(),
            }
        }
        let _ = store.set("inventory", &main);
        let _ = store.set("hotbar", &hot);
    }
}
