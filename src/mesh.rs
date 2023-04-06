use bevy::prelude::{Handle, Assets};
use std::collections::HashMap;
use bevy_wave_collapse::{objects::Connection, prelude::{RVec3, BakeError}};

use crate::{WaveObject, WaveMesh, WaveBuilder, FixedPoint};
use strum_macros::EnumString;

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
            can_connect_fn: |_| {false}
        }
    }
    fn build(obj: &WaveObject, offset: RVec3<FixedPoint>, assets: &Assets<WaveMesh>, builder: &mut WaveBuilder, _: &u64, _: u64) -> Result<(), BakeError> {
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
            can_connect_fn: |_| {false}
        }
    }
    fn build(obj: &WaveObject, offset: RVec3<FixedPoint>, assets: &Assets<WaveMesh>, builder: &mut WaveBuilder, _: &u64, _: u64) -> Result<(), BakeError> {
        let wave = assets.get(obj.meshes.get(&Connection::new("Island")).unwrap()).unwrap();
        builder.bake(offset, wave)
    }
}