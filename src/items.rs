use bevy::prelude::*;
use bevy_ascii_terminal::color;

use crate::{
    combat::{Dice, HitPoints},
    components::{LogMessage, Position, Renderable},
};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(use_item);
    }
}

#[derive(Component, Default, Clone, Debug)]
pub struct Item;

#[derive(Component, Default, Clone, Debug)]
pub struct HealEffect(Dice);

#[derive(Component, Default, Clone, Debug)]
pub struct DamageEffect(Dice);

#[derive(Component, Default, Clone, Debug)]
pub struct Drinkable;

#[derive(Component, Default, Clone, Debug)]
pub struct Castable;

#[derive(Event)]
pub struct UseTargetedItem {
    pub user: Entity,
    pub target: Entity,
    pub item: Entity,
}

pub fn minor_healing_potion() -> impl Scene {
    bsn! {
        Name("Minor Healing Potion")
        Item
        HealEffect(Dice { dice: 4, faces: 3 })
        Renderable { glyph: '¡', fg_color: color::css::BLUE }
        Drinkable
    }
}

pub fn minor_healing_potion_pos(p: IVec2) -> impl Scene {
    bsn! {
        minor_healing_potion()
        Position(p)
    }
}

pub fn scroll_of_magic_missile() -> impl Scene {
    bsn! {
        Name("Scroll of Magic Missile")
        Item
        DamageEffect(Dice { dice: 4, faces: 4})
        Renderable { glyph: ')', fg_color: color::css::ORANGE }
        Castable
    }
}

pub fn scroll_of_magic_missile_pos(p: IVec2) -> impl Scene {
    bsn! {
        scroll_of_magic_missile()
        Position(p)
    }
}

fn use_item(
    e: On<UseTargetedItem>,
    q_user: Query<&Name>,
    mut q_target: Query<(&Name, &mut HitPoints)>,
    q_healing_item: Query<(&Name, &HealEffect)>,
    q_damage_item: Query<(&Name, &DamageEffect)>,
    mut commands: Commands,
) {
    let user = q_user.get(e.user).expect("Unnamed actor used an item");
    let (tar, mut hp) = q_target.get_mut(e.target).expect("Invalid target");

    if let Ok((name, effect)) = q_healing_item.get(e.item) {
        let old = hp.current;
        hp.current = (hp.current + effect.0.roll()).min(hp.max);
        let diff = hp.current - old;

        if e.target == e.user {
            if diff == 0 {
                commands.write_message(LogMessage(format!(
                    "{} used {}, but it had no effect...",
                    user.as_str(),
                    name.as_str(),
                )));
            } else {
                commands.write_message(LogMessage(format!(
                    "{} used {} and restored {} hit points.",
                    user.as_str(),
                    name.as_str(),
                    diff
                )));
            }
        } else {
            if diff == 0 {
                commands.write_message(LogMessage(format!(
                    "{} used {} on {}, but it had no effect...",
                    user.as_str(),
                    name.as_str(),
                    tar.as_str(),
                )));
            } else {
                commands.write_message(LogMessage(format!(
                    "{} used {} on {}, it restored {} hit points.",
                    user.as_str(),
                    name.as_str(),
                    tar.as_str(),
                    diff
                )));
            }
        }
        commands.entity(e.item).despawn();
    }

    if let Ok((name, effect)) = q_damage_item.get(e.item) {
        // todo: Need a system to detect when hp is 0 instead of having to send messages manually
        let old = hp.current;
        hp.current = (hp.current - effect.0.roll()).max(0);
        let diff = hp.current - old;
    }
}
