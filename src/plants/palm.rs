use super::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::PickableBundle;
use rand::Rng;

#[derive(Component, Default, Reflect)]
pub struct PalmTree;

impl PalmTree {
    fn leaf_offset() -> Vec3 {
        Vec3 {
            x: 0.,
            y: 0.9,
            z: -0.32,
        }
    }
    fn coconut_offset(id: u8) -> Vec3 {
        match id {
            0 => Vec3 {
                x: 0.1,
                y: 0.8,
                z: -0.32,
            },
            1 => Vec3 {
                x: -0.1,
                y: 0.8,
                z: -0.32,
            },
            _ => Vec3 {
                x: -0.05,
                y: 0.8,
                z: -0.16,
            },
        }
    }
    pub fn tool_tip_text(part: PlantPart) -> String {
        match part {
            PlantPart::Seed => format!(
                "A coconut thats still green; Maybe You Could Plant it ;P:"
            ),
            PlantPart::Leaf => format!("A Palm Fron:"),
            PlantPart::Root => format!("A Coconut Root:"),
            PlantPart::Stem => format!("A Log of a Palm Tree:"),
            PlantPart::Fruit => format!("A Ripe coconut its to old to plant:"),
            PlantPart::Bark => format!("A Fibers:"),
            _ => format!(
                "How did you get this :P [this is a bug!]: {:08b}",
                part as u8
            ),
        }
    }
}

pub struct PalmPlugin;

impl Plugin for PalmPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PalmTree>()
            .add_systems(
                (grow_palm, grow_nut)
                    .after(update_growth)
                    .before(scail_with_groth)
                    .in_set(OnUpdate(GameState::Playing)),
            )
            .add_system(pick_leaf.in_set(OnUpdate(Tool::Shears)))
            .add_system(pick_nut.in_set(OnUpdate(Tool::Hand)))
            .add_system(dig_root.in_set(OnUpdate(Tool::Shovel)))
            .add_system(chop_palm.in_set(OnUpdate(Tool::Axe)))
            .init_resource::<PalmAssets>();
    }
}

impl PlantTrait for PalmTree {
    fn spawn(cell: &MapCell, mut parent: EntityCommands) {
        parent.with_children(|p| {
            p.spawn((
                MaterialMeshBundle::<CustomMaterial> {
                    material: Handle::weak(ConstHandles::WaveMaterial.into()),
                    mesh: Handle::weak("objs/Palm.obj#Nut".into()),
                    transform: Transform::from_translation(cell.seed_offset()),
                    ..Default::default()
                },
                PalmTree,
                GrothStage::Seed,
                GrothProgress::new(rand::thread_rng().gen_range(2.0..5.0)),
            ));
        });
    }
}

fn grow_nut(
    mut palm: Query<(&mut Handle<Mesh>, &GrothProgress), With<PalmNut>>,
    palm_asstes: Res<PalmAssets>,
) {
    for (mut nut, groth) in &mut palm {
        if groth.percent() > 0.95 {
            *nut = palm_asstes.fruit.clone();
        } else {
            *nut = palm_asstes.seed.clone();
        }
    }
}
fn grow_palm(
    mut commands: Commands,
    mut palm: Query<(Entity, &mut GrothProgress, &mut GrothStage, &mut Transform), With<PalmTree>>,
    palm_asstes: Res<PalmAssets>,
) {
    for (entity, mut palm, mut stage, mut transform) in &mut palm {
        if palm.finished() {
            match *stage {
                GrothStage::Dead => {}
                GrothStage::Seed => {
                    commands.entity(entity).insert(palm_asstes.sprout.clone());
                    *stage = GrothStage::Sprout;
                    palm.reset();
                }
                GrothStage::Sprout => {
                    transform.scale = Vec3::splat(0.);
                    commands
                        .entity(entity)
                        .insert((palm_asstes.trunk.clone(), ScailWithGroth))
                        .with_children(|p| {
                            for i in 0..6 {
                                p.spawn((
                                    MaterialMeshBundle::<CustomMaterial> {
                                        transform: Transform::from_translation(
                                            PalmTree::leaf_offset(),
                                        )
                                        .with_rotation(Quat::from_rotation_y(1.0472 * i as f32))
                                        .with_scale(Vec3::ZERO),
                                        material: Handle::weak(ConstHandles::WaveMaterial.into()),
                                        mesh: palm_asstes.leaf.clone(),
                                        ..Default::default()
                                    },
                                    GrothProgress::new(1.),
                                    ScailWithGroth,
                                    PalmLeaf,
                                    PickableBundle::default(),
                                ));
                            }
                        });
                    *stage = GrothStage::Small;
                    palm.reset();
                }
                GrothStage::Small => {
                    *stage = GrothStage::Full;
                    commands.entity(entity).remove::<ScailWithGroth>();
                    commands
                        .entity(entity)
                        .insert(bevy_mod_picking::PickableBundle::default());
                }
                GrothStage::Full => {
                    commands.entity(entity).with_children(|p| {
                        for i in 0..3 {
                            p.spawn((
                                MaterialMeshBundle::<CustomMaterial> {
                                    transform: Transform::from_translation(
                                        PalmTree::coconut_offset(i),
                                    )
                                    .with_scale(Vec3::ZERO),
                                    material: Handle::weak(ConstHandles::WaveMaterial.into()),
                                    mesh: palm_asstes.seed.clone(),
                                    ..Default::default()
                                },
                                GrothProgress::new(5.),
                                ScailWithGroth,
                                PalmNut,
                                PickableBundle::default(),
                            ));
                        }
                    });
                    *stage = GrothStage::Dead;
                }
                _ => {
                    error!("palm grow from {:?} not impl", *stage);
                    *stage = GrothStage::Dead
                }
            }
        }
    }
}

#[derive(Component)]
struct PalmLeaf;
#[derive(Component)]
struct PalmNut;

fn pick_leaf(
    mut events: EventWriter<InventoryEvent>,
    mut leaf: Query<(&mut GrothProgress, &Interaction), (Changed<Interaction>, With<PalmLeaf>)>,
) {
    for (mut groth, interaction) in &mut leaf {
        if groth.percent() != 1. {
            continue;
        }
        if let Interaction::Clicked = interaction {
            events.send(InventoryEvent::AddItem(Item::Ingredient(
                Plant::Palm,
                PlantPart::Leaf,
            )));
            groth.reset();
        }
    }
}

fn pick_nut(
    mut events: EventWriter<InventoryEvent>,
    mut nut: Query<(&mut GrothProgress, &Interaction), (Changed<Interaction>, With<PalmNut>)>,
) {
    for (mut groth, interaction) in &mut nut {
        if groth.percent() < 0.75 {
            continue;
        }
        if let Interaction::Clicked = interaction {
            if groth.percent() > 0.95 {
                events.send(InventoryEvent::AddItem(Item::Ingredient(
                    Plant::Palm,
                    PlantPart::Fruit,
                )));
            } else {
                events.send(InventoryEvent::AddItem(Item::Ingredient(
                    Plant::Palm,
                    PlantPart::Seed,
                )));
            }
            groth.reset();
        }
    }
}

fn dig_root(
    mut events: EventWriter<InventoryEvent>,
    leaf: Query<(&GrothProgress, &Interaction), (Changed<Interaction>, With<PalmTree>)>,
) {
    for (groth, interaction) in &leaf {
        if groth.percent() != 1. {
            continue;
        }
        if let Interaction::Clicked = interaction {
            events.send(InventoryEvent::AddItem(Item::Ingredient(
                Plant::Palm,
                PlantPart::Root,
            )));
        }
    }
}

fn chop_palm(
    mut commands: Commands,
    mut events: EventWriter<InventoryEvent>,
    mut nut: Query<
        (Entity, &GrothProgress, &Interaction, &Parent),
        (Changed<Interaction>, With<PalmTree>),
    >,
) {
    for (entity, groth, interaction, parent) in &mut nut {
        if groth.percent() != 1. {
            continue;
        }
        if let Interaction::Clicked = interaction {
            commands.entity(entity).despawn_recursive();
            commands.entity(parent.get()).remove::<Plant>();
            events.send(InventoryEvent::AddItem(Item::Ingredient(
                Plant::Palm,
                PlantPart::Stem,
            )));
            events.send(InventoryEvent::AddItem(Item::Ingredient(
                Plant::Palm,
                PlantPart::Bark,
            )));
        }
    }
}

#[allow(dead_code)]
#[derive(AssetCollection, Resource)]
pub(super) struct PalmAssets {
    #[asset(path = "objs/Palm.obj#Leaf")]
    leaf: Handle<Mesh>,
    #[asset(path = "objs/Palm.obj#Trunk")]
    trunk: Handle<Mesh>,
    #[asset(path = "objs/Palm.obj#Nut")]
    seed: Handle<Mesh>,
    #[asset(path = "objs/Palm.obj#Sprout")]
    sprout: Handle<Mesh>,
    #[asset(path = "objs/Palm.obj#Fruit")]
    fruit: Handle<Mesh>,
}

impl FromWorld for PalmAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        PalmAssets {
            leaf: asset_server.load("objs/Palm.obj#Leaf"),
            trunk: asset_server.load("objs/Palm.obj#Trunk"),
            seed: asset_server.load("objs/Palm.obj#Nut"),
            sprout: asset_server.load("objs/Palm.obj#Sprout"),
            fruit: asset_server.load("objs/Palm.obj#Fruit"),
        }
    }
}
