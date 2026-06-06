use bevy::prelude::*;
use bevy_ascii_terminal::{BoxStyle, Pivot, Terminal, color};

use crate::{
    GameState, GameTerminal,
    components::{LogMessage, Position, Redraw},
    player::Player,
    render,
};

#[derive(Component, Debug)]
#[relationship(relationship_target = HeldItems)]
pub struct ItemHeldBy(pub Entity);

/// Items Held by an entity. Shouldn't be modified - use [ItemHeldBy] instead
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

fn show_inventory(
    mut term: Single<&mut Terminal, With<GameTerminal>>,
    inventory: Option<Single<&HeldItems, With<Player>>>,
    q_name: Query<&Name>,
    //mut commands: Commands,
) {
    for t in term.tiles_mut() {
        t.fg_color = render::greyscale(t.fg_color);
    }
    let piv = term.pivot();
    term.set_pivot(Pivot::LeftTop);

    let size = IVec2::new(30, 15);
    let pos = IVec2::new(8, 8);

    let bordered_pos = pos - 1;
    let bordered_size = (size + 1).as_uvec2();

    term.put_box(
        bordered_pos,
        bordered_size,
        BoxStyle::SINGLE_LINE.fill_center(),
    );
    term.put_string(bordered_pos + IVec2::new(1, 0), "Inventory");
    if let Some(inventory) = inventory.map(|i| i.into_inner()) {
        let mut y = pos.y;
        for iname in inventory
            .iter()
            .map(|i| q_name.get(i).expect("Missing item name"))
        {
            term.put_string([pos.x, y], iname.as_str());
            y += 1;
        }
    }

    term.set_pivot(piv);
}

fn inventory_update(input: Res<ButtonInput<KeyCode>>, mut commands: Commands) {
    if input.just_pressed(KeyCode::Escape) {
        commands.set_state(GameState::Exploring);
        commands.write_message(Redraw);
    }
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
