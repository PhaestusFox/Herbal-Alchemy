use crate::loading::TextureAssets;
use crate::GameState;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Serialize, Deserialize};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Resource, Serialize, Deserialize)]
pub struct PlayerSettings { 
    pub speed: f32,
    pub sensitivity: f32
}

impl Default for PlayerSettings {
    fn default() -> Self {
        PlayerSettings { speed: 1., sensitivity: 0.005 }
    }
}

#[derive(Default, Component)]
pub struct LookData {
    offset: Vec3,
    yaw: f32,
    pitch: f32,
}

fn look_mode(input: Res<Input<MouseButton>>) -> bool {
    input.pressed(MouseButton::Middle)
}

fn move_mode(input: Res<Input<MouseButton>>) -> bool {
    input.pressed(MouseButton::Left)
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(AmbientLight{ brightness: 1., color: Color::WHITE})
            .add_system(move_player.in_set(OnUpdate(GameState::Playing))
            .run_if(move_mode))
            .add_system(player_look.in_set(OnUpdate(GameState::Playing))
            .run_if(look_mode))
        .add_system(setup_settings.on_startup());
    }
}

fn setup_settings(mut pkv: ResMut<PkvStore>) {
    if let Err(e) = pkv.get::<PlayerSettings>("player settings") {
        match e {
            bevy_pkv::GetError::NotFound => {
                if let Err(e) = pkv.set("player settings", &PlayerSettings::default()) {
                    error!("{}", e);
                }
            },
            e => {
                error!("PKV Error for Player Settings {}", e);
                if let Err(e) = pkv.set("player settings", &PlayerSettings::default()) {
                    error!("{}", e);
                }
            }
        }
    }
}

fn move_player(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Children), With<Player>>,
    camera: Query<&Transform, Without<Player>>,
    mut mouse_move: EventReader<MouseMotion>,
    pkv: Res<PkvStore>
) {
    let setting = pkv.get::<PlayerSettings>("player settings").expect("player settings are loaded");
    let mut player_movement = Vec2::ZERO;
    for MouseMotion{delta} in mouse_move.iter() {
        player_movement += *delta;
    }
    for (mut player_transform, children) in &mut player_query {
        let local_z = if let Ok(cam_tran) = camera.get(*children.get(0).unwrap_or(&Entity::from_raw(0))) {cam_tran.local_z()} else {
            error!("First Child on player should be camera");
            continue;
        };
        let forward = player_movement.y * -Vec3::new(local_z.x, 0., local_z.z) + player_movement.x * -Vec3::new(local_z.z, 0., -local_z.x);
        player_transform.translation += forward * setting.speed * time.delta_seconds();
    }
}

fn player_look(
    mut player: Query<(&mut Transform, &mut LookData)>,
    mut mouse_move: EventReader<MouseMotion>,
    pkv: Res<PkvStore>,
) {
    let setting = pkv.get::<PlayerSettings>("player settings").expect("player settings is loaded");
    let mut total = Vec2::ZERO;
    for MouseMotion{delta} in mouse_move.iter() {
        total += *delta;
    }
    for (mut transfrom, mut data) in player.iter_mut() {
        data.yaw += total.x * setting.sensitivity;
        data.pitch += total.y * setting.sensitivity;
        data.pitch = data.pitch.clamp(0., 1.5);
        let cos = data.yaw.cos();
        let sin = data.yaw.sin();
        data.offset = Vec3::new(cos - sin, data.pitch, cos + sin);
        
        transfrom.translation = data.offset * 5.0;
        transfrom.look_at(Vec3::ZERO, Vec3::Y);

    }
}