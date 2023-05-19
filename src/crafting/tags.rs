use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::{EnumCount, EnumIter};

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
    Life = 1 << 1,
    Air = 1 << 2,
    Earth = 1 << 3,
    Water = 1 << 4,
    Fire = 1 << 5,
    Fibrous = 1 << 6,
    Time = 1 << 7,
}

pub enum TagGroups {
    Hot = 0b00100001,
    Cold = 0b00010100,
    Elemental = 0b00111100,
    Light = 0b00100100,
    Heavy = 0b00001000,
    Volatile = 0b00100111,
    Stable = 0b11011000,
}

enum Aspect {
    Aequalitas,  // Equality or Balance
    Aeternitas,  // Eternity or Timelessness
    Aevitas,     // Age or Time
    Amor,        // Love
    Anima,       // Soul or Spirit
    Celeritas,   // Speed or Swiftness
    Cibus,       // Food or Nourishment
    Cinis,       // Ash or Dust
    Coniunctio,  // Connection or Union
    Creatio,     // Creation
    Cruciatum,   // Torment or Suffering
    Decessus,    // Departure or Death
    Defensio,    // Defense or Protection
    Desiderium,  // Longing or Desire
    Destinatio,  // Destiny or Fate
    Dolor,       // Pain or Sorrow
    Dubitatio,   // Doubt or Uncertainty
    Egregia,     // Excellence or Greatness
    Exilium,     // Exile or Banishment
    Exspectatio, // Expectation or Anticipation
    Fames,       // Hunger or Craving
    Fortuna,     // Fortune or Luck
    Gloria,      // Glory or Fame
    Gravitas,    // Gravity or Seriousness
    Harena,      // Sand
    Hibernus,    // Winter or Cold
    Ictus,       // Strike or Blow
    Ignavia,     // Laziness or Indolence
    Imber,       // Rain
    Inanis,      // Emptiness or Void
    Incendium,   // Inferno or Blaze
    Infirmitas,  // Weakness or Frailty
    Iniuria,     // Injury or Wrong
    Insania,     // Madness or Insanity
    Ira,         // Anger or Wrath
    Labor,       // Labor or Toil
    Letum,       // Death
    Lux,         // Light
    Magnitudo,   // Magnitude or Greatness
    Malum,       // Evil or Misfortune
    Materia,     // Matter or Material
    Mortalis,    // Mortality or Death
    Mundus,      // World or Universe
    Natura,      // Nature
    Nox,         // Night or Darkness
    Oculus,      // Eye or Vision
    Omnis,       // All or Everything
    Opacus,      // Shadow or Darkness
    Pax,         // Peace
    Pietas,      // Piety or Duty
    Potentia,    // Power
    Praesidium,  // Protection or Guard
    Proelium,    // Battle or Combat
    Ratio,       // Reason or Logic
    Regnum,      // Kingdom or Reign
    Sanguis,     // Blood
    Scientia,    // Knowledge or Science
    Sensus,      // Sense or Perception
    Sol,         // Sun
    Sonitus,     // Sound or Noise
    Spatium,     // Space or Distance
    Spiritus,    // Breath or Spirit
    Stabilitas,  // Stability or Firmness
    Tempestas,   // Storm or Tempest
    Tenebrae,    // Darkness or Shadows
    Terra,       // Earth
    Timor,       // Fear or Dread
    Umbra,       // Shade or Shadow
    Vita,        // Life
    Aestus,      // Heat or Passion
    Alacritas,   // Cheerfulness or Eagerness
    Amicitia,    // Friendship
    Avaritia,    // Greed or Avarice
    Bellum,      // War or Battle
    Calamitas,   // Disaster or Misfortune
    Caritas,     // Charity or Love
    Aestas,      // Summer, Heat
    Aether,      // Ether, Upper Atmosphere
    Albedo,      // Whiteness, Reflectivity
    Altus,       // Height, Elevation
    Amplus,      // Size, Magnitude
    Aqua,        // Water, Liquid
    Arbor,       // Tree, Wood
    Aurum,       // Gold, Wealth
    Caelum,      // Sky, Heaven
    Calidus,     // Hot, Warm
    Canis,       // Dog, Loyalty
    Cognitio,    // Knowledge, Intellect
    Crepusculum, // Twilight, Dusk
    Deciduus,    // Deciduous, Shedding
    Exiguus,     // Small, Little
    Flos,        // Flower, Blossom
    Frigidus,    // Cold, Chill
    Fuligo,      // Soot, Smoke
    Glacialis,   // Icy, Frozen
    Igneus,      // Fiery, Flames
    Infinitus,   // Infinite, Boundless
    Inops,       // Poor, Needy
    Lapidem,     // Stone, Rock
    Lucidus,     // Clear, Bright
    Maritimus,   // Marine, Maritime
    Matutinus,   // Morning, Dawn
    Mus,         // Mouse, Timidity
    Mutabilis,   // Changeable, Mutable
    Nocturnus,   // Nocturnal, Nighttime
    Nubes,       // Cloud, Mist
    Oceanus,     // Ocean, Sea
    Omnipotens,  // Almighty, All-Powerful
    Parvus,      // Small, Little
    Perennis,    // Perennial, Everlasting
    Pristinus,   // Ancient, Primitive
    Pulcher,     // Beautiful, Handsome
    Purus,       // Pure, Clean
    Radius,      // Ray, Beam
    Ruber,       // Red, Crimson
    Siccus,      // Dry, Arid
    Silex,       // Flint, Stone
    Somnus,      // Sleep, Slumber
    Stellaris,   // Stellar, Starry
    Summus,      // Highest, Supreme
    Terminus,    // End, Boundary
    Tertius,     // Third, Tertiary
    Totus,       // Whole, Entire
    Unda,        // Wave, Water
    Ventus,      // Wind, Breeze
    Ver,         // Spring, Greenery
    Vesper,      // Evening, Twilight
    Vires,       // Strength, Power
}