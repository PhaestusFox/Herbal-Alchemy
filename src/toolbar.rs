use bevy::prelude::*;
use bevy_ninepatch::{NinePatchBundle, NinePatchBuilder, NinePatchData};
use crate::{prelude::*, inventory::HotBar};

pub struct ToolBarPlugin;

impl Plugin for ToolBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_hotbar.in_schedule(OnEnter(GameState::Playing)));
    }
}

fn spawn_hotbar(
    mut commands: Commands,
    assets: Res<UiAssets>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
    hotbar: Res<HotBar>,
) {
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));
    commands.spawn(NodeBundle {
        style: Style {
            size: Size { width: Val::Percent(90.), height: Val::Px(120.) },
            position: UiRect{bottom: Val::Px(10.), left: Val::Auto, right: Val::Auto, top: Val::Auto},
            margin: UiRect::horizontal(Val::Auto),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        ..Default::default()
    }).with_children(|p| {
        for i in 0..10 {
            p.spawn(
                NinePatchBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(10.), Val::Auto),
                        aspect_ratio: Some(1.),
                        ..Default::default()
                    },
                    nine_patch_data: NinePatchData::with_single_content(
                        assets.ui_outline.clone(),
                        nine_patch_handle.clone(),
                        hotbar.0[i],
                    ),
                    ..Default::default()
                },
            );
        }
    });
}