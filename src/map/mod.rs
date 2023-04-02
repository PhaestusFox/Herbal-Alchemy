use std::collections::HashMap;
use ids::Hex;
use bevy::prelude::*;
use bevy_wave_collapse::objects::Connection;
use rand::Rng;

use crate::{loading::{WaveMeshAssets, TextureAssets}, WaveBuilder, mesh::{Table, Island}, RVec3, WaveMesh, WaveObject, GameState};

use self::ids::CellId;

mod ids;
#[derive(Clone, Copy)]
pub enum HexNeighbour {
    One,
    Two,
    Three,
    For,
    Five,
    Six,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_asset::<WaveMesh>();
        app.add_asset_loader(bevy_wave_collapse::prelude::WaveMeshObjLoader::<crate::FixedPoint, crate::mesh::MeshTextureUVS>::default());
        app.add_system(spwan_test_table.in_schedule(OnEnter(GameState::Playing)));
    }
}

#[derive(Resource)]
struct TestTable(Entity, WaveObject);

fn spwan_test_table(
    mut commands: Commands,
    meshs: Res<WaveMeshAssets>,
    texture: Res<TextureAssets>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bevy_meshs: ResMut<Assets<Mesh>>,
    wave_meshs: Res<Assets<WaveMesh>>,
) {
    let mut meshes: HashMap<Connection, Handle<WaveMesh>> = HashMap::new();
    meshes.insert(Connection::new("Table"), meshs.empty_table.clone());
    meshes.insert(Connection::new("Id"), meshs.empty_pot.clone());
    meshes.insert(Connection::new("Island"), meshs.empty_island.clone());
    let table = Table::new(meshes.clone());
    let island = Island::new(meshes);

    for id in ids::HexRangeIterator::<CellId>::new(1) {
        let mut mesh_bulder = WaveBuilder::new();
        let mesh = if rand::thread_rng().gen_bool(0.5) {
            &table
        } else {
            &island
        };
        if let Err(e) = mesh.build(RVec3::default(), &wave_meshs, &mut mesh_bulder, &(), 0) {
            error!("{}", e);
        }
        let mesh = mesh_bulder.extract_mesh(bevy::render::render_resource::PrimitiveTopology::TriangleList);
        let id = commands.spawn(PbrBundle {
            mesh: bevy_meshs.add(mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture.wave_mesh_texture.clone()),
                ..Default::default()
            }),
            transform: Transform::from_translation(id.xyz(0.) * 2.),
            ..Default::default()
        });
    }

}