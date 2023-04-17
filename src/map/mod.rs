use std::collections::HashMap;

use crate::{prelude::*, plants::Plant, inventory::SelectedSlot, crafting::potions::PotionEffect};
use bevy::{asset::HandleId, prelude::*};
use bevy_mod_picking::PickableBundle;
use uuid::uuid;

use self::ids::{CellId, Hex};

#[derive(Resource)]
pub struct MapData {
    map: HashMap<CellId, Entity>,
}

impl MapData {
    fn get(&self, id: &CellId) -> Option<Entity> {
        self.map.get(id).cloned()
    }
}

pub mod ids;
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
        app.add_asset::<WaveMesh>()
            .add_asset::<WaveObject>()
            .add_asset_loader(bevy_wave_collapse::prelude::WaveMeshObjLoader::<
                FixedPoint,
                crate::mesh::MeshTextureUVS,
            >::default())
            .register_type::<CellId>()
            .add_systems((update_cell_transform, update_mesh_system).in_set(OnUpdate(GameState::Playing)))
            .register_type::<MapCell>()
            .add_system(spawn_map.in_schedule(OnExit(GameState::Loading)))
            .add_system(make_island.in_set(OnUpdate(Tool::Hand)));
    }
}

fn spawn_map(mut commands: Commands) {
    //todo add seed to map
    // let mut rng = rand::thread_rng();
    let mut map = HashMap::new();
    let root = commands.spawn((SpatialBundle::INHERITED_IDENTITY, Name::new("Map"))).id();
    let cell = CellId::new(0,0);
    let entity = commands
            .spawn((
                cell,
                MapCell::Sand,
                PickableBundle::default(),
                SpatialBundle::INHERITED_IDENTITY,
                Handle::<CustomMaterial>::weak(ConstHandles::WaveMaterial.into()),
                Handle::<Mesh>::default(),
                Name::new(format!("Cell{}", cell)),
                Plant::Palm,
            ))
            .set_parent(root)
            .id();
        map.insert(cell, entity);

    //todo add setting for world size
    for cell in ids::HexSpiralIterator::new(2).skip(1) {
        let entity = commands
            .spawn((
                cell,
                MapCell::Water,
                PickableBundle::default(),
                SpatialBundle::INHERITED_IDENTITY,
                Handle::<CustomMaterial>::weak(ConstHandles::WaveMaterial.into()),
                Handle::<Mesh>::default(),
                Name::new(format!("Cell{}", cell)),
            ))
            .set_parent(root)
            .id();
        map.insert(cell, entity);
    }
    commands.insert_resource(MapData { map });
}

fn update_mesh(
    id: CellId,
    map: &MapData, 
    cells: &mut Query<(&mut Handle<Mesh>, &MapCell)>,
    objs: &Assets<WaveObject>,
    bevy_meshs: &mut Assets<Mesh>,
    wave_meshs: &Assets<WaveMesh>
) {
    let neighbours = id.neighbours().map(|id| {
        if let Some(entity) = map.get(&id) {
            cells.get(entity).map(|v| *v.1).unwrap_or(MapCell::Water)
        } else {
            MapCell::Water
        }
    });
    let (mut mesh, cell_type) = if let Some(entity) = map.get(&id) {
        match cells.get_mut(entity) {
            Ok(v) => v,
            Err(_) => return
        }
    } else {
        return;
    };
    let mut main_mesh = WaveBuilder::new();
    let Some(obj) = objs.get(&Handle::weak(cell_type.to_handle_id())) else {error!("WaveObj for {:?} not loaded", cell_type); return;};
    if let Err(e) = obj.build(RVec3::default(), wave_meshs, &mut main_mesh, &neighbours) {
        error!("{}", e);
    }
    *mesh = bevy_meshs.add(
        main_mesh.extract_mesh(bevy::render::render_resource::PrimitiveTopology::TriangleList),
    );
}

fn update_mesh_system(
    to_update: Query<&CellId, Changed<MapCell>>,
    mut cells: Query<(&mut Handle<Mesh>, &MapCell)>,
    wave_objs: Res<Assets<WaveObject>>,
    mut bevy_meshs: ResMut<Assets<Mesh>>,
    wave_meshs: Res<Assets<WaveMesh>>,
    map: Res<MapData>,
) {
    for id in &to_update {
        update_mesh(*id, &map, &mut cells, &wave_objs, &mut bevy_meshs, &wave_meshs);
        for neighbour in id.neighbours() {
            update_mesh(neighbour, &map, &mut cells, &wave_objs, &mut bevy_meshs, &wave_meshs);
        }
    }
}

fn update_cell_transform(
    mut cells: Query<(&CellId, &mut Transform), (With<MapCell>, Changed<CellId>)>,
) {
    for (cell, mut pos) in &mut cells {
        pos.translation = cell.xyz(0.) * 2.;
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq, Eq)]
pub enum MapCell {
    Water,
    Sand,
}

impl MapCell {
    pub const fn to_handle_id(self) -> HandleId {
        HandleId::Id(uuid!("40fc8351-b595-4975-be40-77b35dc302fa"), self as u64)
    }
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
            MapCell::Water => Vec3 {
                x: 0.,
                y: 0.1,
                z: 0.0,
            },
            MapCell::Sand => Vec3 {
                x: 0.,
                y: 0.2,
                z: 0.0,
            },
        }
    }
}

fn make_island(
    current: Query<(&Item, &Slot), With<SelectedSlot>>,
    mut click_cell: Query<(&mut MapCell, &Interaction), Changed<Interaction>>,
    mut events: EventWriter<InventoryEvent>,
    mut msgs: EventWriter<crate::msg_event::PlayerMessage>
) {
    let Ok((item, slot)) = current.get_single() else {return;};
    let potion = if let Item::Potion(id) = item {
        PotionEffect::get_potion_effects(*id)
    } else {return;};
    for (mut cell, interaction) in &mut click_cell {
        if *interaction != Interaction::Clicked {continue;}
        if potion.contains(&PotionEffect::TidalWave) && *cell != MapCell::Water {*cell = MapCell::Water; events.send(InventoryEvent::RemoveItem(*slot)); return;};
        if potion.contains(&PotionEffect::IslandOasis) && *cell != MapCell::Sand {*cell = MapCell::Sand; events.send(InventoryEvent::RemoveItem(*slot)); return;};
        msgs.send(PlayerMessage::warn(
            "You don't think this potion will do anything if you toss it there".to_string(),
            Color::YELLOW));
    }
}