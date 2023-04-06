use bevy::{prelude::*, ecs::system::EntityCommands};
use serde::{Serialize, Deserialize};
use crate::prelude::*;
use crate::map::MapCell;
mod Palm;
use bevy_asset_loader::prelude::*;

pub struct PlantPlugin;

impl Plugin for PlantPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((spawn_plant, update_growth, scail_with_groth.after(update_growth)).in_set(OnUpdate(GameState::Playing)))
        .register_type::<GrothProgress>()
        .register_type::<GrothStage>()
        .register_type::<PlantPart>();
        app.add_collection_to_loading_state::<_, Palm::PalmAssets>(GameState::Loading);
    }
}

impl PluginGroup for PlantPlugin {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        bevy::app::PluginGroupBuilder::start::<PlantPlugin>()
        .add(PlantPlugin)
        .add(Palm::PalmPlugin)
    }
}

trait PlantTrait: Send + Sync {
    fn spawn(cell: &MapCell, parent: EntityCommands); 
}

#[derive(Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum Plant {
    Palm
}

impl Plant {
    fn spawn(&self, cell: &MapCell, parent: EntityCommands) {
        match self {
            Self::Palm => Palm::PalmTree::spawn(cell, parent),
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

#[derive(Debug, Reflect, Serialize, Deserialize, Clone, Copy)]
pub enum PlantPart {
    Seed = 1,
    Leaf = 1 << 1,
    Root = 1 << 2,
    Stem = 1 << 3,
    Flower = 1 << 4,
    Fruit = 1 << 5,
    Bark = 1 << 6,
    Auxiliary = 1 << 7
}

fn spawn_plant(
    mut commands: Commands,
    plants: Query<(Entity, &MapCell, &Plant), Added<Plant>>
) {
    for (root, cell, plant) in &plants {
        plant.spawn(cell, commands.entity(root));
    }
}

#[derive(Debug, Reflect, Component)]
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

fn scail_with_groth(
    mut part: Query<(&mut Transform, &GrothProgress), With<ScailWithGroth>>,
) {
    for (mut scail, progress) in &mut part {
        scail.scale = Vec3::splat(progress.percent());
    }
}

fn update_growth(
    time: Res<Time>,
    mut parts: Query<&mut GrothProgress>,
) {
    for mut part in &mut parts {
        part.tick(time.delta())
    }
}