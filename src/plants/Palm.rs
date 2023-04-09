use super::*;
use bevy_asset_loader::prelude::*;
use bevy_mod_picking::PickableBundle;
use rand::Rng;

#[derive(Component, Default, Reflect)]
pub struct PalmTree;

impl PalmTree {
    fn leaf_offset() -> Vec3 {
        Vec3 { x: 0., y: 0.9, z: -0.32 }
    }
}

pub struct PalmPlugin;

impl Plugin for PalmPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<PalmTree>()
        .add_systems((grow_palm.after(update_growth).before(scail_with_groth),).in_set(OnUpdate(GameState::Playing)))
        .add_system(pick_leaf.in_set(OnUpdate(Tool::Shears)));
    }
}

impl PlantTrait for PalmTree {
    fn spawn(cell: &MapCell, mut parent: EntityCommands) {
        parent.with_children(|p| {
            p.spawn((PbrBundle {
                material: Handle::weak(ConstHandles::WaveMaterial.into()),
                mesh: Handle::weak("objs/Palm.obj#Nut".into()),
                transform: Transform::from_translation(cell.seed_offset()),
                ..Default::default()
            }, PalmTree, GrothStage::Seed,
            GrothProgress::new(rand::thread_rng().gen_range(1.0..2.0))));
        });
    }
}

fn grow_palm(
    mut commands: Commands,
    mut palm: Query<(Entity, &mut GrothProgress, &mut GrothStage)>,
    palm_asstes: Res<PalmAssets>,
) {
    for (entity, mut palm, mut stage) in &mut palm {
        if palm.finished() {
            match *stage {
                GrothStage::Dead => {},
                GrothStage::Seed => {
                    commands.entity(entity).insert(palm_asstes.sprout.clone());
                    *stage = GrothStage::Sprout;
                    palm.reset();
                },
                GrothStage::Sprout => {
                    commands.entity(entity)
                    .insert((palm_asstes.trunk.clone(), ScailWithGroth))
                    .with_children(|p| {
                for i in 0..6 {
                    p.spawn((PbrBundle {
                        transform: Transform::from_translation(PalmTree::leaf_offset()).with_rotation(Quat::from_rotation_y(1.0472 * i as f32)).with_scale(Vec3::ZERO),
                        material: Handle::weak(ConstHandles::WaveMaterial.into()),
                        mesh: palm_asstes.leaf.clone(),
                        ..Default::default()
                    }, GrothProgress::new(1.), ScailWithGroth, PalmLeaf, PickableBundle::default()));
                }});
                    *stage = GrothStage::Small;
                    palm.reset();
                },
                GrothStage::Small => {
                    *stage = GrothStage::Full;
                    commands.entity(entity).insert(bevy_mod_picking::PickableBundle::default());
                },
                GrothStage::Full => {
                    commands.entity(entity).with_children(|p| {
                        
                    });
                },
                _ => {error!("palm grow from {:?} not impl", *stage); *stage = GrothStage::Dead},
            }
        }
    }
}

#[derive(Component)]
struct PalmLeaf;

fn pick_leaf(
    mut events: EventWriter<InventoryEvent>,
    mut leaf: Query<(&mut GrothProgress, &Interaction), (Changed<Interaction>, With<PalmLeaf>)>,
) {
    for (mut groth, interaction) in &mut leaf {
        if groth.percent() != 1. {continue;}
        if let Interaction::Clicked = interaction {
            events.send(InventoryEvent::AddItem(Item::Ingredient(Plant::Palm, PlantPart::Leaf)));
            groth.reset();
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
}