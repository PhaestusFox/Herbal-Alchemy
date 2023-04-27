use crate::loading::FontAssets;
use crate::prelude::*;
use crate::GameState;
use bevy::prelude::*;

pub struct MenuPlugin;

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonColors>()
            .add_system(setup_main_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_system(click_play_button.in_set(OnUpdate(GameState::MainMenu)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(setup_settings_menu.in_schedule(OnEnter(GameState::Settings)))
            .add_system(click_settings_button.in_set(OnUpdate(GameState::Settings)))
            .add_system(cleanup_menu.in_schedule(OnExit(GameState::Settings)))
            .add_system(save_settings.in_schedule(OnExit(GameState::Settings)));
    }
}

#[derive(Resource)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
    menu: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::rgb(0.15, 0.15, 0.15),
            hovered: Color::rgb(0.25, 0.25, 0.25),
            menu: Color::rgb(0.5, 0.5, 0.5),
        }
    }
}

const MENU_BOX: Style = Style {
    flex_direction: FlexDirection::Column,
    margin: UiRect::all(Val::Auto),
    size: Size {
        width: Val::Percent(50.),
        height: Val::Percent(75.),
    },
    flex_wrap: FlexWrap::Wrap,
    position_type: PositionType::Absolute,
    ..Style::DEFAULT
};

fn setup_main_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
) {
    commands
        .spawn((
            NodeBundle {
                style: MENU_BOX,
                background_color: button_colors.menu.into(),
                ..Default::default()
            },
            MenuButtonClean,
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    MenuButton::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    MenuButton::Settings,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Settings",
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn setup_settings_menu(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    button_colors: Res<ButtonColors>,
    settings: ResMut<CameraSettings>,
) {
    commands
        .spawn((
            NodeBundle {
                style: MENU_BOX,
                background_color: button_colors.menu.into(),
                ..Default::default()
            },
            MenuButtonClean,
        ))
        .with_children(|commands| {
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Px(50.0)),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MenuButton::Container,
                ))
                .with_children(|p| {
                    p.spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: button_colors.normal.into(),
                            ..Default::default()
                        },
                        MenuButton::ChangeSensitivity(-0.001),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "-",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
                    p.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Px(50.0)),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                format!("Sensitivity: {}", settings.sensitivity),
                                TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            MenuButton::ChangeSensitivity(0.),
                        ));
                    });
                    p.spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: button_colors.normal.into(),
                            ..Default::default()
                        },
                        MenuButton::ChangeSensitivity(0.001),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "+",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
                });
            commands
                .spawn((
                    NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Px(50.0)),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    MenuButton::Container,
                ))
                .with_children(|p| {
                    p.spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: button_colors.normal.into(),
                            ..Default::default()
                        },
                        MenuButton::ChangeSpeed(-1.),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "-",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
                    p.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Auto, Val::Px(50.0)),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    })
                    .with_children(|parent| {
                        parent.spawn((
                            TextBundle::from_section(
                                format!("Pan Speed: {}", settings.speed),
                                TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 40.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ),
                            MenuButton::ChangeSpeed(0.),
                        ));
                    });
                    p.spawn((
                        ButtonBundle {
                            style: Style {
                                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                                margin: UiRect::all(Val::Auto),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            background_color: button_colors.normal.into(),
                            ..Default::default()
                        },
                        MenuButton::ChangeSpeed(1.),
                    ))
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "+",
                            TextStyle {
                                font: font_assets.fira_sans.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ));
                    });
                });
            commands
                .spawn((
                    ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                            margin: UiRect::all(Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        background_color: button_colors.normal.into(),
                        ..Default::default()
                    },
                    MenuButton::Back,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Back",
                        TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

#[derive(Component, Debug)]
enum MenuButton {
    Container,
    Play,
    Settings,
    Back,
    ChangeSensitivity(f32),
    ChangeSpeed(f32),
}

fn click_play_button(
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<MenuButton>),
    >,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match button {
                MenuButton::Play => state.set(GameState::Playing),
                MenuButton::Settings => state.set(GameState::Settings),
                e => error!("Can't Click {:?} on main menu", e),
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

#[derive(Component)]
struct MenuButtonClean;

fn cleanup_menu(mut commands: Commands, button: Query<Entity, With<MenuButtonClean>>) {
    for button in &button {
        commands.entity(button).despawn_recursive();
    }
}

fn click_settings_button(
    mut settings: ResMut<CameraSettings>,
    button_colors: Res<ButtonColors>,
    mut state: ResMut<NextState<GameState>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &MenuButton),
        (Changed<Interaction>, With<MenuButton>),
    >,
    mut text: Query<(&mut Text, &MenuButton)>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => match button {
                MenuButton::Back => state.set(GameState::MainMenu),
                MenuButton::ChangeSensitivity(by) => {
                    settings.sensitivity += by;
                    warn!("change sens to {}", settings.sensitivity);
                    for (mut text, button) in text.iter_mut() {
                        match button {
                            MenuButton::ChangeSensitivity(_) => {
                                text.sections[0].value =
                                    format!("Sensitivity: {:.03}", settings.sensitivity);
                                break;
                            }
                            _ => {}
                        }
                    }
                }
                MenuButton::ChangeSpeed(by) => {
                    settings.speed += by;
                    for (mut text, button) in text.iter_mut() {
                        match button {
                            MenuButton::ChangeSpeed(_) => {
                                text.sections[0].value = format!("Pan Speed: {}", settings.speed);
                            }
                            _ => {}
                        }
                    }
                }
                e => error!("Can't Click {:?} on settings menu", e),
            },
            Interaction::Hovered => {
                *color = button_colors.hovered.into();
            }
            Interaction::None => {
                *color = button_colors.normal.into();
            }
        }
    }
}

fn save_settings(mut pkv: ResMut<bevy_pkv::PkvStore>, settings: ResMut<CameraSettings>) {
    let _ = pkv.set("player settings", settings.as_ref());
}
