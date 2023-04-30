use crate::prelude::*;
use bevy::prelude::*;
use std::collections::HashSet as HashSet;
use rand::{seq::IteratorRandom, Rng};
use std::fmt::Display;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use PotionEffect::*;

use super::tags::TagNames;
impl PotionEffect {
    pub fn get_potion_effects(val: Tags) -> Vec<PotionEffect> {
        use PotionEffect::*;
        let mut effects = Vec::new();
        // the higer the number of bits the stronger the effect
        match val.count() {
            //no tags is just water
            0 => return vec![],
            //all tags is too easy
            8 => return vec![InstantDeath],
            //too many bits is boring
            7 | 6 => effects.push(Paralysis),
            _ => {}
        }
        // life over time is Regeneration
        if val.has_all([TagNames::Life, TagNames::Time]) {
            effects.push(Regeneration)
        }
        // time without life is Poison
        if val.has_tag(TagNames::Time) && !val.has_tag(TagNames::Life) {
            effects.push(Poison)
        }
        match val.0 % 0b01000101 {
            // 01000101 is as "Lucky" number
            0 => effects.push(Luck),
            // if you just miss being "Lucky" you are "Unlucky"
            1 => effects.push(BadLuck),
            // if you just miss being "Lucky" you are "Unlucky"
            0b01000100 => effects.push(BadLuck),
            _ => {}
        }
        // conflicting bits are bad
        if val.in_group(crate::crafting::tags::TagGroups::Elemental).count() > 3
        {
            effects.push(Nausea)
        }
        if !val.has_all([TagNames::Life, TagNames::Tropical]) { //add 
            match (val.has_tag(TagNames::Fire), val.has_tag(TagNames::Air), val.has_tag(TagNames::Water), val.has_tag(TagNames::Earth)) {
                (true, true, true  , true) => effects.push(InfernoBlizzard),
                (true, true, false , false) => effects.push(FireStorm),
                (false, true, true , false) => effects.push(IceStorm),
                (true, false, true , false) => effects.push(EmberFrost),
                (true, false, false, true) => effects.push(FireBall),
                (false, false, true, true) => effects.push(SnowBall),
                (true, true, true, false) => effects.push(FrostFire),
                (true, false, true, true) => effects.push(Explosion),
                (false, true, true, true) => effects.push(Blizzard),
                (true, true, false, true) => effects.push(Sandstorm),// todo!(add fire, air, earth effect)
                (false, true, false, true) |
                (true, false, false, false) |
                (false, false, false, true) |
                (false, true, false, false) |
                (false, false, true, false) |
                (false, false, false, false) => {}
            }
        }
        if val.has_all([TagNames::Water, TagNames::Time]) {
            effects.push(Saturation)
        }
        if val.has_all([TagNames::Air, TagNames::Fire]) {
            effects.push(Invisibility)
        }
        if val.has_all([TagNames::Life, TagNames::Fibrous]) && !val.has_all([TagNames::Earth, TagNames::Water]) {
            effects.push(Strength)
        }
        if !val.has_tag(TagNames::Earth) && val.in_group(crate::crafting::tags::TagGroups::Light).count() > 0 {
            effects.push(Levitation)
        }
        if val.has_all([TagNames::Time, TagNames::Life]) && !val.has_tag(TagNames::Tropical) {
            effects.push(Confusion)
        }
        if val.has_all([TagNames::Life, TagNames::Earth, TagNames::Fire]) && !val.has_tag(TagNames::Time) {
            effects.push(Inflammation)
        }
        if val.has_any([TagNames::Air, TagNames::Fire]) && val.has_tag(TagNames::Tropical) && !val.has_tag(TagNames::Time) {
            effects.push(Teleportation)
        }
        if val.has_tag(TagNames::Earth) && !val.has_tag(TagNames::Water){
            effects.push(IslandOasis)
        }
        if val.has_tag(TagNames::Water) && !val.has_tag(TagNames::Earth) {
            effects.push(TidalWave)
        }
        effects
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, Hash, serde::Serialize, serde::Deserialize)]
pub enum PotionEffect {
    InstantDeath,
    Explosion,
    Blizzard,
    EmberFrost,
    FrostFire,
    InfernoBlizzard,
    IceStorm,
    FireStorm,
    // FrostBolt,
    SnowBall,
    FireBall,
    // FireBolt,
    Paralysis,
    Saturation,
    Luck,
    BadLuck,
    Nausea,
    Poison,
    Regeneration,
    Invisibility,
    Strength,
    Levitation,
    Confusion,
    Inflammation,
    Teleportation,
    IslandOasis,
    TidalWave,
    // Desert Mirage
    Sandstorm
}

impl Display for PotionEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}(", self))?;
        match self {
            InstantDeath => f.write_fmt(format_args!("{:08b}", 255)),
            Blizzard => f.write_str("Water, Earth, Air"),
            Explosion => f.write_str("Fire, Water, Earth"),
            Paralysis => f.write_str("6 > tags"),
            Saturation => f.write_str("Time + Water"),
            Luck => f.write_str("self % 01000101 = 0"),
            BadLuck => f.write_str("self % 0100101 = 1 | -1"),
            Nausea => f.write_str("All Elemental Tags"),
            Poison => f.write_str("Time && !Life"),
            Regeneration => f.write_str("Time && Life"),
            Invisibility => f.write_str("Air & Fire"),
            Strength => f.write_str("(Life && Fibrous) && !(Earth && Water)"),
            Levitation => f.write_str("!Earth && !Time & Light > 0"),
            Confusion => f.write_str("Time && Life & !Tropical"),
            Inflammation => f.write_str("Life && Earth && Fire & !Time"),
            Teleportation => f.write_str("(Air | Fire) && Tropical & !Time"),
            FireBall => f.write_str("Fire && Earth"),
            SnowBall => f.write_str("Water && Earth"),
            FrostFire => f.write_str("Fire && Air && Water"),
            InfernoBlizzard => f.write_str("Fire && Air && Water && Air"),
            IceStorm => f.write_str("Water && Air"),
            FireStorm => f.write_str("Water && Fire"),
            IslandOasis => f.write_str("Earth && !Water"),
            TidalWave => f.write_str("Water && !Earth"),
            EmberFrost => f.write_str("Fire && Water"),
            Sandstorm => f.write_str("Earth && Air && Fire")
        }?;
        f.write_str(")")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, strum_macros::EnumIter, Hash)]
pub enum EffectTags {
    Death,
    Negative,
    SideEffect,
    Instant,
    Ice,
    Fire,
    AreaOfEffect,
    Slow,
    Weather,
    Projectile,
    DamageOverTime,
    Destructive,
    Distracting,
    Health,
    Positive,
    Random,
    Stealth,
    Movement,
    EffectSelf,
    EffectTarget,
    Elemental,
}

/*
pub enum EffectTags {
    Buff,
    Debuff,
    Magic,
    Energy,
    Poison,
    Healing,
    Illusion,
    Transformation,
    MindControl,
    Summoning,
    Protection,
    Light,
    Darkness,
    Sound,
    Time,
    Gravity,
    Dimensional,
    Nature,
    Technology,
    Divine,
    Cursed,
}
*/

impl std::fmt::Display for EffectTags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl PotionEffect {
    pub fn get_tags(&self) -> &'static [EffectTags] {
        match self {
            InstantDeath => &[
                EffectTags::Death,
                EffectTags::Instant,
                EffectTags::Destructive,
            ],
            Explosion => &[
                EffectTags::Destructive,
                EffectTags::AreaOfEffect,
                EffectTags::Projectile,
            ],
            Blizzard => &[
                EffectTags::AreaOfEffect,
                EffectTags::Ice,
                EffectTags::Weather,
                EffectTags::Slow,
            ],
            FrostFire => {
                &[EffectTags::AreaOfEffect, EffectTags::Ice, EffectTags::Fire]
            }
            InfernoBlizzard => &[
                EffectTags::AreaOfEffect,
                EffectTags::Fire,
                EffectTags::Ice,
                EffectTags::Weather,
                EffectTags::Destructive,
            ],
            IceStorm => &[
                EffectTags::AreaOfEffect,
                EffectTags::Ice,
                EffectTags::Weather,
                EffectTags::Slow,
            ],
            FireStorm => &[
                EffectTags::AreaOfEffect,
                EffectTags::Fire,
                EffectTags::Weather,
                EffectTags::Destructive,
            ],
            SnowBall => &[EffectTags::Projectile, EffectTags::Ice],
            FireBall => &[
                EffectTags::Projectile,
                EffectTags::Fire,
                EffectTags::Destructive,
            ],
            Paralysis => &[
                EffectTags::Negative,
                EffectTags::SideEffect,
                EffectTags::Movement,
            ],
            Saturation => &[EffectTags::Positive, EffectTags::Health],
            Luck => &[EffectTags::Positive, EffectTags::Random],
            BadLuck => &[EffectTags::Negative, EffectTags::Random],
            Nausea => &[EffectTags::Negative, EffectTags::SideEffect],
            Poison => &[EffectTags::Negative, EffectTags::DamageOverTime],
            Regeneration => &[EffectTags::Positive, EffectTags::Health],
            Invisibility => &[EffectTags::Positive, EffectTags::Stealth],
            Strength => &[EffectTags::Positive, EffectTags::Movement],
            Levitation => &[EffectTags::Positive, EffectTags::Movement],
            Confusion => &[EffectTags::Negative, EffectTags::SideEffect],
            Inflammation => &[EffectTags::Negative, EffectTags::DamageOverTime],
            Teleportation => &[
                EffectTags::Positive,
                EffectTags::Movement,
                EffectTags::Instant,
            ],
            IslandOasis => &[
                EffectTags::Positive,
                EffectTags::AreaOfEffect,
                EffectTags::Weather,
            ],
            TidalWave => &[
                EffectTags::Destructive,
                EffectTags::AreaOfEffect,
                EffectTags::Projectile,
            ],
            EmberFrost => &[EffectTags::Fire, EffectTags::Ice, EffectTags::DamageOverTime, EffectTags::Destructive, EffectTags::AreaOfEffect, EffectTags::Projectile],
            Sandstorm => &[EffectTags::Destructive, EffectTags::AreaOfEffect, EffectTags::Slow, EffectTags::Weather, EffectTags::Negative],
        }
    }

    fn conflicts(&self) -> &'static [PotionEffect] {
        match self {
            InstantDeath => &[
                Teleportation,
                Explosion,
                Blizzard,
                FrostFire,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                SnowBall,
                FireBall,
                Paralysis,
                Saturation,
                Luck,
                BadLuck,
                Nausea,
                Poison,
                Regeneration,
                Invisibility,
                Strength,
                Levitation,
                Confusion,
                Inflammation,
            ],
            Explosion => &[
                InstantDeath,
                Teleportation,
                Blizzard,
                FrostFire,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                SnowBall,
                FireBall,
                Paralysis,
                Saturation,
                Luck,
                BadLuck,
                Nausea,
                Poison,
                Regeneration,
                Invisibility,
                Strength,
                Levitation,
                Confusion,
                Inflammation,
            ],
            Blizzard => &[
                InstantDeath,
                Explosion,
                Teleportation,
                FrostFire,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                SnowBall,
                FireBall,
                Paralysis,
                Saturation,
                Luck,
                BadLuck,
                Nausea,
                Poison,
                Regeneration,
                Invisibility,
                Strength,
                Levitation,
                Confusion,
                Inflammation,
            ],
            FrostFire => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Luck,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                SnowBall,
                FireBall,
                Paralysis,
                Saturation,
            ],
            InfernoBlizzard => &[
                InstantDeath,
                Explosion,
                Blizzard,
                FrostFire,
                Teleportation,
                IceStorm,
                FireStorm,
                SnowBall,
                FireBall,
                Strength,
                Saturation,
                Luck,
                BadLuck,
                Levitation,
            ],
            IceStorm => &[
                InstantDeath,
                Explosion,
                Blizzard,
                FrostFire,
                InfernoBlizzard,
                Teleportation,
                FireStorm,
                SnowBall,
                FireBall,
                Paralysis,
                Saturation,
                Luck,
                Strength,
                Confusion,
                Poison,
                Regeneration,
                Levitation,
            ],
            FireStorm => &[
                InstantDeath,
                Explosion,
                Blizzard,
                FrostFire,
                InfernoBlizzard,
                IceStorm,
                Teleportation,
                SnowBall,
                FireBall,
                Paralysis,
                Saturation,
                Luck,
                BadLuck,
                Inflammation,
                Poison,
                Regeneration,
                Levitation,
                Strength,
            ],
            SnowBall => &[
                InstantDeath,
                Explosion,
                Blizzard,
                FrostFire,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                Teleportation,
                FireBall,
                Confusion,
                Saturation,
                Luck,
                BadLuck,
            ],
            FireBall => &[
                InstantDeath,
                Explosion,
                Blizzard,
                FrostFire,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                SnowBall,
                Teleportation,
                Inflammation,
                Saturation,
            ],
            Paralysis => &[
                InstantDeath,
                Explosion,
                Blizzard,
                FrostFire,
                BadLuck,
                IceStorm,
                FireStorm,
                Saturation,
            ],
            Saturation => &[
                InstantDeath,
                Explosion,
                Blizzard,
                FrostFire,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                SnowBall,
                FireBall,
                Paralysis,
                Teleportation,
                Luck,
                BadLuck,
                Nausea,
                Poison,
                Regeneration,
                Invisibility,
                Strength,
                Levitation,
                Confusion,
                Inflammation,
            ],
            Luck => &[
                InstantDeath,
                Explosion,
                Blizzard,
                FrostFire,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                SnowBall,
                Levitation,
                Inflammation,
                Saturation,
                Teleportation,
                BadLuck,
                Confusion,
                Poison,
                Regeneration,
                Invisibility,
                Strength,
            ],
            BadLuck => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Levitation,
                InfernoBlizzard,
                Inflammation,
                FireStorm,
                SnowBall,
                Confusion,
                Paralysis,
                Saturation,
                Luck,
                Teleportation,
                Nausea,
                Poison,
                Regeneration,
                Invisibility,
            ],
            Nausea => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Teleportation,
                Saturation,
                BadLuck,
            ],
            Poison => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Strength,
                Regeneration,
                IceStorm,
                FireStorm,
                Teleportation,
                Confusion,
                Levitation,
                Saturation,
                Luck,
                BadLuck,
            ],
            Regeneration => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Inflammation,
                Poison,
                IceStorm,
                FireStorm,
                Strength,
                Teleportation,
                Levitation,
                Saturation,
                Luck,
                BadLuck,
            ],
            Invisibility => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Luck,
                Saturation,
                BadLuck,
                Teleportation,
            ],
            Strength => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Teleportation,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                Regeneration,
                Confusion,
                Poison,
                Saturation,
                Luck,
                Inflammation,
                Levitation,
            ],
            Levitation => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Strength,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                Poison,
                Regeneration,
                Confusion,
                Saturation,
                Luck,
                BadLuck,
            ],
            Confusion => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Levitation,
                BadLuck,
                IceStorm,
                Strength,
                SnowBall,
                Inflammation,
                Poison,
                Saturation,
                Luck,
            ],
            Inflammation => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Confusion,
                BadLuck,
                Strength,
                FireStorm,
                Teleportation,
                FireBall,
                Regeneration,
                Saturation,
                Luck,
            ],
            Teleportation => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Inflammation,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                SnowBall,
                FireBall,
                Strength,
                Saturation,
                Luck,
                BadLuck,
                Nausea,
                Poison,
                Regeneration,
                Invisibility,
            ],
            IslandOasis => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Regeneration,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                Poison,
                Confusion,
                TidalWave,
                Saturation,
                Luck,
            ],
            TidalWave => &[
                InstantDeath,
                Explosion,
                Blizzard,
                Inflammation,
                InfernoBlizzard,
                IceStorm,
                FireStorm,
                Levitation,
                Teleportation,
                Strength,
                Saturation,
                Luck,
                IslandOasis,
                Nausea,
                Poison,
                Regeneration,
                Confusion,
            ],
            EmberFrost => &[],
            Sandstorm => &[],
            
        }
    }

    fn inalienable(&self) -> &'static [PotionEffect] {
        match self {
            InfernoBlizzard => &[Nausea, Paralysis],
            _ => &[],
        }
    }
}

#[derive(Resource, Clone, Copy)]
pub struct TargetPotion {
    customer: Customer,
    main: Option<PotionEffect>,
    extra: Option<Rule>,
}

impl FromWorld for TargetPotion {
    fn from_world(_: &mut World) -> Self {
        TargetPotion::new()
    }
}

impl TargetPotion {
    pub fn new() -> TargetPotion {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.01) {
            return TargetPotion {
                customer: Customer::new(),
                main: None,
                extra: None,
            };
        }
        let mut valid_effects: HashSet<PotionEffect> = PotionEffect::iter().collect();
        let mut valid_tags: HashSet<EffectTags> = EffectTags::iter().collect();
        let effect = *valid_effects.iter().choose(&mut rng).unwrap();
        if let InstantDeath = effect {
            return TargetPotion {
                customer: Customer::new(),
                main: Some(InstantDeath),
                extra: None,
            };
        }
        valid_effects.remove(&InstantDeath);
        valid_effects.remove(&effect);
        for tag in effect.get_tags() {
            valid_tags.remove(tag);
        }
        for conflict in effect.conflicts() {
            valid_effects.remove(conflict);
        }
        let mut extra = None;
        match rng.gen_range(0..10) {
            0 => {
                if let Some(val) = valid_effects.iter().choose(&mut rng) {
                    extra = Some(Rule::Effect(*val));
                }
            }
            1 => {
                for _ in 0..4 {
                    if let Some(temp) = valid_effects.iter().choose(&mut rng) {
                        if !effect.inalienable().contains(temp) {
                            extra = Some(Rule::NotEffect(*temp));
                            break;
                        }
                    }
                }
            }
            2 => {
                for _ in 0..5 {
                    if let Some(other) = valid_tags.iter().choose(&mut rng) {
                        for effect in valid_effects.iter() {
                            if effect.get_tags().contains(other) {
                                extra = Some(Rule::Tag(*other));
                            }
                        }
                    }
                }
            }
            3 => {
                if let Some(val) = valid_tags.iter().choose(&mut rng) {
                    extra = Some(Rule::NotTag(*val));
                }
            }
            _ => {}
        }
        TargetPotion {
            customer: Customer::new(),
            main: Some(effect),
            extra,
        }
    }

    pub fn potion_request(&self) -> String {
        let Some(main) = self.main else {return String::from(self.customer.get_water_text())};
        let mut request = self
            .customer
            .get_main_order_text()
            .replace("{}", &main.to_string());
        if let Some(extra) = self.extra {
            request.push('\n');
            request.push_str(&self.customer.get_extra_text(extra))
        }
        request.push_str(&format!(
            "\n\n- {} {:?}",
            self.customer.mood, self.customer.archetype
        ));
        request
    }

    pub fn is_match(&self, new: Tags) -> Result<(), String> {
        let Some(main) = self.main else {return if new == Tags::EMPTY {Ok(())} else {Err(String::from("Thats not water"))};};
        let effects = PotionEffect::get_potion_effects(new);
        if !effects.contains(&main) {
            return Err("It Dosen't Have the needed effect".into());
        }
        match &self.extra {
            None => Ok(()),
            Some(Rule::Effect(effect)) => if effects.contains(effect) {
                Ok(())
            } else {
                Err("Id Doesnt Have the added Effect".into())
            },
            Some(Rule::NotEffect(effect)) => if effects.contains(effect) {
                Err("Id Have an unwanted Effect".into())
            } else {
                Ok(())
            },
            Some(Rule::Tag(tag)) => {
                let mut tags = HashSet::new();
                for effect in effects {
                    for tag in effect.get_tags() {
                        tags.insert(*tag);
                    }
                }
                if tags.contains(tag) {
                    Ok(())
                } else {
                    Err("It doesn't have the needed tag".into())
                }
            }
            Some(Rule::NotTag(tag)) => {
                let mut tags = HashSet::new();
                for effect in effects {
                    for tag in effect.get_tags() {
                        tags.insert(*tag);
                    }
                }
                if tags.contains(tag) {
                    Err("It has and unwanted tag".into())
                } else {
                    Ok(())
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Customer {
    archetype: CustomerType,
    mood: CustomerMood,
}

impl Customer {
    fn new() -> Customer {
        let mut rng = rand::thread_rng();
        Customer {
            archetype: CustomerType::iter().choose(&mut rng).unwrap(),
            mood: CustomerMood::iter().choose(&mut rng).unwrap(),
        }
    }

    fn get_main_order_text(&self) -> &'static str {
        match self.archetype {
            CustomerType::Adventurer => match self.mood {
                CustomerMood::Happy => "Hello good sir, I am in need of a potion of {}.",
                CustomerMood::Angry => {
                    "I don't have time for pleasantries, just give me a potion of {} now!"
                }
                CustomerMood::Anxious => "Excuse me, I need a potion of {}. ",
                CustomerMood::Excited => {
                    "Wow, I'm so excited to try out a potion of {}! Can you make it for me?"
                }
                CustomerMood::Impatient => "Listen, I need a potion of {} and I need it right now.",
                CustomerMood::Indifferent => "Hi, I am in need of a potion of {}.",
                CustomerMood::Grumpy => "Ugh, just give me a potion of {} and be quick about it.",
            },
            CustomerType::Witch => match self.mood {
                CustomerMood::Happy => {
                    "Greetings, I require a potion of {} for a special occasion."
                }
                CustomerMood::Angry => "I demand a potion of {} immediately!",
                CustomerMood::Anxious => {
                    "Please, I need a potion of {} to complete a crucial spell."
                }
                CustomerMood::Excited => "Oh, I can't wait to try out a potion of {}!",
                CustomerMood::Impatient => "Hurry up, I need a potion of {} right now!",
                CustomerMood::Indifferent => "I suppose I'll take a potion of {}. Whatever.",
                CustomerMood::Grumpy => "Just give me a potion of {}. And make it snappy.",
            },
            CustomerType::Wizard => match self.mood {
                CustomerMood::Happy => {
                    "Greetings, I require a potion of {} to aid me in my studies."
                }
                CustomerMood::Angry => {
                    "Hurry up and give me a potion of {} before I turn you into a toad!"
                }
                CustomerMood::Anxious => "Please, I need a potion of {} as soon as possible!",
                CustomerMood::Excited => "I'm feeling adventurous today, how about a potion of {}?",
                CustomerMood::Impatient => {
                    "I don't have all day, give me a potion of {} right now!"
                }
                CustomerMood::Indifferent => {
                    "I suppose I could use a potion of {} if you have one."
                }
                CustomerMood::Grumpy => {
                    "What do you want? Just give me a potion of {} and be done with it."
                }
            },
            CustomerType::Alchemist => match self.mood {
                CustomerMood::Happy => {
                    "Greetings, I require a potion of {} for my latest experiment."
                }
                CustomerMood::Angry => "I demand a potion of {} immediately!",
                CustomerMood::Anxious => "Excuse me, can you make me a potion of {}?",
                CustomerMood::Excited => {
                    "Hello, I'm looking for a potion of {} to help me with a new discovery!"
                }
                CustomerMood::Impatient => "Hurry up! I need a potion of {} now!",
                CustomerMood::Indifferent => {
                    "Hi, I need a potion of {} for some research I'm doing."
                }
                CustomerMood::Grumpy => {
                    "I suppose I need a potion of {}...if you can even make one properly."
                }
            },
            CustomerType::Noble => match self.mood {
                CustomerMood::Happy => {
                    "Greetings, I require a potion of {} for my evening entertainment."
                }
                CustomerMood::Angry => "I demand a potion of {} at once!",
                CustomerMood::Anxious => "Excuse me, I am in need of a potion of {}. ",
                CustomerMood::Excited => {
                    "Good day, I require a potion of {} for my upcoming festivities!"
                }
                CustomerMood::Impatient => {
                    "I haven't got all day, I need a potion of {} right now!"
                }
                CustomerMood::Indifferent => "Hello there, I am in need of a potion of {}.",
                CustomerMood::Grumpy => "Just give me a potion of {} and be done with it!",
            },
            CustomerType::Peasant => match self.mood {
                CustomerMood::Happy => "Good day, I'm in need of a potion of {}.",
                CustomerMood::Angry => "I demand a potion of {} right now!",
                CustomerMood::Anxious => "Excuse me, can you provide me with a potion of {}?",
                CustomerMood::Excited => "Oh boy, I'm so excited for a potion of {}!",
                CustomerMood::Impatient => "Hurry up and give me a potion of {} already!",
                CustomerMood::Indifferent => "I guess I need a potion of {}.",
                CustomerMood::Grumpy => {
                    "What do you want? Give me a potion of {} and be quick about it!"
                }
            },
            CustomerType::Merchant => match self.mood {
                CustomerMood::Happy => "Greetings! Might you have a potion of {} for sale?",
                CustomerMood::Angry => "This better be the right potion of {}, or else!",
                CustomerMood::Anxious => "I need a potion of {}. Can you help me with that?",
                CustomerMood::Excited => "I've been waiting for this! Give me a potion of {}!",
                CustomerMood::Impatient => "I don't have all day, I need a potion of {} now!",
                CustomerMood::Indifferent => "I suppose I could use a potion of {}.",
                CustomerMood::Grumpy => {
                    "Get on with it, I need a potion of {} and I don't have all day!"
                }
            },
            CustomerType::Guard => match self.mood {
                CustomerMood::Happy => "Good day! I'm in need of a potion of {} to keep me alert.",
                CustomerMood::Angry => "Listen up! I need a potion of {} right now, got it?",
                CustomerMood::Anxious => "I'm feeling uneasy. Can I get a potion of {}, please?",
                CustomerMood::Excited => "Oh, I've been waiting for this! A potion of {} please!",
                CustomerMood::Impatient => {
                    "I don't have time for this. Just give me a potion of {}."
                }
                CustomerMood::Indifferent => "I need a potion of {}. That's all.",
                CustomerMood::Grumpy => {
                    "What do you want? Give me a potion of {} and be quick about it!"
                }
            },
        }
    }

    fn get_extra_text(&self, extra: Rule) -> String {
        match self.archetype {
            CustomerType::Adventurer => match extra {
                Rule::Effect(val) => format!("It also needs the added effect of {}", val),
                Rule::Tag(val) => format!("It must have a side effect to make it {}", val),
                Rule::NotEffect(val) => format!("It can't have {} as a side effect", val),
                Rule::NotTag(val) => format!(
                    "It really can't have any side effects that would make it {}",
                    val
                ),
            },
            CustomerType::Witch => match extra {
                Rule::Effect(val) => format!("It must have {} as an effect or it's a no-go", val),
                Rule::Tag(val) => format!("Make sure it's {} or there's no deal", val),
                Rule::NotEffect(val) => format!("No potion with {} as an effect, please", val),
                Rule::NotTag(val) => format!("I don't want any potions with {} side effects", val),
            },
            CustomerType::Wizard => match extra {
                Rule::Effect(val) => format!("Add {} to the potion or I'll find another shop", val),
                Rule::Tag(val) => format!("It must be {} or it's not worth my time", val),
                Rule::NotEffect(val) => {
                    format!("No potions with {} effect, I have enough of that", val)
                }
                Rule::NotTag(val) => {
                    format!("Keep {} away from the potion, it's a deal breaker", val)
                }
            },
            CustomerType::Alchemist => match extra {
                Rule::Effect(val) => format!("Don't forget the effect of {}", val),
                Rule::Tag(val) => format!(
                    "It should contain the element of {} for the desired effect",
                    val
                ),
                Rule::NotEffect(val) => {
                    format!("No {} effect please, not what I'm looking for", val)
                }
                Rule::NotTag(val) => format!("I don't want any side effects that have {}", val),
            },
            CustomerType::Noble => match extra {
                Rule::Effect(val) => format!("Add {} effect for the extra boost I need", val),
                Rule::Tag(val) => format!("I want it to be {} or I'm out of here", val),
                Rule::NotEffect(val) => format!("Avoid any {} effect, it's not for me", val),
                Rule::NotTag(val) => {
                    format!("I'm allergic to {} side effects, so none of that", val)
                }
            },
            CustomerType::Peasant => match extra {
                Rule::Effect(val) => format!("Can you add {} to make it more potent?", val),
                Rule::Tag(val) => format!("Make it {} or don't even bother", val),
                Rule::NotEffect(val) => format!("I don't want any {} effect, thanks", val),
                Rule::NotTag(val) => format!(
                    "I don't want to suffer from {} side effects, so avoid that",
                    val
                ),
            },
            CustomerType::Merchant => match extra {
                Rule::Effect(val) => format!("Add {} to make it more valuable", val),
                Rule::Tag(val) => format!("I want it to be {} or I'm not buying", val),
                Rule::NotEffect(val) => format!("No {} effect, it's not worth my money", val),
                Rule::NotTag(val) => format!(
                    "Avoid any {} side effects, I don't want to lose business over this",
                    val
                ),
            },
            CustomerType::Guard => match extra {
                Rule::Effect(val) => format!(
                    "Make sure it has {} effect, it's for a special mission",
                    val
                ),
                Rule::Tag(val) => format!("I need it to be {} for my duty", val),
                Rule::NotEffect(val) => format!("Avoid {} effect, it could jeopardize my job", val),
                Rule::NotTag(val) => format!(
                    "I don't want to suffer from {} side effects, it's not safe for my work",
                    val
                ),
            },
        }
    }

    fn get_water_text(&self) -> &'static str {
        match self.archetype {
            CustomerType::Adventurer => "Just a plain glass of water will do for now.",
            CustomerType::Witch => "I require some pure water for my potion. Do you have any?",
            CustomerType::Wizard => "I need some water to cast a spell. Can I have a glass?",
            CustomerType::Alchemist => {
                "I just need some pure water to facilitate a special reaction."
            }
            CustomerType::Noble => "I'll have some water, thank you.",
            CustomerType::Peasant => "Just water for me, please.",
            CustomerType::Merchant => "I'll take a glass of water, please.",
            CustomerType::Guard => "Just water, please. I'm on duty and need to stay hydrated.",
        }
    }
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum CustomerType {
    Adventurer,
    Witch,
    Wizard,
    Alchemist,
    Noble,
    Peasant,
    Merchant,
    Guard,
}

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum CustomerMood {
    Happy,
    Angry,
    Anxious,
    Excited,
    Impatient,
    Indifferent,
    Grumpy,
}

impl std::fmt::Display for CustomerMood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomerMood::Happy => f.write_str("a Happy"),
            CustomerMood::Angry => f.write_str("an Angry"),
            CustomerMood::Anxious => f.write_str("an Anxious"),
            CustomerMood::Excited => f.write_str("an Excited"),
            CustomerMood::Impatient => f.write_str("a very Impatient"),
            CustomerMood::Indifferent => f.write_str("an Indifferent"),
            CustomerMood::Grumpy => f.write_str("a Grumpy"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rule {
    Effect(PotionEffect),
    Tag(EffectTags),
    NotEffect(PotionEffect),
    NotTag(EffectTags),
}

#[test]
fn possible_potions() {
    use indexmap::IndexMap;
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("potions.txt")
        .unwrap();
    let mut map = IndexMap::new();
    let mut reverse_map: indexmap::IndexMap<PotionEffect, Vec<u8>> = indexmap::IndexMap::new();
    for i in 0..=255u8 {
        let mut effects = HashSet::new();
        for effect in PotionEffect::get_potion_effects(Tags(i)) {
            effects.insert(effect);
            reverse_map.entry(effect).or_insert(Vec::new()).push(i);
        }
        map.insert(i, effects);
    }
    let all_effect = PotionEffect::iter().collect::<HashSet<_>>();
    let mut have_no_pure = all_effect.clone();
    let mut cant_make = Vec::new();
    for effect in PotionEffect::iter() {
        match reverse_map.get(&effect) {
            None => cant_make.push(effect),
            Some(set) => {
                for val in set {
                    if map.get(val).unwrap().len() == 1 {
                        have_no_pure.remove(&effect);
                    }
                }
            }
        }
    }
    let _ = writeln!(&mut file, "Potions => Effect");

    // let potion_effect = std::fs::write("potion.effects", ron::ser::to_string_pretty(&map, ron::ser::PrettyConfig::default()).unwrap());
    for (key, val) in map.iter() {
        let _ = writeln!(&mut file, "{} = {:?}", key, val);
    }
    let _ = writeln!(&mut file, "How to get each Effect");
    for (key, val) in reverse_map.iter() {
        let _ = writeln!(&mut file, "{:?} = {:?}", key, val);
    }
    if cant_make.len() > 0 {
        let _ = writeln!(&mut file, "**Error** Can't Make = {:?}", cant_make);
    }
    if have_no_pure.len() > 0 {
        let _ = writeln!(&mut file, "**Warn** Have no pure = {:?}", have_no_pure);
    }

    let mut potion_tags = IndexMap::new();
    for (key, val) in map.iter() {
        let mut tags = HashSet::new();
        for effect in val {
            for tag in effect.get_tags() {
                tags.insert(*tag);
            }
        }
        potion_tags.insert(key, tags);
    }
    let _ = writeln!(&mut file, "Potions => Tags");
    for (key, val) in potion_tags.iter() {
        let _ = writeln!(&mut file, "{} = {:?}", key, val);
    }

    let mut cant_have_tags = IndexMap::new();
    let all_tags = EffectTags::iter().collect::<HashSet<_>>();
    for effect in PotionEffect::iter() {
        let mut sub_all = all_tags.clone();
        for tag in effect.get_tags() {
            sub_all.remove(tag);
        }
        cant_have_tags.insert(effect, sub_all);
    }

    for (key, value) in reverse_map.iter() {
        let current = cant_have_tags.get_mut(key).unwrap();
        for potion in value {
            let effects = map.get(potion).unwrap();
            for effect in effects {
                for tag in effect.get_tags() {
                    current.remove(tag);
                }
            }
        }
    }

    let _ = writeln!(&mut file, "Effect !=> Tags");
    for (key, val) in cant_have_tags.iter() {
        let _ = writeln!(&mut file, "{:?} = {:?}", key, val);
    }

    let mut cant_pair = IndexMap::new();
    for effect in PotionEffect::iter() {
        let mut sub_all = all_effect.clone();
        sub_all.remove(&effect);
        cant_pair.insert(effect, sub_all);
    }

    for (key, val) in reverse_map.iter() {
        let current = cant_pair.get_mut(key).unwrap();
        for potion in val {
            for effect in map.get(potion).unwrap() {
                current.remove(effect);
            }
        }
    }

    let _ = writeln!(&mut file, "Effect !=> Effect");
    for (key, val) in cant_pair.iter() {
        let _ = writeln!(
            &mut file,
            "{:?} => &{:?}",
            key,
            val.iter().collect::<Vec<_>>()
        );
    }
}
