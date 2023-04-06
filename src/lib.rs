mod actions;
mod audio;
mod loading;
mod menu;
mod player;
mod map;
mod mesh;
mod plants;
mod utils;
mod toolbar;
mod inventory;
mod tabs;

mod prelude { 
    pub(crate) use super::GameState;
    pub(crate) use crate::utils::ConstHandles;
    pub(crate) use crate::loading::*;
}

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use player::{Player, LookData};

type FixedPoint = fixed::types::I16F16;
type WaveObject = bevy_wave_collapse::objects::WaveObject<FixedPoint, mesh::MeshTextureUVS, u64, u64>;
type WaveMesh = bevy_wave_collapse::prelude::WaveMesh<FixedPoint, mesh::MeshTextureUVS>;
type WaveBuilder = bevy_wave_collapse::prelude::WaveBuilder<FixedPoint, mesh::MeshTextureUVS>;
type RVec3 = bevy_wave_collapse::prelude::RVec3<FixedPoint>;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the main menu is drawn and waiting for player interaction
    MainMenu,
    // Here the player can change settings
    SettingsMenu
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(map::MapPlugin)
            .add_plugins(plants::PlantPlugin)
            .init_asset_loader::<utils::ObjLoader>()
            .init_resource::<utils::VoidHandles>()
            .add_plugin(toolbar::ToolBarPlugin)
            .add_plugin(tabs::TabPlugin)
            .add_plugin(inventory::InventoryPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}


pub fn setup_camera(mut commands: Commands) {
    commands.spawn((Player, SpatialBundle::default())).with_children(|p| {
        p.spawn((Camera3dBundle {
            transform: Transform::from_translation(Vec3::Z * 5.),
            ..Default::default()
        }, LookData::default(), bevy_mod_picking::PickingCameraBundle::default()));
    });
}