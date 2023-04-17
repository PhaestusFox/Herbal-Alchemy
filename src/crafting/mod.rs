use crate::{inventory::SelectedSlot, prelude::*};
use bevy::prelude::*;
use bevy_console::{reply, reply_failed, AddConsoleCommand, ConsoleCommand};
use clap::{Parser, ValueEnum};
use strum::IntoEnumIterator;
use thiserror::Error;

pub mod potions;

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
    mut item: Query<&mut Item, With<SelectedSlot>>,
) {
    if let Some(Ok(LabCommand { process })) = log.take() {
        let Ok(mut item) = item.get_single_mut() else {
            reply_failed!(log, "No Item Selected; Click an item to select it.");
            return;
        };
        let old = *item;
        if let Process::Taste = process {
            let taste = item.taste();
            reply!(log, "{taste}");
            return;
        }
        if let Err(e) = item.apply_process(process) {
            reply_failed!(log, "{e}")
        } else {
            if let Item::Intimidate(id, _) = *item {
                reply!(log, "{old:?} -> {:08b}", id);
            } else {
                reply!(log, "this is a bug; please report how you did it");
            }
        };
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
    mut item: Query<(Entity, &mut Item, &Slot), With<SelectedSlot>>,
    mut potion: ResMut<CurrentPotion>,
    mut events: EventWriter<InventoryEvent>,
    mut commands: Commands,
) {
    let Some(Ok(PotionCommand { action })) = log.take() else {return;};
    match action {
        PotionAcction::Bottle => {
            events.send(InventoryEvent::AddItem(potion.0));
        }
        PotionAcction::Add => {
            let Ok((entity, mut item, slot)) = item.get_single_mut() else {reply_failed!(log, "No Item Selected; Click an item to select it."); return;};
            if let Err(e) = potion.0.brew(*item) {
                reply_failed!(log, "{e}");
            } else {
                events.send(InventoryEvent::RemoveItem(*slot));
            }
        }
        PotionAcction::Empty => potion.0 = Item::Potion(0),
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
    DuplicateProcess(u8),
    #[error("This is a bug please report how you got this message")]
    Bug,
}

impl Item {
    fn apply_process(&mut self, process: Process) -> Result<(), CraftingError> {
        if let Process::Taste = process {
            return Ok(());
        }
        match self {
            Item::Empty => return Err(CraftingError::NoItem),
            Item::Potion(_) => return Err(CraftingError::Potion),
            Item::Ingredient(_, part) => *self = Item::Intimidate(*part as u8, u8::from(process)),
            Item::Intimidate(_, done) => {
                if *done & u8::from(process) > 0 {
                    return Err(CraftingError::DuplicateProcess(u8::from(process)));
                } else {
                    *done |= u8::from(process);
                }
            }
        };
        let Item::Intimidate(val, _) = self else {return Err(CraftingError::Bug);};
        *val = match process {
            Process::SpinLeft => spin_left(*val),
            Process::SpinRight => spin_right(*val),
            Process::Freeze => freeze(*val),
            Process::Burn => burn(*val),
            Process::Chop => chop(*val),
            Process::Grind => grind(*val),
            Process::Shake => shake(*val),
            Process::Blend => blend(*val),
            Process::Mix => mix(*val),
            Process::Dice => dice(*val),
            Process::Taste => *val,
        };
        Ok(())
    }
}

#[test]
fn missing_vals() {
    use crate::plants::PlantPart;
    use std::collections::HashSet;
    let mut all = (0..=255).collect::<HashSet<u8>>();
    const PARTS: [Item; 6] = [
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Leaf),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Seed),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Root),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Stem),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Bark),
        Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Fruit),
    ];
    // const nut_bark: u8 = 0b00010010;
    // const wood_bark: u8 = 0b00010100;
    fn test_all(process_test: Item, depth: u8, all: &mut HashSet<u8>) {
        for process in <Process as strum::IntoEnumIterator>::iter() {
            let mut new_process_test = process_test;
            let res = new_process_test.apply_process(process);
            if depth == 0 {
                if res.is_ok() {
                    all.remove(&new_process_test.brew(Item::Potion(0)).unwrap());
                    for mut part in PARTS {
                        all.remove(&part.brew(new_process_test).unwrap());
                        for part_2 in PARTS {
                            let mut part = part;
                            all.remove(&part.brew(part_2).unwrap());
                            for part_3 in PARTS {
                                let mut part = part;
                                all.remove(&part.brew(part_3).unwrap());
                            }
                        }
                    }
                } else {
                    match res.unwrap_err() {
                        CraftingError::NoItem | CraftingError::Potion | CraftingError::Bug => {
                            println!("Error {:?}", res)
                        }
                        _ => {}
                    }
                }
            } else {
                test_all(new_process_test, depth - 1, all);
            }
        }
    }
    for mut part in PARTS {
        all.remove(&part.brew(Item::Potion(0)).unwrap());
        for part_2 in PARTS {
            let mut part = part;
            all.remove(&part.brew(part_2).unwrap());
            for part_3 in PARTS {
                let mut part = part;
                all.remove(&part.brew(part_3).unwrap());
                for part_4 in PARTS {
                    let mut part = part;
                    all.remove(&part.brew(part_4).unwrap());
                }
            }
        }
    }
    for depth in 0..4 {
        for part in PARTS {
            test_all(part, depth, &mut all);
        }
        println!("after Part({}): {}", depth, all.len());
        println!("{:?}", all);
    }
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

#[derive(
    Debug, strum_macros::EnumIter, Clone, Copy, PartialEq, strum_macros::EnumString, ValueEnum,
)]
pub enum Process {
    /// Spin the item real fast CCW this will rotate all the bits to the left atlast 1 up to 4 unless a 1 hits an edge
    SpinLeft,
    /// Spin the item real fast CW this will rotate all the bits to the left atlast 1 up to 4 unless a 1 hits an edge
    SpinRight,
    /// Put it in the freezer; XOr 11110000
    Freeze,
    /// Set it on Fire; XOr 00001111
    Burn,
    /// Cut it into big chunks; Flip around Middle 12345678 -> 432187654
    Dice,
    /// Chop it into odd bits; XOr with 01010101
    Chop,
    /// Grind it into even bits; XOr with 10101010
    Grind,
    /// Shake it up; not
    Shake,
    /// Blend it up; Xor 11000011
    Blend,
    /// Mix it around; Xor 00111100
    Mix,
    /// Give it a lick so see whats in it
    Taste,
}

#[test]
fn test_can_do() {
    let mut item = Item::Intimidate(0, 0);
    item.apply_process(Process::Burn).unwrap();
    assert!(!item.can_do_process().contains(&Process::Burn));
    assert!(!item.can_do_process().contains(&Process::Freeze));
    println!("can do {:?}", item.can_do_process());
}

impl Process {
    pub fn can_do(val: u8) -> Vec<Process> {
        let mut vals = Vec::new();
        for process in Process::iter() {
            if u8::from(process) & val == 0 {
                vals.push(process)
            }
        }
        vals
    }
}

impl From<Process> for u8 {
    fn from(value: Process) -> Self {
        match value {
            Process::SpinLeft | Process::SpinRight => 1 << 3,
            Process::Freeze | Process::Burn => 1,
            Process::Dice => 1 << 5,
            Process::Chop | Process::Grind => 2,
            Process::Shake | Process::Taste => 0,
            Process::Blend | Process::Mix => 1 << 4,
        }
    }
}
