use crate::inventory::Inventory;
use crate::prelude::*;
use belly::prelude::*;
use bevy::prelude::*;

pub struct InventoryUiPlugin;

impl Plugin for InventoryUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(open_inventory.in_schedule(OnEnter(Tab::Inventory)))
            .add_system(super::hide_pannel::<3>.in_schedule(OnExit(Tab::Inventory)))
            .add_system(populate_inventory)
            .add_system(update_inventory.run_if(|inventory: Res<Inventory>| inventory.is_changed()))
            .add_system(update_inventory.in_schedule(OnExit(GameState::MainMenu)))
            .add_system(set_icon);
    }
}

fn open_inventory(mut commands: Elements, mut is_init: Local<bool>) {
    if *is_init {
        commands.select("#inventory").remove_class("hidden");
    } else {
        let bgc = BackgroundColor(Color::WHITE);
        commands.commands().add(eml! {
            <div id="inventory" c:content>
                <for id in = Slot::iter_inventory()>
                    <button s:background-color=managed() flat c:slot with=(id, Item, bgc)>
                        <img c:slot-icon/>
                    </button>
                </for>
            </div>
        });
        *is_init = true;
    }
}

fn populate_inventory(
    inventory: Res<Inventory>,
    mut items: Query<(&mut Item, &Slot), Added<Slot>>,
) {
    for (mut item, slot) in &mut items {
        *item = inventory.get(slot).unwrap_or(Item::Empty);
    }
}

fn update_inventory(inventory: Res<Inventory>, mut items: Query<(&mut Item, &Slot)>) {
    for (mut item, slot) in &mut items {
        *item = inventory.get(slot).unwrap_or(Item::Empty);
    }
}

fn set_icon(
    items: Query<(&Item, &Children), Changed<Item>>,
    mut images: Query<&mut Img>,
    asset_server: Res<AssetServer>,
) {
    for (item, icon) in &items {
        for child in icon {
            if let Ok(mut img) = images.get_mut(*child) {
                img.src = asset_server.load(item.icon_path()).into();
                img.modulate = item.background_color();
            } else {
                warn!("Faild to find img on child");
            }
        }
    }
}
