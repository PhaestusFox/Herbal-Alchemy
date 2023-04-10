use crate::prelude::*;
use bevy::prelude::*;

pub struct ToolTipPlugin;

impl Plugin for ToolTipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_tooltip.in_schedule(OnExit(GameState::Loading)))
            .register_type::<TimeToOpen>()
            .add_system(move_tooltip.in_set(OnUpdate(GameState::Playing)))
            .add_system(unhide_tooltip.in_set(OnUpdate(GameState::Playing)))
            .add_system(update_item_tooltip.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Reflect)]
struct TimeToOpen(Timer);

#[derive(Component)]
pub struct ToolTipData(pub String);

#[derive(Component)]
struct ToolTip;
#[derive(Component)]
struct ToolTipText;

fn move_tooltip(
    query: Query<(Entity, &Interaction), (Changed<Interaction>, With<ToolTipData>)>,
    data: Query<&ToolTipData>,
    mut tool_tip_text: Query<&mut Text, With<ToolTipText>>,
    mut tool_tip: Query<(&mut Visibility, &mut TimeToOpen), With<ToolTip>>,
) {
    let mut text = tool_tip_text.single_mut();
    let (mut vis, mut open) = tool_tip.single_mut();
    let mut set = None;
    for (item, interaction) in &query {
        if let Interaction::Hovered = interaction {
            set = Some(item)
        }
        open.0.pause();
        open.0.reset();
    }
    if let Some(set) = set {
        open.0.unpause();
        *vis = Visibility::Hidden;
        if let Ok(data) = data.get(set) {
            text.sections[0].value = data.0.clone();
        }
    }
}

fn unhide_tooltip(
    time: Res<Time>,
    mut tool_tip: Query<(&mut Visibility, &mut TimeToOpen), With<ToolTip>>,
) {
    let (mut vis, mut open) = tool_tip.single_mut();
    open.0.tick(time.delta());
    if open.0.finished() {
        *vis = Visibility::Visible;
    }
}

fn spawn_tooltip(mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn((
            NodeBundle {
                z_index: ZIndex::Global(10),
                background_color: BackgroundColor(Color::WHITE),
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Auto, Val::Auto),
                    min_size: Size::new(Val::Auto, Val::Px(80.)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        left: Val::Auto,
                        right: Val::Auto,
                        top: Val::Px(0.),
                        bottom: Val::Auto,
                    },
                    margin: UiRect::horizontal(Val::Auto),
                    ..Default::default()
                },
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            ToolTip,
            TimeToOpen(Timer::from_seconds(0.5, TimerMode::Once)),
            Name::new("ToolTip"),
        ))
        .with_children(|p| {
            p.spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            style: TextStyle {
                                font: fonts.fira_sans.clone(),
                                font_size: 25.,
                                color: Color::BLACK,
                            },
                            value: String::from("Uninit ToolTip"),
                        }],
                        alignment: TextAlignment::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ToolTipText,
            ));
        });
}

fn update_item_tooltip(mut commands: Commands, items: Query<(Entity, &Item), Changed<Item>>) {
    for (entity, item) in &items {
        commands
            .entity(entity)
            .insert(ToolTipData(item.tool_tip_text()));
    }
}
