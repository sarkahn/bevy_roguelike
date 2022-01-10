use bevy::prelude::*;
use bevy_ascii_terminal::RED;

use crate::{bundle::MovingEntityBundle, map_state::PathBlocker, visibility::MapView, turn_system::{Energy, TakingATurn}, combat::{CombatantBundle, HitPoints, MaxHitPoints, Defense, AttackPower}};

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(monster_ai);
    }
}

#[derive(Component, Default)]
pub struct Monster;

#[derive(Bundle)]
pub struct MonsterBundle {
    #[bundle]
    pub movable: MovingEntityBundle,
    #[bundle]
    pub combatant_bundle: CombatantBundle,
    pub monster: Monster,
    pub name: Name,
    pub blocker: PathBlocker,
    pub vision: MapView,
}

impl MonsterBundle {
    pub fn new_goblin() -> Self {
        MonsterBundle {
            movable: MovingEntityBundle::new(RED, 'g', 15),
            combatant_bundle: CombatantBundle {
                hp: HitPoints(12),
                max_hp: MaxHitPoints(12),
                defense: Defense(0),
                attack: AttackPower(2),
            },
            monster: Default::default(),
            name: Name::new("Goblin"),
            blocker: Default::default(),
            vision: Default::default(),
        }
    }

    pub fn new_orc() -> Self {
        Self {
            movable: MovingEntityBundle::new(RED, 'o', 10),
            combatant_bundle: CombatantBundle {
                hp: HitPoints(18),
                max_hp: MaxHitPoints(18),
                defense: Defense(2),
                attack: AttackPower(3),
            },
            monster: Default::default(),
            name: Name::new("Orc"),
            blocker: Default::default(),
            vision: Default::default(),
        }
    }

    pub fn get_from_index(index: u32) -> MonsterBundle {
        match index {
            0 => MonsterBundle::new_goblin(),
            1 => MonsterBundle::new_orc(),
            _ => MonsterBundle::new_goblin(),
        }
    }

    pub fn max_index() -> u32 {
        2
    }
}

fn monster_ai(
    mut q_monster: Query<&mut Energy, (With<Monster>, With<TakingATurn>)>,
) {
    for mut energy in q_monster.iter_mut() {
        //println!("Monster taking a turn.");
        energy.0 = 0;
    }
}