mod inventory;
mod menu;
mod shop;

use crate::prelude::*;
use belly::prelude::*;
use bevy::prelude::*;

use menu::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BellyPlugin)
            .add_system(setup_loading.on_startup())
            .add_system(setup_ui.in_schedule(OnEnter(GameState::Playing)))
            .add_system(detect_change)
            .add_system(open_menu.run_if(state_changed::<GameState>()))
            .add_system(
                back_to_menu.run_if(
                    just_pressed(KeyCode::Escape)
                        .or_else(state_changed::<Tab>())
                        .and_then(state_exists_and_equals(Tab::Menu)),
                ),
            )
            .add_plugin(inventory::InventoryUiPlugin)
            .add_system(remove_pannel::<0>.in_schedule(OnExit(GameState::Loading)))
            .add_system(shop::open_shop.in_schedule(OnEnter(Tab::Shop)))
            .add_system(hide_pannel::<1>.in_schedule(OnExit(Tab::Shop)))
            .add_system(hide_pannel::<2>.run_if(state_changed::<Tab>()))
            .add_system(hide_hidden);
    }
}

pub trait UiItem {
    fn icon_path(&self) -> &'static str;
    fn background_color(&self) -> Color {
        Color::WHITE
    }
}

const PANNEL_IDS: [&'static str; 4] = ["#loading", "#shop", ".menu", "#inventory"];

fn hide_pannel<const PANNEL_ID: usize>(mut elements: Elements) {
    elements.select(PANNEL_IDS[PANNEL_ID]).add_class("hidden");
}

fn remove_pannel<const PANNEL_ID: usize>(mut elements: Elements) {
    elements.select(PANNEL_IDS[PANNEL_ID]).remove();
}

fn hide_hidden(mut query: Query<(&Element, &mut Visibility), Changed<Element>>) {
    for (element, mut visibility) in &mut query {
        if element.classes.contains(&"hidden".into()) {
            *visibility = Visibility::Hidden
        } else {
            *visibility = Visibility::Inherited
        }
    }
}
