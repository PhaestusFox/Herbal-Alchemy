use crate::prelude::*;
use bevy::{asset::HandleId, prelude::*};
use bevy_mod_picking::PickableBundle;
use rand::Rng;

use crate::{GameState, WaveMesh, WaveObject};

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
        app.add_asset::<WaveObject>();
        app.add_asset_loader(bevy_wave_collapse::prelude::WaveMeshObjLoader::<
            crate::FixedPoint,
            crate::mesh::MeshTextureUVS,
        >::default());
        app.add_system(make_pot_assets.in_schedule(OnExit(GameState::Loading)));
        app.add_systems((update_cell_transform, update_mesh).in_set(OnUpdate(GameState::Playing)));
        app.register_type::<MapCell>();
    }
}

fn make_pot_assets(mut commands: Commands) {
    for id in ids::HexRangeIterator::<CellId>::new(1) {
        let cell = MapCell::Island;
        let mut c = commands.spawn((
            PbrBundle {
                material: Handle::weak(ConstHandles::WaveMaterial.into()),
                ..Default::default()
            },
            cell,
            id,
            PickableBundle::default(),
        ));
        if id == CellId::new(0, 0) {
            c.insert(crate::plants::Plant::Palm);
        }
    }
}

fn update_mesh(mut cells: Query<(&mut Handle<Mesh>, &MapCell), Changed<MapCell>>) {
    for (mut mesh, cell_type) in &mut cells {
        *mesh = Handle::weak((*cell_type).into());
    }
}

fn update_cell_transform(
    mut cells: Query<(&CellId, &mut Transform), (With<MapCell>, Changed<CellId>)>,
) {
    for (cell, mut pos) in &mut cells {
        pos.translation = cell.xyz(0.) * 2.;
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect)]
pub enum MapCell {
    Island,
    Table,
}

use bevy::reflect::TypeUuid;

impl TypeUuid for MapCell {
    const TYPE_UUID: uuid::Uuid = uuid::uuid!("ae52aa38-f993-481e-b9b5-554d4ee2da22");
}

impl Into<HandleId> for MapCell {
    fn into(self) -> HandleId {
        format!("objs/Pots.obj#{:?}", self).into()
    }
}

impl MapCell {
    pub fn seed_offset(&self) -> Vec3 {
        match self {
            MapCell::Island => Vec3 {
                x: 0.,
                y: 0.2,
                z: 0.0,
            },
            MapCell::Table => Vec3 {
                x: 0.,
                y: 0.275,
                z: 0.0,
            },
        }
    }
}
