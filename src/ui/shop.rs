use crate::{inventory::Inventory, prelude::*};
use belly::prelude::*;
use bevy::prelude::*;

pub(super) fn open_shop(
    mut commands: Elements,
    mut is_init: Local<bool>,
    target: Res<TargetPotion>,
) {
    if *is_init {
        commands.select("#shop").remove_class("hidden");
    } else {
        let id = Slot::Shop;
        let bgc = BackgroundColor(Color::WHITE);
        let val = target.potion_request();
        commands.commands().add(eml!{
        <div id="shop" c:content>
            <div c:shop-text>
                <label bind:value=from!(TargetPotion:potion_request())
                value=val/>
            </div>
            <button id="turn-in" flat on:press=run!(|ctx| {
                ctx.commands().add(
                    |world: &mut World| {
                        println!("Clicked turn in");
                        let inventory = world.resource::<Inventory>();
                        let target = world.resource::<TargetPotion>();
                        let Some(item) = inventory.get(&Slot::Shop) else {world.send_event(PlayerMessage::say("Add a potion to hand in into the slot", Color::YELLOW)); return;};

                        let Item::Potion(item) = item else {world.send_event(PlayerMessage::say("Item in slot must be potion", Color::YELLOW)); return;};
                        if target.is_match(item) {
                            //todo return item as reward
                            world.send_event(InventoryEvent::RemoveItem(Slot::Shop));
                            world.insert_resource(crate::crafting::potions::TargetPotion::new());
                        }
                    }
                )
            })>{"Turn In"}</button>
            <button id="item" s:background-color=managed() flat c:slot with=(id, Item, bgc)>
                <img c:slot-icon/>
            </button>
            <button id="skip" flat on:press=run!(|ctx| ctx.commands().insert_resource(TargetPotion::new()))>{"Skip"}</button>
        </div>
    });
        *is_init = true;
    }
}
