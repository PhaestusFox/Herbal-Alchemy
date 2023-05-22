use crate::{inventory::SelectedSlot, prelude::*};
use bevy::{ecs::world::EntityMut, prelude::*};
use bevy_console::ConsoleOpen;
use serde::{Deserialize, Serialize};

pub struct TabPlugin;
impl Plugin for TabPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Tab>()
            .register_type::<Tab>()
            .add_state::<Tool>()
            .register_type::<Tool>()
            .add_system(on_exit_menu.in_schedule(OnEnter(GameState::Playing)))
            .add_system(update_current_tab.in_set(OnUpdate(GameState::Playing)))
            .add_system(open_lab.in_schedule(OnEnter(Tab::Lab)))
            .add_system(close_lap.in_schedule(OnExit(Tab::Lab)))
            // .add_system(hand_in.in_set(OnUpdate(Tab::Shop)))
            .add_systems((change_tool, change_tab, change_state));
    }
}

#[derive(Resource)]
pub struct CurrentPotion(pub Item);
impl FromWorld for CurrentPotion {
    fn from_world(_: &mut World) -> Self {
        CurrentPotion(Item::Potion(Tags::EMPTY))
    }
}

#[derive(
    Debug,
    Reflect,
    States,
    PartialEq,
    Eq,
    Hash,
    Default,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    strum_macros::EnumIter,
    Component,
)]
pub enum Tab {
    #[default]
    Menu,
    World,
    Shop,
    Inventory,
    Lab,
}

impl UiItem for Tab {
    fn icon_path(&self) -> &'static str {
        match self {
            Tab::Menu => "textures/menu.png",
            Tab::World => "textures/map.png",
            Tab::Shop => "textures/shop.png",
            Tab::Inventory => "textures/inventory.png",
            Tab::Lab => "textures/lab.png",
        }
        .into()
    }
}

impl Tab {
    pub fn tool_tip(&self) -> ToolTipData {
        ToolTipData(match self {
            Tab::Menu => String::from("Open the Menu"),
            Tab::World => String::from("Visit The World to Garden"),
            Tab::Shop => String::from("See a customer"),
            Tab::Inventory => String::from("See ingredents"),
            Tab::Lab => String::from("Go to the lab to make a potion"),
        })
    }
}

fn on_exit_menu(pkv: Res<bevy_pkv::PkvStore>, mut next: ResMut<NextState<Tab>>) {
    if let Ok(current) = pkv.get("current_tab") {
        next.set(current);
    } else {
        next.set(Tab::World);
    }
}

fn update_current_tab(mut pkv: ResMut<bevy_pkv::PkvStore>, current: Res<State<Tab>>) {
    if current.is_changed() && current.0 != Tab::Menu {
        if let Err(e) = pkv.set("current_tab", &current.0) {
            error!("{:?}", e);
        };
    }
}

#[derive(
    Debug,
    Reflect,
    States,
    PartialEq,
    Eq,
    Hash,
    Default,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    strum_macros::EnumIter,
    Component,
)]
pub enum Tool {
    #[default]
    Hand,
    Axe,
    Shovel,
    Trowl,
    Shears,
}

impl UiItem for Tool {
    fn icon_path(&self) -> &'static str {
        match self {
            Tool::Hand => "textures/hand.png",
            Tool::Axe => "textures/axe.png",
            Tool::Shovel => "textures/shovel.png",
            Tool::Trowl => "textures/trowl.png",
            Tool::Shears => "textures/shears.png",
        }
    }
}

impl Tool {
    pub fn tool_tip(&self) -> ToolTipData {
        ToolTipData(match self {
            Tool::Hand => String::from("Your Hand used to collect Fruit"),
            Tool::Axe => String::from("An Axe used to cut down trees"),
            Tool::Shovel => String::from("A Shovel used to dig up roots"),
            Tool::Trowl => String::from("A trowl used to plant seeds"),
            Tool::Shears => String::from("Shears used to cut leaves of plants"),
        })
    }
}

pub const MAIN_WINDOW_STYLE: Style = Style {
    size: Size {
        width: Val::Percent(80.),
        height: Val::Percent(80.),
    },
    min_size: Size::all(Val::Percent(80.)),
    max_size: Size::all(Val::Percent(80.)),
    margin: UiRect {
        top: Val::Undefined,
        bottom: Val::Undefined,
        left: Val::Auto,
        right: Val::Auto,
    },
    position: UiRect {
        left: Val::Undefined,
        right: Val::Undefined,
        top: Val::Px(0.),
        bottom: Val::Undefined,
    },
    flex_wrap: FlexWrap::Wrap,
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};

fn open_lab(mut open: ResMut<ConsoleOpen>) {
    open.open = true;
}

fn close_lap(mut open: ResMut<ConsoleOpen>) {
    open.open = false;
}

#[derive(Component)]
struct ShopText;

#[derive(Component)]
struct ShopTab;

#[derive(Component)]
enum ShopButton {
    TurnIn,
    Skip,
}

pub fn spawn_shop_tab(
    mut root: EntityMut,
    nine_patch: &Handle<bevy_ninepatch::NinePatchBuilder>,
    assets: &UiAssets,
    font: Handle<Font>,
) {
    root.insert((
        NodeBundle {
            style: MAIN_WINDOW_STYLE.clone(),
            visibility: Visibility::Hidden,
            ..Default::default()
        },
        ShopTab,
        Name::new("ShopTab"),
    ))
    .with_children(|p| {
        let content = p
            .spawn((
                TextBundle {
                    style: Style {
                        size: Size::all(Val::Percent(100.)),
                        max_size: Size {
                            width: Val::Px(1000.),
                            height: Val::Auto,
                        },
                        margin: UiRect::horizontal(Val::Auto),
                        ..Default::default()
                    },
                    text: Text {
                        sections: vec![TextSection {
                            style: TextStyle {
                                font: font.clone(),
                                font_size: 40.,
                                color: Color::BLACK,
                            },
                            value: "Shop Text Init".to_string(),
                        }],
                        alignment: TextAlignment::Center,
                        linebreak_behaviour: bevy::text::BreakLineOn::WordBoundary,
                    },
                    ..Default::default()
                },
                ShopText,
            ))
            .id();
        p.spawn(bevy_ninepatch::NinePatchBundle {
            style: Style {
                size: Size::all(Val::Percent(100.)),
                ..Default::default()
            },
            nine_patch_data: bevy_ninepatch::NinePatchData::with_single_content(
                assets.outline.clone(),
                nine_patch.clone(),
                content,
            ),
            ..Default::default()
        });
        p.spawn((
            ButtonBundle {
                style: Style {
                    position: UiRect {
                        bottom: Val::Percent(10.),
                        right: Val::Percent(40.),
                        ..Default::default()
                    },
                    size: Size::all(Val::Px(100.)),
                    margin: UiRect::horizontal(Val::Auto),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
            ShopButton::TurnIn,
        ))
        .with_children(|p| {
            p.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Turn IN".to_string(),
                        style: TextStyle {
                            font: font.clone(),
                            font_size: 40.,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::GRAY),
                style: Style {
                    size: Size::all(Val::Percent(100.)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
        p.spawn((
            ButtonBundle {
                style: Style {
                    position: UiRect {
                        bottom: Val::Percent(10.),
                        left: Val::Percent(40.),
                        ..Default::default()
                    },
                    size: Size::all(Val::Px(100.)),
                    margin: UiRect::horizontal(Val::Auto),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
            ShopButton::Skip,
        ))
        .with_children(|p| {
            p.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Skip".to_string(),
                        style: TextStyle {
                            font,
                            font_size: 40.,
                            color: Color::BLACK,
                        },
                    }],
                    alignment: TextAlignment::Center,
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::GRAY),
                style: Style {
                    size: Size::all(Val::Percent(100.)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
    });
}

fn show_shop(mut tab: Query<&mut Visibility, With<ShopTab>>) {
    for mut shop in &mut tab {
        *shop = Visibility::Visible;
    }
}

fn hide_shop(mut tab: Query<&mut Visibility, With<ShopTab>>) {
    for mut shop in &mut tab {
        *shop = Visibility::Hidden;
    }
}

fn update_shop(
    mut text: Query<&mut Text, With<ShopText>>,
    target_potion: Res<crate::crafting::potions::TargetPotion>,
) {
    if target_potion.is_changed() {
        text.single_mut().sections[0].value = target_potion.potion_request();
    }
}

// fn hand_in(
//     item: Query<(&Item, &Slot), With<SelectedSlot>>,
//     mut target_potion: ResMut<crate::crafting::potions::TargetPotion>,
//     button: Query<(&Interaction, &ShopButton), Changed<Interaction>>,
//     mut events: EventWriter<InventoryEvent>,
// ) {
//     for (interaction, button) in &button {
//         if let Interaction::Clicked = interaction {
//             match button {
//                 ShopButton::Skip => *target_potion = crate::crafting::potions::TargetPotion::new(),
//                 ShopButton::TurnIn => {
//                     let Ok((Item::Potion(item), slot)) = item.get_single() else {return;};
//                     if target_potion.is_match(*item) {
//                         events.send(InventoryEvent::RemoveItem(*slot));
//                         *target_potion = crate::crafting::potions::TargetPotion::new();
//                     }
//                 }
//             }
//         }
//     }
// }

fn change_tab(
    query: Query<(&Interaction, &Tab), Changed<Interaction>>,
    mut next: ResMut<NextState<Tab>>,
) {
    for (interaction, tab) in &query {
        if let Interaction::Clicked = interaction {
            next.set(*tab);
        }
    }
}

fn change_tool(
    query: Query<(&Interaction, &Tool), Changed<Interaction>>,
    mut next: ResMut<NextState<Tool>>,
) {
    for (interaction, tool) in &query {
        if let Interaction::Clicked = interaction {
            next.set(*tool);
        }
    }
}

fn change_state(
    query: Query<(&Interaction, &GameState), Changed<Interaction>>,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, current) in &query {
        if let Interaction::Clicked = interaction {
            state.set(current.clone());
        }
    }
}
