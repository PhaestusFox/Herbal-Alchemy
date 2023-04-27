use crate::prelude::*;
use belly::widgets::common::Label;
use bevy::prelude::*;

pub struct ToolTipPlugin;

impl Plugin for ToolTipPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(wright_tooltip.in_set(OnUpdate(GameState::Playing)))
            .add_system(update_item_tooltip.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Component, Reflect)]
struct TimeToOpen(Timer);

#[derive(Component)]
pub struct ToolTipData(pub String);

#[derive(Component, Default)]
pub(crate) struct ToolTip;
#[derive(Component)]
struct ToolTipText;

fn wright_tooltip(
    query: Query<(&ToolTipData, &Interaction), (Changed<Interaction>, With<ToolTipData>)>,
    mut tool_tip_text: Query<&mut Label, With<ToolTip>>,
) {
    let mut text = tool_tip_text.single_mut();
    for (data, interaction) in &query {
        if let Interaction::Hovered = interaction {
            text.value = data.0.clone();
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
