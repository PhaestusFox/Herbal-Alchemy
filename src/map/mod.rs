use std::collections::HashMap;

use crate::{crafting::potions::PotionEffect, inventory::SelectedSlot, plants::Plant, prelude::*, mesh::{WaveData, EMPTY_PALATE, DEEP_PALATE, FRESH_PALATE}};
use bevy::{asset::HandleId, prelude::*, utils::HashSet};
use bevy_mod_picking::PickableBundle;
use serde::{Deserialize, Serialize};
use uuid::uuid;

use self::ids::{CellId, Hex, WithOffset};

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
        app.register_type::<CellId>()
            .add_systems(
                (update_cell_transform, update_mesh_system).in_set(OnUpdate(GameState::Playing)),
            )
            .register_type::<MapCell>()
            .add_system(spawn_map.in_schedule(OnExit(GameState::Loading)))
            .add_system(make_island.in_set(OnUpdate(Tool::Hand)))
            .add_system(save_map.in_set(OnUpdate(GameState::Playing)))
            .add_system(dynamic_update.in_schedule(CoreSchedule::FixedUpdate).run_if(resource_exists::<NeedMapUpdate>()))
            .insert_resource(FixedTime::new_from_secs(1.));
    }
}

const MAPSIZE: u32 = 3;

fn spawn_map(mut commands: Commands, pkv: Res<bevy_pkv::PkvStore>) {
    //todo add seed to map
    let old_map = match pkv.get::<HashMap<CellId, (MapCell, Option<Plant>)>>("Map") {
        Ok(mut val) => {
            let mut need_tree = true;
            val.values()
                .map(|(_, tree)| {
                    if tree.is_some() {
                        need_tree = false;
                    }
                })
                .count();
            if need_tree {
                val.insert(CellId::ZERO, (MapCell::Sand, Some(Plant::Palm)));
            };
            val
        }
        Err(e) => {
            error!("{:?}", e);
            let mut new_map = ids::HexSpiralIterator::new(MAPSIZE)
                .map(|id| (id, (MapCell::Water, None)))
                .collect::<HashMap<CellId, (MapCell, Option<Plant>)>>();
            new_map.insert(CellId::ZERO, (MapCell::Sand, Some(Plant::Palm)));
            new_map
        }
    };

    let mut map = HashMap::new();
    let root = commands
        .spawn((SpatialBundle::INHERITED_IDENTITY, Name::new("Map")))
        .id();

    //todo add setting for world size
    for id in ids::HexSpiralIterator::new(MAPSIZE) {
        let (cell, plant) = if let Some(val) = old_map.get(&id) {
            *val
        } else {
            (MapCell::Water, None)
        };
        let cell = if id == CellId::ZERO {
            MapCell::FreshWater
        } else {
            cell
        };
        let mut entity = commands.spawn((
            id,
            cell,
            PickableBundle::default(),
            SpatialBundle::INHERITED_IDENTITY,
            Handle::<CustomMaterial>::weak(ConstHandles::WaveMaterial.into()),
            Handle::<Mesh>::default(),
            Name::new(format!("Cell {}", id)),
        ));
        entity.set_parent(root);
        if let Some(plant) = plant {
            entity.insert(plant);
        }
        map.insert(id, entity.id());
    }
    for id in ids::HexRingIterator::new(MAPSIZE + 1) {
        let entity = commands.spawn((
            id,
            MapCell::DeepWater,
            PickableBundle::default(),
            SpatialBundle::INHERITED_IDENTITY,
            Handle::<CustomMaterial>::weak(ConstHandles::WaveMaterial.into()),
            Handle::<Mesh>::default(),
            Name::new(format!("Cell {}", id)),
        ))
        .set_parent(root).id();
        map.insert(id, entity);
    }
    
    commands.insert_resource(MapData { map });
}

fn update_mesh(
    id: CellId,
    map: &MapData,
    cells: &mut Query<(&mut Handle<Mesh>, &MapCell)>,
    objs: &Assets<WaveObject>,
    bevy_meshs: &mut Assets<Mesh>,
    wave_meshs: &Assets<WaveMesh>,
) {
    let neighbours = id.neighbours().map(|id| {
        if let Some(entity) = map.get(&id) {
            cells.get(entity).map(|v| *v.1).unwrap_or_default()
        } else {
            MapCell::DeepWater
        }
    });

    let neighbours = WaveData {
        neighbours,
        palate: match if let Some(entity) = map.get(&id) {
            cells.get(entity).map(|v| *v.1).unwrap_or_default()
        } else {
            MapCell::DeepWater
        } {
            MapCell::Water |
            MapCell::Sand => &EMPTY_PALATE,
            MapCell::DeepWater => &DEEP_PALATE,
            MapCell::FreshWater => &FRESH_PALATE
        }
    };
    let (mut mesh, cell_type) = if let Some(entity) = map.get(&id) {
        match cells.get_mut(entity) {
            Ok(v) => v,
            Err(_) => return,
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
    let mut updated = HashSet::new();
    for id in &to_update {
        updated.get_or_insert_with(id, |neighbour| {
            update_mesh(
                *neighbour,
                &map,
                &mut cells,
                &wave_objs,
                &mut bevy_meshs,
                &wave_meshs,
            );
            *neighbour
        });
        for neighbour in id.neighbours() {
            updated.get_or_insert_with(&neighbour, |neighbour| {
                update_mesh(
                    *neighbour,
                    &map,
                    &mut cells,
                    &wave_objs,
                    &mut bevy_meshs,
                    &wave_meshs,
                );
                *neighbour
            });
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

#[derive(
    Component, Clone, Copy, Debug, Reflect, PartialEq, Eq, Default, Serialize, Deserialize,
)]
#[reflect(Component)]
pub enum MapCell {
    Water,
    Sand,
    #[default]
    DeepWater,
    FreshWater,
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
            MapCell::FreshWater => Vec3 {
                x: 0.,
                y: 0.1,
                z: 0.0,
            },
            MapCell::DeepWater => Vec3 {
                x: 0.,
                y: 0.1,
                z: 0.0,
            },
        }
    }
}

#[derive(Resource)]
pub struct NeedMapUpdate;

fn make_island(
    mut commands: Commands,
    current: Query<(&Item, &Slot), With<SelectedSlot>>,
    mut click_cell: Query<(&mut MapCell, &Interaction), Changed<Interaction>>,
    mut events: EventWriter<InventoryEvent>,
    mut msgs: EventWriter<crate::msg_event::PlayerMessage>,
) {
    let Ok((item, slot)) = current.get_single() else {return;};
    let potion = if let Item::Potion(id) = item {
        PotionEffect::get_potion_effects(*id)
    } else {
        return;
    };
    for (mut cell, interaction) in &mut click_cell {
        if *interaction != Interaction::Clicked {
            continue;
        }
        if potion.contains(&PotionEffect::TidalWave) && *cell != MapCell::Water {
            *cell = MapCell::Water;
            events.send(InventoryEvent::RemoveItem(*slot));
            commands.insert_resource(NeedMapUpdate);
            return;
        };
        if potion.contains(&PotionEffect::IslandOasis) && *cell != MapCell::Sand {
            *cell = MapCell::Sand;
            events.send(InventoryEvent::RemoveItem(*slot));
            commands.insert_resource(NeedMapUpdate);
            return;
        };
        msgs.send(PlayerMessage::warn(
            "You don't think this potion will do anything if you toss it there".to_string(),
        ));
    }
}

fn save_map(
    map: Res<MapData>,
    mut pkv: ResMut<bevy_pkv::PkvStore>,
    query: Query<(&MapCell, Option<&Plant>)>,
    time: Res<Time>,
    mut next_save: Local<SaveTime>,
) {
    next_save.0.tick(time.delta());
    if !next_save.0.finished() {
        return;
    }
    let mut new_map = HashMap::new();
    for (id, entity) in map.map.iter() {
        if id.distance(CellId::ZERO) > MAPSIZE {continue;}
        if let Ok(cell) = query.get(*entity) {
            new_map.insert(*id, (*cell.0, cell.1.cloned()));
        }
    }
    if let Err(e) = pkv.set("Map", &new_map) {
        error!("{:?}", e);
    };
}

fn dynamic_update(
    mut commands: Commands,
    map: Res<MapData>,
    mut real: Query<(Entity, &mut MapCell, &CellId)>,
) {
    let mut set_water = Vec::new();
    let mut set_fresh = Vec::new();
    'next: for (entity, cell, id) in &real {
        match cell {
            MapCell::Sand |
            MapCell::DeepWater => continue,
            MapCell::Water |
            MapCell::FreshWater => {
                for ring in 1..=MAPSIZE*2 + 1 {
                    let mut found_out = false;
                    for around in ids::HexRingIterator::new(ring).with_offset(*id) {
                        if let Some(other) = map.get(&around) {
                            let Ok((_, other, _)) = real.get(other) else {error!("Cell in map need to have vaild enity {:?}", other); continue;};
                            match other {
                                MapCell::FreshWater |
                                MapCell::Water => found_out = true,
                                MapCell::Sand => {},
                                MapCell::DeepWater => {set_water.push(entity); continue 'next;},
                            }
                        }
                    }
                    if !found_out {
                        set_fresh.push(entity);
                        break;
                    }
                }
            }
        }
    }
    for water in set_water {
        if let Ok((_, mut cell, _)) = real.get_mut(water) {
            *cell = MapCell::Water;
        }
    }
    for fresh in set_fresh {
        if let Ok((_, mut cell, _)) = real.get_mut(fresh) {
            *cell = MapCell::FreshWater;
        }
    }
    commands.remove_resource::<NeedMapUpdate>();
}

struct SaveTime(Timer);
impl Default for SaveTime {
    fn default() -> Self {
        SaveTime(Timer::from_seconds(60., TimerMode::Repeating))
    }
}
