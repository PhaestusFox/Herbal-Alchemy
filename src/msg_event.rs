use bevy::prelude::Color;

pub struct PlayerMessage {
    msg: String,
    color: Color,
}

impl PlayerMessage {
    pub fn error(msg: impl Into<String>) -> PlayerMessage {
        PlayerMessage { msg: msg.into(), color: Color::RED }
    }
}