// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::winit::WinitWindows;
use bevy::DefaultPlugins;
use bevy_pkv::PkvStore;
use herbal_alchemy::GamePlugin;
use std::io::Cursor;
use winit::window::Icon;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Herbal Alchemy".to_string(), // ToDo
                resolution: (1440., 720.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(PkvStore::new("PhoxCorp", "HerbalAlchemy"))
        .add_plugin(GamePlugin)
        .add_system(set_window_icon.on_startup())
        .add_system(herbal_alchemy::setup_camera.on_startup());
    #[cfg(debug_assertions)]
    app.add_plugin(bevy_editor_pls::EditorPlugin::default());
    app.add_plugins(bevy_mod_picking::DefaultPickingPlugins)
        .add_plugin(bevy_ninepatch::NinePatchPlugin::<()>::default())
        .add_plugin(bevy_console::ConsolePlugin)
        .insert_resource(bevy_console::ConsoleConfiguration {
            keys: Vec::with_capacity(0),
            ..Default::default()
        })
        //.add_plugin(bevy_atmosphere::prelude::AtmospherePlugin)
        .run();
}

// Sets the icon on windows and X11
fn set_window_icon(
    windows: NonSend<WinitWindows>,
    primary_window: Query<Entity, With<PrimaryWindow>>,
) {
    let primary_entity = primary_window.single();
    let primary = windows.get_window(primary_entity).unwrap();
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png"
    ));
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height).unwrap();
        primary.set_window_icon(Some(icon));
    };
}
