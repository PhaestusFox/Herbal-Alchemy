use crate::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(progress.in_set(OnUpdate(GameState::Loading)))
            .add_plugin(iyes_progress::ProgressPlugin::new(GameState::Loading));
        app.add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::MainMenu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, WaveMeshAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, UiAssets>(GameState::Loading)
        .add_collection_to_loading_state::<_, ItemIcons>(GameState::Loading);
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

#[derive(AssetCollection, Resource)]
pub struct WaveMeshAssets {
    #[asset(path = "objs/sand.wfo")]
    pub sand: Handle<WaveMesh>,
    #[asset(path = "objs/water.wfo")]
    pub water: Handle<WaveMesh>,
}

#[derive(AssetCollection, Resource)]
pub struct UiAssets {
    #[asset(path = "textures/ui_outline.png")]
    pub outline: Handle<Image>,
    #[asset(path = "textures/ui_selected.png")]
    pub selected: Handle<Image>,
    #[asset(path = "textures/shop.png")]
    pub shop_icon: Handle<Image>,
    #[asset(path = "textures/lab.png")]
    pub lab_icon: Handle<Image>,
    #[asset(path = "textures/menu.png")]
    pub menu_icon: Handle<Image>,
    #[asset(path = "textures/map.png")]
    pub map_icon: Handle<Image>,
    #[asset(path = "textures/inventory.png")]
    pub inventory_icon: Handle<Image>,
    #[asset(path = "textures/hand.png")]
    pub hand_icon: Handle<Image>,
    #[asset(path = "textures/axe.png")]
    pub axe_icon: Handle<Image>,
    #[asset(path = "textures/shovel.png")]
    pub shovel_icon: Handle<Image>,
    #[asset(path = "textures/trowl.png")]
    pub trowl_icon: Handle<Image>,
    #[asset(path = "textures/shears.png")]
    pub shears_icon: Handle<Image>,
}

impl UiAssets {
    pub fn get_tab_icon(&self, tab: Tab) -> Handle<Image> {
        match tab {
            Tab::Menu => self.menu_icon.clone(),
            Tab::World => self.map_icon.clone(),
            Tab::Shop => self.shop_icon.clone(),
            Tab::Inventory => self.inventory_icon.clone(),
            Tab::Lab => self.lab_icon.clone(),
        }
    }

    pub fn get_tool_icon(&self, tool: Tool) -> Handle<Image> {
        match tool {
            Tool::Hand => self.hand_icon.clone(),
            Tool::Axe => self.axe_icon.clone(),
            Tool::Shovel => self.shovel_icon.clone(),
            Tool::Trowl => self.trowl_icon.clone(),
            Tool::Shears => self.shears_icon.clone(),
        }
    }
}

#[derive(AssetCollection, Resource)]
pub struct ItemIcons {
    #[asset(path = "icons/null.png")]
    pub null: Handle<Image>,
    #[asset(path = "icons/empty.png")]
    pub empty: Handle<Image>,
    #[asset(path = "icons/palm/leaf.png")]
    pub palm_leaf: Handle<Image>,
    #[asset(path = "icons/palm/fruit.png")]
    pub palm_fruit: Handle<Image>,
    #[asset(path = "icons/palm/seed.png")]
    pub palm_seed: Handle<Image>,
    #[asset(path = "icons/cube.png")]
    pub cube: Handle<Image>,
    #[asset(path = "icons/ash.png")]
    pub ash: Handle<Image>,
    #[asset(path = "icons/palm/root.png")]
    pub palm_root: Handle<Image>,
    #[asset(path = "icons/palm/wood.png")]
    pub palm_wood: Handle<Image>,
    #[asset(path = "icons/palm/bark.png")]
    pub palm_bark: Handle<Image>,
    #[asset(path = "icons/potion_0.png")]
    pub potion_0: Handle<Image>,
    #[asset(path = "icons/potion_1.png")]
    pub potion_1: Handle<Image>,
    #[asset(path = "icons/potion_2.png")]
    pub potion_2: Handle<Image>,
    #[asset(path = "icons/potion_3.png")]
    pub potion_3: Handle<Image>,
    #[asset(path = "icons/potion_4.png")]
    pub potion_4: Handle<Image>,
    #[asset(path = "icons/potion_5.png")]
    pub potion_5: Handle<Image>,
    #[asset(path = "icons/potion_6.png")]
    pub potion_6: Handle<Image>,
    #[asset(path = "icons/potion_7.png")]
    pub potion_7: Handle<Image>,
    #[asset(path = "icons/potion_d.png")]
    pub potion_d: Handle<Image>,
}

#[derive(Component, Default)]
pub struct LoadingProgress;
use belly::widgets::common::Label;
fn progress(
    progress: Res<iyes_progress::ProgressCounter>,
    mut query: Query<&mut Label, With<LoadingProgress>>,
) {
    let progress = progress.progress();
    for mut label in query.iter_mut() {
        label.value = format!("Loading: {:?}", progress)
    }
}
