use crate::{inventory::SelectedSlot, crafting::tags::TagNames};
use crate::map::MapCell;
use crate::prelude::*;
use bevy::{ecs::system::EntityCommands, prelude::*};
use serde::{Deserialize, Serialize};
mod palm;

pub struct PlantPlugin;

impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            (
                spawn_plant,
                update_growth,
                scail_with_groth.after(update_growth),
            )
                .in_set(OnUpdate(GameState::Playing)),
        )
        .register_type::<GrothProgress>()
        .register_type::<GrothStage>()
        .register_type::<PlantPart>()
        .register_type::<Plant>()
        .add_system(plant_plant.in_set(OnUpdate(Tool::Trowl)));
    }
}

impl PluginGroup for PlantPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<PlantPlugin>()
            .add(PlantPlugin)
            .add(palm::PalmPlugin)
    }
}

trait PlantTrait: Send + Sync {
    fn spawn(cell: &MapCell, parent: EntityCommands);
}

#[derive(
    Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, FromReflect,
)]
#[reflect_value()]
pub enum Plant {
    Palm,
}

impl Ingredient for Plant {
    fn get_tags(&self) -> Tags {
        match self {
            Plant::Palm => Tags::new([TagNames::Tropical])
        }
    }
}

impl Plant {
    fn spawn(&self, cell: &MapCell, parent: EntityCommands) {
        match self {
            Self::Palm => palm::PalmTree::spawn(cell, parent),
        }
    }

    pub fn tool_tip_text(&self, part: PlantPart) -> String {
        match self {
            Plant::Palm => palm::PalmTree::tool_tip_text(part),
        }
    }
}

#[derive(Component, Debug, Reflect)]
enum GrothStage {
    Seed,
    Sprout,
    Small,
    Full,
    Budding,
    Flower,
    Fruiting,
    Dead,
}

#[derive(Debug, Reflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, FromReflect)]
#[reflect_value()]
pub enum PlantPart {
    Seed = 2,
    Leaf = 64,
    Root = 8,
    Stem = 16,
    Flower = 1 << 7,
    Fruit = 20,
    Bark = 18,
    Auxiliary = 1 << 5,
}

impl Ingredient for PlantPart {
    fn get_tags(&self) -> Tags {
        use TagNames::*;
        match self {
            PlantPart::Seed => Tags::new([Life]),
            PlantPart::Leaf => Tags::new([Air]),
            PlantPart::Root => Tags::new([Water, Earth]),
            PlantPart::Stem => Tags::new([Fibrous]),
            PlantPart::Flower => todo!(),
            PlantPart::Fruit => Tags::new([Water, Time]),
            PlantPart::Bark => Tags::new([Fire]),
            PlantPart::Auxiliary => todo!(),
        }
    }
}

fn spawn_plant(mut commands: Commands, plants: Query<(Entity, &MapCell, &Plant), Added<Plant>>) {
    for (root, cell, plant) in &plants {
        plant.spawn(cell, commands.entity(root));
    }
}

#[derive(Debug, Reflect, Component, Clone)]
struct GrothProgress(Timer);

impl GrothProgress {
    fn new(time: f32) -> GrothProgress {
        GrothProgress(Timer::from_seconds(time, TimerMode::Once))
    }
    fn percent(&self) -> f32 {
        self.0.percent()
    }
    fn finished(&self) -> bool {
        self.0.finished()
    }
    fn reset(&mut self) {
        self.0.reset()
    }
    fn tick(&mut self, delta: std::time::Duration) {
        self.0.tick(delta);
    }
}

#[derive(Component)]
struct ScailWithGroth;

fn scail_with_groth(mut part: Query<(&mut Transform, &GrothProgress), With<ScailWithGroth>>) {
    for (mut scail, progress) in &mut part {
        scail.scale = Vec3::splat(progress.percent());
    }
}

fn update_growth(time: Res<Time>, mut parts: Query<&mut GrothProgress>) {
    for mut part in &mut parts {
        part.tick(time.delta())
    }
}

fn plant_plant(
    mut commands: Commands,
    cells: Query<(Entity, &Interaction), (Without<Plant>, Changed<Interaction>, With<MapCell>)>,
    mut seed: Query<(&Item, &Slot), With<SelectedSlot>>,
    mut events: EventWriter<InventoryEvent>,
) {
    for (e, interaction) in &cells {
        if let Interaction::Clicked = interaction {
            if let Ok((seed, slot)) = seed.get_single_mut() {
                if let Item::Ingredient(plant, PlantPart::Seed) = *seed {
                    commands.entity(e).insert(plant);
                    events.send(InventoryEvent::RemoveItem(*slot));
                }
            }
        }
    }
}
