use crate::prelude::*;
use thiserror::Error;
#[derive(Clone)]
struct ProcessTest(u8, u8);

impl ProcessTest {
    fn apply_process(&mut self, process: Process) {
        if self.1 & u8::from(process) > 0 {return;}
        self.1 |= u8::from(process);
        self.0 = match process {
            Process::SpinLeft => spin_left(self.0),
            Process::SpinRight => spin_right(self.0),
            Process::Freeze => freeze(self.0),
            Process::Burn => burn(self.0),
            Process::Chop => chop(self.0),
            Process::Grind => grind(self.0),
            Process::Shake => shake(self.0),
            Process::Blend => blend(self.0),
            Process::Mix => mix(self.0),
            Process::Dice => dice(self.0)
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
        match self {
            Item::Empty => return Err(CraftingError::NoItem),
            Item::Potion(_) => return Err(CraftingError::Potion),
            Item::Ingredient(_, part) => *self = Item::Intimidate(*part as u8, 0),
            Item::Intimidate(_, done) => {
                if *done & u8::from(process) > 0 {return Err(CraftingError::DuplicateProcess(u8::from(process)));} else {
                    *done |= u8::from(process);
                }},
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
            Process::Dice => dice(*val)
        };
        Ok(())
    }
}

#[test]
fn missing_vals() {
    use crate::plants::PlantPart;
    use std::collections::HashSet;
    let mut all = (0..=255).collect::<HashSet<u8>>();
    const parts: [Item; 6] = [Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Leaf),
    Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Seed),
    Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Root),
    Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Stem),
    Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Bark),
    Item::Ingredient(crate::plants::Plant::Palm, PlantPart::Fruit)];
    // const nut_bark: u8 = 0b00010010;
    // const wood_bark: u8 = 0b00010100;
    fn test_all(process_test: Item, depth: u8, all: &mut HashSet<u8>) {
        for process in <Process as strum::IntoEnumIterator>::iter() {
            let mut new_process_test = process_test;
            let res = new_process_test.apply_process(process);
            if depth == 0 {
                if res.is_ok() {
                    all.remove(&new_process_test.brew(Item::Potion(0)).unwrap());
                    for mut part in parts {
                        all.remove(&part.brew(new_process_test).unwrap());
                        for part_2 in parts {
                            let mut part = part;
                            all.remove(&part.brew(part_2).unwrap());
                            for part_3 in parts {
                                let mut part = part;
                                all.remove(&part.brew(part_3).unwrap());
                            }
                        }
                    }
                } else {
                    match res.unwrap_err() {
                        CraftingError::NoItem |
                        CraftingError::Potion |
                        CraftingError::Bug => println!("Error {:?}", res),
                        _ => {}
                    }
                    
                }
            } else {
                test_all(new_process_test, depth - 1, all);
            }
        }
    }
    for mut part in parts {
        all.remove(&part.brew(Item::Potion(0)).unwrap());
        for part_2 in parts {
            let mut part = part;
            all.remove(&part.brew(part_2).unwrap());
            for part_3 in parts {
                let mut part = part;
                all.remove(&part.brew(part_3).unwrap());
                for part_4 in parts {
                    let mut part = part;
                    all.remove(&part.brew(part_4).unwrap());
                }
            }
        }
    }
    for depth in 0..4 {
        for part in parts {
            test_all(part, depth, &mut all);
        }
        println!("after Part({}): {}", depth, all.len());
        println!("{:?}", all);
    }
}

fn freeze(val: u8) -> u8 {
    val ^ 0x0F
}

fn burn(val: u8) -> u8 {
    val ^ 0xF0
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

fn chop(mut val: u8) -> u8 {
    val ^ 0b01010101
}

fn grind(mut val: u8) -> u8 {
    val ^ 0b10101010
}

fn shake(mut val: u8) -> u8 {
    !val
}

fn blend(mut val: u8) -> u8 {
    val ^ 0b11000011
}

fn mix(mut val: u8) -> u8 {
    val ^ 0b00111100
}

fn dice(val: u8) -> u8 {
    let mut out: u8 = 0;
    for i in 0..8 {
        out = out.rotate_left(1) | (val.rotate_right(i) & 1);
    }
    val >> 4 | val << 4
}

#[derive(Debug, strum_macros::EnumIter, Clone, Copy, PartialEq)]
enum Process {
    SpinLeft,
    SpinRight,
    Freeze,
    Burn,
    Dice,
    Chop,
    Grind,
    Shake,
    Blend,
    Mix,
}

impl From<Process> for u8 {
    fn from(value: Process) -> Self {
        match value {
            Process::SpinLeft
            |Process::SpinRight => 1<<3,
            Process::Freeze |
            Process::Burn => 1,
            Process::Dice => 1 << 5,
            Process::Chop |
            Process::Grind => 2,
            Process::Shake => 0,
            Process::Blend | Process::Mix => 1 << 4,
        }
    }
}

