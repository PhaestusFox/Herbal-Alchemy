use std::{collections::HashMap, hash::Hash};

use crate::prelude::*;
use bevy::prelude::*;
use bevy_wave_collapse::{objects::hexs_map::HexTrig, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::EnumString;
use lazy_static::lazy_static;

pub type WaveObject = bevy_wave_collapse::objects::WaveObject<FixedPoint, MeshTextureUVS, WaveData>;
pub type WaveMesh = bevy_wave_collapse::prelude::WaveMesh<FixedPoint, MeshTextureUVS>;
pub type RVec3 = bevy_wave_collapse::prelude::RVec3<FixedPoint>;
pub type WaveBuilder = bevy_wave_collapse::prelude::WaveBuilder<FixedPoint, MeshTextureUVS>;
pub struct WaveData {
    pub neighbours: [MapCell; 6],
    pub palate: &'static HashMap<MeshTextureUVS, MeshTextureUVS>
}

lazy_static! {
    pub static ref EMPTY_PALATE: HashMap<MeshTextureUVS, MeshTextureUVS> = HashMap::new();
    pub static ref DEEP_PALATE: HashMap<MeshTextureUVS, MeshTextureUVS> = [(MeshTextureUVS::Water, MeshTextureUVS::DeepWater)].into_iter().collect();
    pub static ref FRESH_PALATE: HashMap<MeshTextureUVS, MeshTextureUVS> = [(MeshTextureUVS::Water, MeshTextureUVS::FreshWater)].into_iter().collect();
}

mod islands;
use islands::DynamicIsland;

use self::islands::StaticIsland;
pub struct MeshPlugin;

const NAMES: [&'static str; 2] = ["Sand", "Water"];

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<WaveMesh>()
            .add_asset::<WaveObject>()
            .init_asset_loader::<WaveMeshObjLoader<FixedPoint, MeshTextureUVS>>()
            .init_resource::<IslandObjects>();
    }
}

#[derive(Debug, Hash, PartialEq, Clone, Copy, Default, EnumString, Eq)]
pub enum MeshTextureUVS {
    Legs = 0,
    #[default]
    Pot = 1,
    Wood = 2,
    Sand = 7,
    Water = 8,
    PalmTrunk = 9,
    PalmLeaf = 10,
    PalmNut = 11,
    PalmSeed = 12,
    Red = 13,
    Rock = 14,
    DeepWater = 15,
    FreshWater = 16,
}

impl bevy_wave_collapse::vertex::VertexUV for MeshTextureUVS {
    fn to_f32x2(&self) -> [f32; 2] {
        (*self as u8).to_f32x2()
    }
}

#[derive(Resource)]
struct IslandObjects(Vec<Handle<WaveObject>>);

impl FromWorld for IslandObjects {
    fn from_world(world: &mut World) -> Self {
        let mut data = Vec::new();
        world.resource_scope(|world, mut objs: Mut<Assets<WaveObject>>| {
            let asset_server = world.resource::<AssetServer>();
            let obj = objs.set(
                MapCell::Sand.to_handle_id(),
                DynamicIsland::new::<0>(asset_server, "objs/sand.wfo"),
            );
            data.push(obj);
            let obj = objs.set(
                MapCell::Water.to_handle_id(),
                StaticIsland::new::<1>(asset_server, "objs/water.wfo"),
            );
            data.push(obj);
            let obj = objs.set(
                MapCell::FreshWater.to_handle_id(),
                StaticIsland::new::<1>(asset_server, "objs/water.wfo"),
            );
            data.push(obj);
            let obj = objs.set(
                MapCell::DeepWater.to_handle_id(),
                StaticIsland::new::<1>(asset_server, "objs/water.wfo"),
            );
            data.push(obj);
        });
        IslandObjects(data)
    }
}