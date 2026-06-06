use bevy::prelude::*;
use bevy_ascii_terminal::{BoxStyle, Pivot, Terminal, TerminalStringBuilder};

use crate::{
    GameState, GameTerminal,
    components::{LogMessage, Position, Redraw},
    input,
    items::UseTargetedItem,
    player::Player,
    render,
};

#[derive(Component, Debug)]
#[relationship(relationship_target = HeldItems)]
pub struct ItemHeldBy(pub Entity);

/// Items held by an entity. Shouldn't be modified directly, use [ItemHeldBy] to manage
/// inventories
#[derive(Component, Debug, Deref)]
#[relationship_target(relationship = ItemHeldBy)]
pub struct HeldItems(Vec<Entity>);

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Inventory), show_inventory)
            .add_systems(
                Update,
                inventory_update.run_if(in_state(GameState::Inventory)),
            )
            .add_observer(pickup_item);
    }
}

#[derive(Event)]
pub struct PickupItem {
    pub item: Entity,
    pub picker_upper: Entity,
}

const INV_POS: IVec2 = IVec2::new(8, 8);
const INV_SIZE: IVec2 = IVec2::new(30, 15);

fn show_inventory(
    mut term: Single<&mut Terminal, With<GameTerminal>>,
    inventory: Option<Single<&HeldItems, With<Player>>>,
    q_name: Query<&Name>,
    mut commands: Commands,
) {
    let Some(inventory) = inventory.map(|i| i.into_inner()) else {
        commands.write_message(LogMessage("You have no items.".to_owned()));
        commands.set_state(GameState::Exploring);
        return;
    };

    for t in term.tiles_mut() {
        t.fg_color = render::greyscale(t.fg_color);
    }
    let piv = term.pivot();
    term.set_pivot(Pivot::LeftTop);

    let bordered_pos = INV_POS - 1;
    let bordered_size = (INV_SIZE + 1).as_uvec2();

    term.put_box(
        bordered_pos,
        bordered_size,
        BoxStyle::SINGLE_LINE.fill_center(),
    );
    term.put_string(
        bordered_pos + IVec2::new(1, 0),
        "[<fg=yellow>Inventory</fg>]",
    );

    term.set_pivot(piv);
}

fn inventory_update(
    input: Res<ButtonInput<KeyCode>>,
    mut selected: Local<i32>,
    mut term: Single<&mut Terminal, With<GameTerminal>>,
    inventory: Option<Single<&HeldItems, With<Player>>>,
    q_name: Query<&Name>,
    q_player: Single<Entity, With<Player>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::Escape) {
        commands.set_state(GameState::Exploring);
        commands.write_message(Redraw);
        return;
    }

    let Some(inventory) = inventory.map(|i| i.into_inner()) else {
        warn!("Inventory open with no items, returning to game state");
        commands.set_state(GameState::Exploring);
        return;
    };

    let max = (*selected).max(inventory.len() as i32);

    *selected = (*selected).clamp(0, max);

    if input.any_just_pressed(input::DOWN.iter().cloned()) {
        *selected = (*selected + 1) % max;
    }

    if input.any_just_pressed(input::UP.iter().cloned()) {
        *selected = (*selected - 1).rem_euclid(max);
    }

    if input.any_just_pressed(input::ACCEPT.iter().cloned()) {
        // TODO: Handle targeting for scrolls, allow throwing, etc...
        let player = q_player.into_inner();
        commands.trigger(UseTargetedItem {
            user: player,
            target: player,
            item: inventory[*selected as usize],
        });
        commands.set_state(GameState::Exploring);
        commands.write_message(Redraw);
        return;
    }

    let piv = term.pivot();
    term.set_pivot(Pivot::LeftTop);
    for (i, iname) in inventory
        .iter()
        .map(|i| q_name.get(i).expect("Missing item name"))
        .enumerate()
    {
        let i = i as i32;
        if i == *selected {
            let fg = term.clear_tile().bg_color;
            let bg = term.clear_tile().fg_color;
            term.put_string([INV_POS.x, INV_POS.y + i], iname.as_str().fg(fg).bg(bg));
        } else {
            term.put_string([INV_POS.x, INV_POS.y + i], iname.as_str());
        }
    }
    term.set_pivot(piv);
}

fn pickup_item(pickup: On<PickupItem>, q_name: Query<&Name>, mut commands: Commands) {
    commands
        .entity(pickup.item)
        .insert(ItemHeldBy(pickup.picker_upper))
        .remove::<Position>();

    let item_name = q_name.get(pickup.item).expect("Missing item name");
    let pup_name = q_name
        .get(pickup.picker_upper)
        .expect("Missing picker upper name");

    commands.write_message(LogMessage(format!(
        "{} picks up <fg=sky_blue>{}</fg>.",
        pup_name, item_name
    )));
}
