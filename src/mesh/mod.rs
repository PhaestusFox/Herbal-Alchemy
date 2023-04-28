use std::{collections::HashMap, hash::Hash};

use crate::prelude::*;
use bevy::prelude::*;
use bevy_wave_collapse::{objects::hexs_map::HexTrig, prelude::*};
use strum::IntoEnumIterator;
use strum_macros::EnumString;

pub type WaveObject = bevy_wave_collapse::objects::WaveObject<FixedPoint, MeshTextureUVS, WaveData>;
pub type WaveMesh = bevy_wave_collapse::prelude::WaveMesh<FixedPoint, MeshTextureUVS>;
pub type RVec3 = bevy_wave_collapse::prelude::RVec3<FixedPoint>;
pub type WaveBuilder = bevy_wave_collapse::prelude::WaveBuilder<FixedPoint, MeshTextureUVS>;
pub type WaveData = [MapCell; 6];

mod islands;
use islands::DynamicIsland;

use self::islands::StaticIsland;
pub struct MeshPlugin;

const NAMES: [&'static str; 2] = [
    "Sand",
    "Water",
];

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
        });
        IslandObjects(data)
    }
}

// fn build_map(
//     mut commands: Commands,
//     objs: Res<Assets<WaveObject>>,
//     wave_mesh: Res<Assets<WaveMesh>>,
//     mut bevy_mesh: ResMut<Assets<Mesh>>,
// ) {
//     let mut rng = rand::thread_rng();
//     let sand = objs.get(&Handle::weak(MapCell::Sand.to_handle_id())).unwrap();
//     let water = objs.get(&Handle::weak(MapCell::Water.to_handle_id())).unwrap();
//     let mut temp_map = HashMap::new();
//     for id in crate::map::ids::HexRangeIterator::new(2) {
//         temp_map.insert(id, if rng.gen_bool(0.33) {MapCell::Water} else {MapCell::Sand});
//     }
//     temp_map.insert(CellId::new(0, 0), MapCell::Sand);
//     for id in crate::map::ids::HexRingIterator::new(3) {
//         temp_map.insert(id, MapCell::Water);
//     }

//     commands.spawn((Name::new("Map"), SpatialBundle::INHERITED_IDENTITY)).with_children(|commands| {

//     for (id, cell) in temp_map.iter() {
//         let neighbours = id.neighbours().map(|id| temp_map.get(&id).copied().unwrap_or(MapCell::Water));
//         let mut mesh = WaveBuilder::new();
//         info!("{} = {:?}", id, neighbours);
//         if let Err(e) = match cell {
//             MapCell::Water => {
//                 water.build(RVec3::default(), &wave_mesh, &mut mesh, &neighbours)
//             },
//             MapCell::Sand => {
//                 sand.build(RVec3::default(), &wave_mesh, &mut mesh, &neighbours)
//             },
//             #[allow(unreachable_patterns)]
//             e => {error!("MapCell::{:?} does not have a wave obj", e); continue;},
//         } {error!("{}", e)};
//         let mesh = bevy_mesh.add(mesh.extract_mesh(bevy::render::render_resource::PrimitiveTopology::TriangleList));
//         commands.spawn((*id, MaterialMeshBundle::<CustomMaterial> {
//             mesh,
//             material: Handle::weak(crate::utils::ConstHandles::WaveMaterial.into()),
//             transform: Transform::from_translation(id.xyz(0.) * 2.),
//             ..Default::default()
//         }));
//     }
//     });
// }
