#![feature(adt_const_params)]
mod actions;
mod audio;
mod crafting;
mod inventory;
mod loading;
mod map;
mod menu;
mod mesh;
mod plants;
mod player;
mod tabs;
mod tool_tips;
mod toolbar;
mod utils;

mod msg_event;

mod shader;

mod prelude {
    pub(crate) use super::GameState;
    pub(crate) use crate::inventory::{InventoryEvent, Item, Slot};
    pub(crate) use crate::loading::*;
    pub use crate::map::MapCell;
    pub(crate) use crate::mesh::{RVec3, WaveBuilder, WaveMesh, WaveObject};
    pub(crate) use crate::tabs::{CurrentPotion, Tab, Tool};
    pub(crate) use crate::tool_tips::ToolTipData;
    pub(crate) use crate::utils::ConstHandles;
    pub type FixedPoint = fixed::types::I16F16;
    pub use super::shader::CustomMaterial;
    pub use crate::inventory::SelectedSlot;
    pub use crate::crafting::potions::PotionEffect;
    pub use crate::msg_event::PlayerMessage;
}

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::player::PlayerPlugin;

use bevy::app::App;
// #[cfg(debug_assertions)]
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use player::{LookData, Player};
use prelude::CustomMaterial;

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
    SettingsMenu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_asset::<CustomMaterial>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(map::MapPlugin)
            .add_plugins(plants::PlantPlugin)
            .init_asset_loader::<utils::ObjLoader>()
            .init_resource::<utils::VoidHandles>()
            .add_plugin(tabs::TabPlugin)
            .add_plugin(toolbar::ToolBarPlugin)
            .add_plugin(inventory::InventoryPlugin)
            .add_plugin(tool_tips::ToolTipPlugin)
            .add_plugin(crafting::CraftingPlugin)
            .add_plugin(mesh::MeshPlugin)
            .add_plugin(shader::ShaderPlugin)
            .add_system(setup_camera.on_startup())
            .add_plugin(msg_event::MsgPlugin);

        // #[cfg(debug_assertions)]
        // {
        //     app.add_plugin(FrameTimeDiagnosticsPlugin::default())
        //         .add_plugin(LogDiagnosticsPlugin::default());
        // }
    }
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn((Player, SpatialBundle::default()))
        .with_children(|p| {
            p.spawn((
                Camera3dBundle {
                    transform: Transform::from_translation(
                        Vec3::new(1., 45.0f32.to_radians(), 1.) * 5.0,
                    )
                    .looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                },
                LookData::default(),
                bevy_mod_picking::PickingCameraBundle::default(),
                // bevy_atmosphere::prelude::AtmosphereCamera::default(),
            ));
        });
}
