use belly::build::{widget, FromWorldAndParams};
use belly::prelude::*;
use bevy::log::prelude::*;
use bevy::log::Level;
use bevy::prelude::*;

pub struct MsgPlugin;
impl Plugin for MsgPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PlayerMessage>();
        #[cfg(debug_assertions)]
        app.add_system(log_msgs);
        #[cfg(debug_assertions)]
        app.add_system(log_test.on_startup());
        app.add_systems((spawn_msg, despawn_msg));
        app.add_system(spawn_msg_space.on_startup());
    }
}
pub struct PlayerMessage {
    msg: String,
    color: Color,
    level: Level,
}

fn log_test(mut events: EventWriter<PlayerMessage>) {
    events.send(PlayerMessage::error("You are in debug mode"));
}

impl PlayerMessage {
    pub fn error(msg: impl Into<String>) -> PlayerMessage {
        PlayerMessage {
            msg: msg.into(),
            color: Color::RED,
            level: Level::ERROR,
        }
    }
    pub fn warn(msg: impl Into<String>) -> PlayerMessage {
        PlayerMessage {
            msg: msg.into(),
            color: Color::YELLOW,
            level: Level::WARN,
        }
    }
    pub fn say(msg: impl Into<String>, color: Color) -> PlayerMessage {
        PlayerMessage {
            msg: msg.into(),
            color,
            level: Level::INFO,
        }
    }
}

fn log_msgs(mut events: EventReader<PlayerMessage>) {
    for event in events.iter() {
        match event.level {
            Level::DEBUG => {
                debug!("{}", event.msg)
            }
            Level::ERROR => {
                error!("{}", event.msg)
            }
            Level::INFO => {
                info!("{}", event.msg)
            }
            Level::TRACE => {
                trace!("{}", event.msg)
            }
            Level::WARN => {
                warn!("{}", event.msg)
            }
        }
    }
}

#[derive(Component)]
struct MsgTime(Timer);
impl Default for MsgTime {
    fn default() -> Self {
        MsgTime(Timer::from_seconds(5., bevy::time::TimerMode::Once))
    }
}

fn spawn_msg_space(mut commands: Commands) {
    commands.add(eml! {
        <div id="Msgs">

        </div>
    });
}

fn spawn_msg(
    mut events: EventReader<PlayerMessage>,
    mut elements: Elements,
    mut old: Query<&mut UiMsg>,
) {
    'event: for event in events.iter() {
        for mut msg in &mut old {
            if msg.value.starts_with(&event.msg) {
                msg.trys += 1;
                if let Some(index) = msg.value.find(' ') {
                    msg.value.truncate(index);
                }
                let suffix = format!(" \tx{}", msg.trys + 1);
                msg.value.push_str(&suffix);
                continue 'event;
            }
        }
        let msg = event.msg.clone();
        let color = event.color;
        elements.select("#Msgs").add_child(eml! {
            <ui_msg c:msg color=color value=msg />
        });
    }
}

fn despawn_msg(mut commands: Elements, time: Res<Time>, mut query: Query<(Entity, &mut MsgTime)>) {
    for (entity, mut timer) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).remove();
        }
    }
}

#[derive(Component, Default)]
pub struct UiMsg {
    value: String,
    color: Color,
    trys: usize,
}

#[widget]
#[param(value:String => UiMsg:value)]
#[param(color:Color => UiMsg:color)]
/// The `<ui_msg>` tag is a binable single line of text. It consumes
/// the children and renders the content of bindable `value` param.
fn ui_msg(ctx: &mut belly::build::WidgetContext) {
    let this = ctx.this().id();
    ctx.add(from!(this, UiMsg: value) >> to!(this, Text:sections[0].value));
    ctx.add(from!(this, UiMsg: color) >> to!(this, Text:sections[0].style.color));
    ctx.insert((
        belly::build::TextElementBundle::default(),
        MsgTime::default(),
    ));
}
