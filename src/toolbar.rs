use bevy::{prelude::*, ecs::world::EntityMut};
use bevy_ninepatch::{NinePatchBundle, NinePatchBuilder, NinePatchData};
use strum::IntoEnumIterator;
use crate::{prelude::*, inventory::{HotBar, Inventory}};

pub struct ToolBarPlugin;

#[derive(Resource)]
pub struct BarEntitys {
    pub hot_bar: Entity,
    pub tab_bar: Entity,
    pub tool_bar: Entity,
    pub inventory_tab: Entity,
}

impl FromWorld for BarEntitys {
    fn from_world(world: &mut World) -> Self {
        let mut nine_patch = world.resource_mut::<Assets<NinePatchBuilder>>();
        let nine_patch = nine_patch.add(NinePatchBuilder::by_margins(15, 15, 15, 15));
        world.resource_scope(|world, assets: Mut<UiAssets>| {
            world.resource_scope(|world, hotbar: Mut<HotBar>| {
                world.resource_scope(|world, inventory: Mut<Inventory>| {
                BarEntitys { hot_bar: spawn_hotbar(world.spawn_empty(), &assets, &nine_patch, &hotbar), tab_bar: spawn_tab_menu(world.spawn_empty(), &nine_patch, &assets), tool_bar: spawn_tool_menu(world.spawn_empty(), &nine_patch, &assets), inventory_tab: crate::inventory::spawn_inventory_tab(world.spawn_empty(), &nine_patch, &assets, &inventory) }
            })
            })
        })
    }
}

#[derive(Component)]
struct ToolbarElement;

impl Plugin for ToolBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems((init_bars, show_bars).in_schedule(OnEnter(GameState::Playing)))
        .add_systems((click_tab_button, click_tool_button, update_button_color).in_set(OnUpdate(GameState::Playing)))
        .add_system(back_to_menu.in_schedule(OnEnter(Tab::Menu)).run_if(in_state(GameState::Playing)));
    }
}

fn init_bars(mut commands: Commands) {
    commands.init_resource::<BarEntitys>();
}

fn show_bars(
    entitys: Option<Res<BarEntitys>>,
    mut visibilitys: Query<&mut Visibility>,
) {
    let Some(entitys) = entitys else {return};
    let _ = visibilitys.get_mut(entitys.hot_bar).and_then(|mut v| {*v = Visibility::Visible; Ok(())});
    let _ = visibilitys.get_mut(entitys.tab_bar).and_then(|mut v| {*v = Visibility::Visible; Ok(())});
    let _ = visibilitys.get_mut(entitys.tool_bar).and_then(|mut v| {*v = Visibility::Visible; Ok(())});
}

fn spawn_hotbar(
    mut commands: EntityMut,
    assets: &UiAssets,
    nine_patch: &Handle<NinePatchBuilder>,
    hotbar: &HotBar,
) -> Entity {
    commands.insert((NodeBundle {
        style: Style {
            size: Size { width: Val::Percent(90.), height: Val::Percent(10.) },
            position: UiRect{bottom: Val::Px(10.), left: Val::Auto, right: Val::Auto, top: Val::Auto},
            margin: UiRect::horizontal(Val::Auto),
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        ..Default::default()
    }, ToolbarElement)).with_children(|p| {
        for i in 0..10 {
            p.spawn(
                NinePatchBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Percent(9.5), Val::Auto),
                        aspect_ratio: Some(1.),
                        padding: UiRect::horizontal(Val::Px(2.)),
                        ..Default::default()
                    },
                    nine_patch_data: NinePatchData::with_single_content(
                        assets.outline.clone(),
                        nine_patch.clone(),
                        hotbar.0[i],
                    ),
                    ..Default::default()
                },
            );
        }
    }).id()
}

fn spawn_tab_menu(
    mut commands: EntityMut,
    nine_patch: &Handle<NinePatchBuilder>,
    assets: &UiAssets,
) -> Entity {
    commands.insert((NodeBundle {
        style: Style {
            size: Size { width: Val::Percent(10.), height: Val::Percent(80.) },
            position: UiRect{bottom: Val::Auto, left: Val::Px(0.), right: Val::Auto, top: Val::Px(0.)},
            margin: UiRect::bottom(Val::Auto),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ..Default::default()
    }, ToolbarElement)).with_children(|p| {
        for tab in Tab::iter() {
            let content = p.spawn((ButtonBundle {
                style: Style {
                    size: Size::all(Val::Percent(100.)),
                    min_size: Size::new(Val::Auto, Val::Px(60.)),
                    ..Default::default()
                },
                image: assets.get_tab_icon(tab).into(),
                ..Default::default()
            }, tab)).id();
            p.spawn(
                NinePatchBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Auto, Val::Percent(19.)),
                        aspect_ratio: Some(1.),
                        padding: UiRect::vertical(Val::Px(2.)),
                        ..Default::default()
                    },
                    nine_patch_data: NinePatchData::with_single_content(
                        assets.outline.clone(),
                        nine_patch.clone(),
                        content,
                    ),
                    ..Default::default()
                },
            );
        }
    }).id()
}

fn spawn_tool_menu(
    mut commands: EntityMut,
    nine_patch: &Handle<NinePatchBuilder>,
    assets: &UiAssets,
) -> Entity {
    commands.insert((NodeBundle {
        style: Style {
            size: Size { width: Val::Percent(10.), height: Val::Percent(80.) },
            position: UiRect{bottom: Val::Auto, right: Val::Px(0.), left: Val::Auto, top: Val::Px(0.)},
            margin: UiRect::bottom(Val::Auto),
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            ..Default::default()
        },
        ..Default::default()
    }, ToolbarElement)).with_children(|p| {
        for tool in Tool::iter() {
            let content = p.spawn((ButtonBundle {
                style: Style {
                    size: Size::all(Val::Percent(100.)),
                    min_size: Size::new(Val::Auto, Val::Px(60.)),
                    ..Default::default()
                },
                image: assets.get_tool_icon(tool).into(),
                ..Default::default()
            }, tool)).id();
            p.spawn(
                NinePatchBundle {
                    style: Style {
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        size: Size::new(Val::Auto, Val::Percent(19.)),
                        aspect_ratio: Some(1.),
                        padding: UiRect::vertical(Val::Px(2.)),
                        ..Default::default()
                    },
                    nine_patch_data: NinePatchData::with_single_content(
                        assets.outline.clone(),
                        nine_patch.clone(),
                        content,
                    ),
                    ..Default::default()
                },
            );
        }
    }).id()
}

fn click_tab_button(
    buttons: Query<(&Interaction, &Tab), Changed<Interaction>>,
    mut set_tab: ResMut<NextState<Tab>>,
) {
    for (interaction, tab) in &buttons {
        if let Interaction::Clicked = interaction {
            set_tab.set(*tab);
        }
    }
}

fn click_tool_button(
    buttons: Query<(&Interaction, &Tool), Changed<Interaction>>,
    mut set_tab: ResMut<NextState<Tool>>,
) {
    for (interaction, tab) in &buttons {
        if let Interaction::Clicked = interaction {
            set_tab.set(*tab);
        }
    }
}

fn update_button_color(
    mut tools: Query<(&mut BackgroundColor, &Tool), Without<Tab>>,
    tool: Res<State<Tool>>,
    mut tabs: Query<(&mut BackgroundColor, &Tab), Without<Tool>>,
    tab: Res<State<Tab>>,
) {
    if tool.is_changed() {
        for (mut color, button_tool) in &mut tools {
            if tool.0 == *button_tool {
                color.0 = Color::GRAY;
            } else {
                color.0 = Color::WHITE;
            }
        }
    }
    if tab.is_changed() {
        for (mut color, button_tab) in &mut tabs {
            if tab.0 == *button_tab {
                color.0 = Color::GRAY;
            } else {
                color.0 = Color::WHITE;
            }
        }
    }
}

fn back_to_menu(
    mut gamestate: ResMut<NextState<GameState>>,
    entitys: Res<BarEntitys>,
    mut visibilitys: Query<&mut Visibility>,
) {
    let _ = visibilitys.get_mut(entitys.hot_bar).and_then(|mut v| {*v = Visibility::Hidden; Ok(())});
    let _ = visibilitys.get_mut(entitys.tab_bar).and_then(|mut v| {*v = Visibility::Hidden; Ok(())});
    let _ = visibilitys.get_mut(entitys.tool_bar).and_then(|mut v| {*v = Visibility::Hidden; Ok(())});
    gamestate.set(GameState::MainMenu);
}