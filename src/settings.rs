use belly::{prelude::BtnGroup, widgets::range::Range};
use bevy::prelude::*;
use bevy_pkv::PkvStore;
use serde::{Deserialize, Serialize};

use crate::GameState;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraSettings>()
            .add_system(update_camera_settings)
            .add_system(save_settings.in_schedule(OnExit(GameState::Settings)));
    }
}

#[derive(Debug, Resource, Serialize, Deserialize, Clone, Copy)]
pub struct CameraSettings {
    pub speed: f32,
    pub sensitivity: f32,
    pub move_cam: MouseButton,
    pub rotate_cam: MouseButton,
}

impl FromWorld for CameraSettings {
    fn from_world(world: &mut World) -> Self {
        let pkv_store = world.resource_mut::<PkvStore>();
        match pkv_store.get("player settings") {
            Err(e) => {
                match e {
                    bevy_pkv::GetError::NotFound => {}
                    e => {
                        world.send_event(crate::msg_event::PlayerMessage::error(format!(
                            "Error Getting Settings; {}",
                            e
                        )));
                    }
                }
                CameraSettings {
                    speed: 1.,
                    sensitivity: 0.005,
                    move_cam: MouseButton::Left,
                    rotate_cam: MouseButton::Middle,
                }
            }
            Ok(val) => val,
        }
    }
}

#[derive(Debug, Component)]
pub enum SettingsSlider {
    Sensitivity,
    Speed,
}

#[derive(Component, Clone, Copy)]
pub enum SettingsButton {
    MoveCamera,
    RotateCamera,
}

fn update_camera_settings(
    sliders: Query<(&Range, &SettingsSlider), Changed<Range>>,
    buttons: Query<(&BtnGroup, &SettingsButton), Changed<BtnGroup>>,
    mut settings: ResMut<CameraSettings>,
) {
    for (Range { value, .. }, slider) in &sliders {
        info!("set settings from {:?}", settings);
        match slider {
            SettingsSlider::Sensitivity => {
                if value.relative() == 0. {
                    settings.set_changed()
                } else {
                    settings.sensitivity = value.absolute()
                }
            }
            SettingsSlider::Speed => {
                if value.relative() == 0. {
                    settings.set_changed()
                } else {
                    settings.speed = value.absolute()
                }
            }
        }
        info!("set settings to {:?}", settings);
    }

    for (BtnGroup { value, .. }, button) in &buttons {
        let new_button = match &value[..] {
            "Left" => MouseButton::Left,
            "Right" => MouseButton::Right,
            "Middle" => MouseButton::Middle,
            _ => {
                warn!("Unknown Button {}", value);
                continue;
            }
        };
        match button {
            SettingsButton::MoveCamera => settings.move_cam = new_button,
            SettingsButton::RotateCamera => settings.rotate_cam = new_button,
        }
    }
}

fn save_settings(mut pkv: ResMut<bevy_pkv::PkvStore>, settings: ResMut<CameraSettings>) {
    let _ = pkv.set("player settings", settings.as_ref());
}
