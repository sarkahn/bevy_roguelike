use bevy::prelude::*;
use bevy_ascii_terminal::color;

use crate::{
    GameState,
    combat::{Dice, HitPoints},
    components::{LogMessage, Renderable},
    state_animation::Animation,
    state_targeting::TargetingRange,
    turn_system::Energy,
};

pub struct ItemsPlugin;

impl Plugin for ItemsPlugin {
    fn build(&self, app: &mut App) {
        // Delay item use until after animations
        app.add_observer(use_item.run_if(not(in_state(GameState::Animating))));
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
pub struct MissileAnimation {
    pub glyph: char,
    pub fg_color: LinearRgba,
}

#[derive(Event)]
pub struct UseItem {
    pub user: Entity,
    pub item: Entity,
    pub targets: Vec<Entity>,
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

pub fn scroll_of_magic_missile() -> impl Scene {
    bsn! {
        Name("Scroll of Magic Missile")
        Item
        DamageEffect(Dice { dice: 4, faces: 4})
        Renderable { glyph: ')', fg_color: color::css::ORANGE }
        TargetingRange(4)
        Animation::Missile { glyph: '~', fg_color: Option::Some(LinearRgba::RED), time_secs: 0.55 }
    }
}

fn use_item(
    e: On<UseItem>,
    q_user: Query<&Name>,
    mut q_target: Query<(&Name, &mut HitPoints)>,
    q_healing_item: Query<(&Name, &HealEffect)>,
    q_damage_item: Query<(&Name, &DamageEffect)>,
    mut q_energy: Query<&mut Energy>,
    mut commands: Commands,
    mut buff: Local<Vec<Entity>>,
) {
    println!("USING ITEM");

    let user = q_user.get(e.user).expect("Unnamed actor used an item");
    let mut energy = q_energy
        .get_mut(e.user)
        .expect("Item user was missing energy component");

    if e.targets.is_empty() {
        buff.push(e.user);
    } else {
        buff.extend(e.targets.iter())
    }

    for tar in buff.iter().cloned() {
        let (tar_name, mut tar_hp) = q_target.get_mut(tar).expect("Invalid target");

        if let Ok((name, effect)) = q_healing_item.get(e.item) {
            let old = tar_hp.current;
            tar_hp.current = (tar_hp.current + effect.0.roll()).min(tar_hp.max);
            let diff = tar_hp.current - old;

            if tar == e.user {
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
                        tar_name.as_str(),
                    )));
                } else {
                    commands.write_message(LogMessage(format!(
                        "{} used {} on {}, it restored {} hit points.",
                        user.as_str(),
                        name.as_str(),
                        tar_name.as_str(),
                        diff
                    )));
                }
            }
            commands.entity(e.item).despawn();
        }

        if let Ok((name, effect)) = q_damage_item.get(e.item) {
            // todo: Need a system to detect when hp is 0 instead of having to send messages manually?
            let old = tar_hp.current;
            tar_hp.current = (tar_hp.current - effect.0.roll()).max(0);
            let diff = old - tar_hp.current;

            if tar == e.user {
                if diff == 0 {
                    commands.write_message(LogMessage(format!(
                        "{} used {}, but it had no effect...",
                        user.as_str(),
                        name.as_str(),
                    )));
                } else {
                    commands.write_message(LogMessage(format!(
                        "{} used {} and lost {} hit points.",
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
                        tar_name.as_str(),
                    )));
                } else {
                    commands.write_message(LogMessage(format!(
                        "{} used {} on {}, it dealt {} damage.",
                        user.as_str(),
                        name.as_str(),
                        tar_name.as_str(),
                        diff
                    )));
                }
            }
            commands.entity(e.item).despawn();
        }
    }
    // let (tar, mut hp) = q_target.get_mut(e.targets).expect("Invalid target");

    energy.0 = 0;
}
