use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use strum::EnumCount;
use strum_macros::{EnumIter, EnumCount};
use strum::IntoEnumIterator;

use crate::inventory::Ingredient;

#[derive(Clone, Copy, PartialEq, Eq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect_value()]
pub struct Tags(pub u8);

impl std::fmt::Debug for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.get_tag_names()))
    }
}

impl Tags {
    pub const EMPTY: Tags = Tags(0);

    pub fn new(tags: impl IntoIterator<Item = TagNames>) -> Tags {
        let mut new = Tags::EMPTY;
        for tag in tags {
            new.add_tag(tag);
        }
        new
    }

    pub fn get_tag_names(&self) -> Vec<TagNames> {
        let mut has = Vec::with_capacity(TagNames::COUNT);
        for tag in TagNames::iter() {
            if self.0 & tag as u8 == tag as u8 {
                has.push(tag);
            }
        }
        has
    }

    pub fn has_tag(self, tag: TagNames) -> bool {
        self.0 & tag as u8 == tag as u8
    }

    pub fn has_all(self, tags: impl IntoIterator<Item = TagNames>) -> bool {
        let target = Tags::new(tags);
        self & target == target
    }

    pub fn has_any(self, tags: impl IntoIterator<Item = TagNames>) -> bool {
        let target = Tags::new(tags);
        (self & target).0 > 0
    }

    pub fn count(&self) -> u32 {
        self.0.count_ones()
    }

    #[inline(always)]
    pub fn add_tag(&mut self, tag: TagNames) {
        self.0 |= tag as u8;
    }

    pub fn in_group(self, group: TagGroups) -> Tags {
        Tags(self.0 & group as u8)
    }

    pub fn not_in_group(self, group: TagGroups) -> Tags {
        Tags(self.0 & !(group as u8))
    }

    pub fn remove_tag(&mut self, tag: TagNames) {
        self.0 &= !(tag as u8)
    }

    #[inline(always)]
    pub fn with_tag(mut self, tag: TagNames) -> Tags {
        self.add_tag(tag);
        self
    }
}

#[test]
fn test_remove() {
    let mut tags = Tags::new([TagNames::Life]);
    assert!(!tags.has_tag(TagNames::Air));
    assert!(tags.has_tag(TagNames::Life));
    tags.add_tag(TagNames::Air);
    assert!(tags.has_tag(TagNames::Air));
    assert!(tags.has_tag(TagNames::Life));
    tags.remove_tag(TagNames::Life);
    assert!(tags.has_tag(TagNames::Air));
    assert!(!tags.has_tag(TagNames::Life));
    tags.remove_tag(TagNames::Air);
    assert!(!tags.has_tag(TagNames::Air));
    assert!(!tags.has_tag(TagNames::Life));
}

#[test]
fn groups() {
    let tags = Tags::new([TagNames::Life, TagNames::Air, TagNames::Earth]);
    assert!(tags.in_group(TagGroups::Heavy).has_tag(TagNames::Earth));
    assert!(!tags.in_group(TagGroups::Heavy).has_tag(TagNames::Air));
    assert!(!tags.not_in_group(TagGroups::Heavy).has_tag(TagNames::Earth));
    assert!(tags.not_in_group(TagGroups::Heavy).has_tag(TagNames::Air));
}

impl std::ops::BitAnd for Tags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Tags(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for Tags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Tags(self.0 | rhs.0)
    }
}

#[derive(Debug, EnumIter, EnumCount, Clone, Copy)]
pub enum TagNames {
    Tropical = 1,
    Life = 1<<1,
    Air = 1<<2,
    Earth = 1<<3,
    Water = 1<<4,
    Fire = 1<<5,
    Fibrous = 1<<6,
    Time = 1<<7,
}

pub enum TagGroups {
    Hot =           0b00100001,
    Cold =          0b00010100,
    Elemental =     0b00111100,
    Light =         0b00100100,
    Heavy =         0b00001000,
    Volatile =      0b00100111,
    Stable =        0b11011000,
}