use bevy::prelude::*;
use crate::prelude::*;
use serde::{Serialize, Deserialize};

pub struct TabPlugin;
impl Plugin for TabPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<Tab>()
        .register_type::<Tab>()
        .add_state::<Tool>()
        .register_type::<Tool>()
        .add_system(on_exit_menu.in_schedule(OnEnter(GameState::Playing)))
        .add_system(update_current_tab.in_set(OnUpdate(GameState::Playing)));
    }
}

#[derive(Debug, Reflect, States, PartialEq, Eq, Hash, Default, Clone, Copy, Serialize, Deserialize, strum_macros::EnumIter, Component)]
pub enum Tab {
    #[default]
    Menu,
    World,
    Shop,
    Inventory,
    Lab,
}

fn on_exit_menu(
    pkv: Res<bevy_pkv::PkvStore>,
    mut next: ResMut<NextState<Tab>>
) {
    if let Ok(current) = pkv.get("current_tab") {
        next.set(current);
    } else {
        next.set(Tab::World);
    }
}

fn update_current_tab(
    mut pkv: ResMut<bevy_pkv::PkvStore>,
    current: Res<State<Tab>>,
) {
    if current.is_changed() && current.0 != Tab::Menu {
        if let Err(e) = pkv.set("current_tab", &current.0) {
            error!("{:?}", e);
        };
    }
}

#[derive(Debug, Reflect, States, PartialEq, Eq, Hash, Default, Clone, Copy, Serialize, Deserialize, strum_macros::EnumIter, Component)]
pub enum Tool {
    #[default]
    Hand,
    Axe,
    Shovel,
    Trowl,
    Shears,
}