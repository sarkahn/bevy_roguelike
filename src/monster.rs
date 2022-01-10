use bevy::prelude::*;
use bevy_ascii_terminal::RED;

use crate::{bundle::MovingEntityBundle, map_state::PathBlocker};

#[derive(Component, Default)]
pub struct Monster;

#[derive(Bundle)]
pub struct MonsterBundle {
    #[bundle]
    pub movable: MovingEntityBundle,
    pub monster: Monster,
    pub name: Name,
    pub blocker: PathBlocker,
}

impl MonsterBundle {
    pub fn new_goblin() -> Self {
        MonsterBundle {
            movable: MovingEntityBundle::new(RED, 'g'),
            monster: Default::default(),
            name: Name::new("Goblin"),
            blocker: Default::default(),
        }
    }

    pub fn new_orc() -> Self {
        Self {
            movable: MovingEntityBundle::new(RED, 'o'),
            monster: Default::default(),
            name: Name::new("Orc"),
            blocker: Default::default(),
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
