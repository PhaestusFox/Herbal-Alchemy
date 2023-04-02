use std::str::FromStr;

use bevy::prelude::{Handle, Assets, warn};
use std::collections::HashMap;
use bevy_wave_collapse::{objects::Connection, prelude::{RVec3, BakeError}};

use crate::{WaveObject, WaveMesh, WaveBuilder, FixedPoint};

#[derive(Debug, Hash, PartialEq, Clone, Copy, Default)]
pub enum MeshTextureUVS {
    Legs = 0,
    #[default]
    Pot = 1,
    Wood = 2,
    Sand = 7,
    Water = 8,
}

impl FromStr for MeshTextureUVS {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        warn!("{}", s);
        match &s.to_lowercase()[..] {
            "wood" => Ok(MeshTextureUVS::Wood),
            "pot" => Ok(MeshTextureUVS::Pot),
            "legs" => Ok(MeshTextureUVS::Legs),
            "sand" => Ok(MeshTextureUVS::Sand),
            "water" => Ok(MeshTextureUVS::Water),
            _ => Err(format!("'{}' is not a configured texture", s)),
        }
    }
}

impl bevy_wave_collapse::vertex::VertexUV for MeshTextureUVS {
    fn to_f32x2(&self) -> [f32; 2] {
        (*self as u8).to_f32x2()
    }
}

pub struct Table;

impl Table {
    pub fn new(meshes: HashMap<Connection, Handle<WaveMesh>>) -> WaveObject {
        WaveObject {
            meshes,
            build_fn: Self::build,
            can_connect_fn: |c| {false}
        }
    }
    fn build(obj: &WaveObject, offset: RVec3<FixedPoint>, assets: &Assets<WaveMesh>, builder: &mut WaveBuilder, data: &(), seed: u64) -> Result<(), BakeError> {
        let wave = assets.get(obj.meshes.get(&Connection::new("Table")).unwrap()).unwrap();
        builder.bake(offset, wave)
    }
}

pub struct Island;

impl Island {
    pub fn new(meshes: HashMap<Connection, Handle<WaveMesh>>) -> WaveObject {
        WaveObject {
            meshes,
            build_fn: Self::build,
            can_connect_fn: |c| {false}
        }
    }
    fn build(obj: &WaveObject, offset: RVec3<FixedPoint>, assets: &Assets<WaveMesh>, builder: &mut WaveBuilder, data: &(), seed: u64) -> Result<(), BakeError> {
        let wave = assets.get(obj.meshes.get(&Connection::new("Island")).unwrap()).unwrap();
        builder.bake(offset, wave)
    }
}