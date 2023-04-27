use bevy::log::prelude::*;
use bevy::log::Level;
use bevy::prelude::{Color, EventReader, Plugin};
pub struct MsgPlugin;
impl Plugin for MsgPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<PlayerMessage>();
        #[cfg(debug_assertions)]
        app.add_system(log_msgs);
    }
}
pub struct PlayerMessage {
    msg: String,
    color: Color,
    level: Level,
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
