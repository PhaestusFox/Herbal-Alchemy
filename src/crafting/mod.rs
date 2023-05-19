use crate::{
    inventory::{Inventory, SelectedSlot},
    prelude::*,
};
use bevy::prelude::*;
use bevy_console::{reply, reply_failed, AddConsoleCommand, ConsoleCommand};
use clap::{Parser, ValueEnum};
use strum::IntoEnumIterator;
use tags::TagGroups;
use thiserror::Error;

use self::tags::TagNames;

pub mod potions;

pub mod tags;

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_console_command::<LabCommand, _>(process_command)
            .add_console_command::<PotionCommand, _>(brew_potion)
            .init_resource::<potions::TargetPotion>()
            .init_resource::<CurrentPotion>();
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "process")]
struct LabCommand {
    /// What Process to apply to the item
    #[arg(value_enum)]
    process: Process,
}

fn process_command(
    mut log: ConsoleCommand<LabCommand>,
    item: Query<(&Item, &Slot), With<SelectedSlot>>,
    mut events: EventWriter<InventoryEvent>,
    mut inventory: ResMut<Inventory>,
) {
    if let Some(Ok(LabCommand { process })) = log.take() {
        let Ok((item, slot)) = item.get_single() else {
            reply_failed!(log, "No Item Selected; Click an item to select it.");
            return;
        };
        if let Process::Test = process {
            let taste = item.taste();
            reply!(log, "{taste}");
            return;
        }
        match item.apply_process(process) {
            Ok(mut items) => {
                if let Some(item) = items.pop() {
                    events.send(InventoryEvent::InsertItem(item, *slot));
                }
                for item in items {
                    events.send(InventoryEvent::AddItem(item));
                }
            }
            Err(e) => reply_failed!(log, "{e}"),
        }
    }
}

#[derive(Parser, ConsoleCommand)]
#[command(name = "brew")]
struct PotionCommand {
    /// What action to do on the potion
    #[arg(value_enum)]
    action: PotionAcction,
}

#[derive(Debug, strum_macros::EnumString, Clone, Copy, ValueEnum)]
enum PotionAcction {
    /// Take Some And bottle it up
    Bottle,
    /// Add the current Ingrediant
    Add,
    /// Pore it out and start Again
    Empty,
    /// Give it a Sip
    Taste,
}

fn brew_potion(
    mut log: ConsoleCommand<PotionCommand>,
    item: Query<(&Item, &Slot), With<SelectedSlot>>,
    mut potion: ResMut<CurrentPotion>,
    mut events: EventWriter<InventoryEvent>,
) {
    let Some(Ok(PotionCommand { action })) = log.take() else {return;};
    match action {
        PotionAcction::Bottle => {
            events.send(InventoryEvent::AddItem(potion.0));
        }
        PotionAcction::Add => {
            let Ok((item, slot)) = item.get_single() else {reply_failed!(log, "No Item Selected; Click an item to select it."); return;};
            if let Err(e) = potion.0.brew(*item) {
                reply_failed!(log, "{e}");
            } else {
                events.send(InventoryEvent::RemoveItem(*slot));
            }
        }
        PotionAcction::Empty => potion.0 = Item::Potion(Tags::EMPTY),
        PotionAcction::Taste => {
            let taste = potion.0.taste();
            reply!(log, "{taste}");
        }
    }
}

#[derive(Debug, Error, Clone, Copy)]
pub enum CraftingError {
    #[error("No Item Selected")]
    NoItem,
    #[error("Potion Selected, you cant cook that ;P")]
    Potion,
    #[error("This has allready been done to this item")]
    DuplicateProcess(Process),
    #[error("This is a bug please report how you got this message")]
    Bug,
}

impl Item {
    // fn apply_process(&mut self, process: Process) -> Result<(), CraftingError> {
    //     if let Process::Taste = process {
    //         return Ok(());
    //     }
    //     match self {
    //         Item::Empty => return Err(CraftingError::NoItem),
    //         Item::Potion(_) => return Err(CraftingError::Potion),
    //         Item::Ingredient(_, part) => *self = Item::Intimidate(*part as u8, u8::from(process)),
    //         Item::Intimidate(_, done) => {
    //             if *done & u8::from(process) > 0 {
    //                 return Err(CraftingError::DuplicateProcess(u8::from(process)));
    //             } else {
    //                 *done |= u8::from(process);
    //             }
    //         }
    //     };
    //     let Item::Intimidate(val, _) = self else {return Err(CraftingError::Bug);};
    //     *val = match process {
    //         Process::SpinLeft => spin_left(*val),
    //         Process::SpinRight => spin_right(*val),
    //         Process::Freeze => freeze(*val),
    //         Process::Burn => burn(*val),
    //         Process::Chop => chop(*val),
    //         Process::Grind => grind(*val),
    //         Process::Shake => shake(*val),
    //         Process::Blend => blend(*val),
    //         Process::Mix => mix(*val),
    //         Process::Dice => dice(*val),
    //         Process::Taste => *val,
    //     };
    //     Ok(())
    // }

    fn apply_process(self, process: Process) -> Result<Vec<Item>, CraftingError> {
        if let Process::Test = process {
            return Ok(vec![self]);
        }
        let val = match self {
            Item::Empty => return Err(CraftingError::NoItem),
            Item::Potion(tags) => tags,
            Item::Ingredient(plant, part) => plant.get_tags() | part.get_tags(),
            Item::Intimidate(tags) => tags,
        };
        Ok(match process {
            Process::Test => unreachable!(),
            Process::Distill => vec![
                val.in_group(TagGroups::Volatile),
                val.not_in_group(TagGroups::Volatile),
            ],
            Process::Condense => vec![
                val.in_group(TagGroups::Heavy),
                val.not_in_group(TagGroups::Heavy),
            ],
            Process::Boil => vec![val.not_in_group(TagGroups::Cold).with_tag(TagNames::Water)],
            Process::Freeze => vec![val.not_in_group(TagGroups::Hot)],
            Process::Age => vec![val.in_group(TagGroups::Stable)],
            Process::Burn => vec![val
                .not_in_group(TagGroups::Volatile)
                .with_tag(TagNames::Fire)],
            Process::Spin => vec![val.not_in_group(TagGroups::Light)],
            Process::Steam => vec![val.not_in_group(TagGroups::Elemental)],
        }
        .into_iter()
        .map(|v| Item::Intimidate(v))
        .collect())
    }
}

#[test]
fn missing_vals() {
    use crate::inventory::Ingredient;
    use crate::plants::PlantPart;
    use indexmap::IndexSet;
    let mut all = (0..=255).collect::<IndexSet<u8>>();
    let mut found = IndexSet::new();
    const PARTS: [Item; 7] = [
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Leaf),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Seed),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Root),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Stem),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Bark),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Fruit),
        Item::Potion(Tags(255)),
    ];
    // let mut test = Item::Potion(Tags(255));
    // println!("test has tags {:?}", test.get_tags().get_tag_names());
    // test.apply_process(Process::Distill).unwrap();
    // println!("test has tags {:?}", test.get_tags().get_tag_names());
    // test.apply_process(Process::Steam).unwrap();
    // println!("test has tags {:?}", test.get_tags().get_tag_names());
    // test.apply_process(Process::Freeze).unwrap();
    // println!("test has tags {:?}", test.get_tags().get_tag_names());
    // const nut_bark: u8 = 0b00010010;
    // const wood_bark: u8 = 0b00010100;
    fn add_all_process(potion: Tags, depth: i32, all: &mut IndexSet<u8>, found: &mut IndexSet<u8>) {
        all.remove(&potion.0);
        found.insert(potion.0);
        if depth == 0 {
            return;
        }
        for ingredient in PARTS {
            let next = potion | ingredient.get_tags();
            if next == potion {
                continue;
            }
            add_all_process(next, depth - 1, all, found);
            for process in Process::iter() {
                let ingredient = ingredient;
                match ingredient.apply_process(process) {
                    Err(e) => match e {
                        CraftingError::NoItem | CraftingError::Bug => println!("{e}"),
                        CraftingError::Potion | CraftingError::DuplicateProcess(_) => {}
                    },
                    Ok(parts) => {
                        for part in parts {
                            if part == ingredient {
                                continue;
                            }
                            add_all_process(potion | part.get_tags(), depth - 1, all, found);
                        }
                    }
                }
            }
        }
    }
    add_all_process(Tags::EMPTY, 2, &mut all, &mut found);
    let mut new_found = IndexSet::new();
    for potion in found {
        add_all_process(Tags(potion), 2, &mut all, &mut new_found)
    }
    let mut all_effects = PotionEffect::iter().collect::<std::collections::HashSet<PotionEffect>>();
    let potions = ron::from_str::<
        std::collections::HashMap<u8, std::collections::HashSet<PotionEffect>>,
    >(&std::fs::read_to_string("potion.effects").unwrap())
    .unwrap();
    for potion in 0..=255 {
        if all.contains(&potion) {
            continue;
        }
        for effects in potions.get(&potion).unwrap() {
            all_effects.remove(effects);
        }
    }
    println!("Need ({}) {:?}", all.len(), all);
    println!("Can't Get {:?}", all_effects);
}

fn freeze(val: u8) -> u8 {
    val ^ 0xF0
}

fn burn(val: u8) -> u8 {
    val ^ 0x0F
}

fn spin_left(mut val: u8) -> u8 {
    for _ in 0..4 {
        val = val.rotate_left(1);
        if val >= 128 {
            return val;
        }
    }
    val
    // val.rotate_left(4)
}

fn spin_right(mut val: u8) -> u8 {
    for _ in 0..4 {
        val = val.rotate_right(1);
        if val & 1 == 1 {
            return val;
        }
    }
    val
    // val.rotate_right(4)
}

fn chop(val: u8) -> u8 {
    val ^ 0b01010101
}

fn grind(val: u8) -> u8 {
    val ^ 0b10101010
}

fn shake(val: u8) -> u8 {
    !val
}

fn blend(val: u8) -> u8 {
    val ^ 0b11000011
}

fn mix(val: u8) -> u8 {
    val ^ 0b00111100
}

fn dice(val: u8) -> u8 {
    let mut out: u8 = 0;
    for i in 0..8 {
        out = out.rotate_left(1) | (val.rotate_right(i) & 1);
    }
    val >> 4 | val << 4
}

// #[derive(
//     Debug, strum_macros::EnumIter, Clone, Copy, PartialEq, strum_macros::EnumString, ValueEnum,
// )]
// pub enum Process {
//     /// Spin the item real fast CCW this will rotate all the bits to the left atlast 1 up to 4 unless a 1 hits an edge
//     SpinLeft,
//     /// Spin the item real fast CW this will rotate all the bits to the left atlast 1 up to 4 unless a 1 hits an edge
//     SpinRight,
//     /// Put it in the freezer; XOr 11110000
//     Freeze,
//     /// Set it on Fire; XOr 00001111
//     Burn,
//     /// Cut it into big chunks; Flip around Middle 12345678 -> 432187654
//     Dice,
//     /// Chop it into odd bits; XOr with 01010101
//     Chop,
//     /// Grind it into even bits; XOr with 10101010
//     Grind,
//     /// Shake it up; not
//     Shake,
//     /// Blend it up; Xor 11000011
//     Blend,
//     /// Mix it around; Xor 00111100
//     Mix,
//     /// Give it a lick so see whats in it
//     Taste,
// }

#[derive(
    Debug, strum_macros::EnumIter, Clone, Copy, PartialEq, strum_macros::EnumString, ValueEnum,
)]
pub enum Process {
    /// See What Tags are in it
    Test,
    /// Collect Volatile Tags
    Distill = 1,
    /// Collect Heavy Tags
    Condense = 1 << 1,
    /// Removes Cold Tags
    Boil = 1 << 2,
    /// Removes Hot Tags
    Freeze = 1 << 3,
    /// Removes Unstable Tags
    Age = 1 << 4,
    /// Removes Volatile Tags add Fire
    Burn = 1 << 5,
    // Removes Heavy Tags
    Spin = 1 << 6,
    /// Remove Elemental
    Steam,
}

impl Process {
    pub fn can_do(val: u8) -> Vec<Process> {
        let mut vals = Vec::new();
        for process in Process::iter() {
            if process as u8 & val == 0 {
                vals.push(process)
            }
        }
        vals
    }
}
