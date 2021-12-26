use bevy::prelude::*;
use bevy_ascii_terminal::RED;

use crate::bundle::MovingEntityBundle;

#[derive(Bundle)]
pub struct MonsterBundle {
    #[bundle]
    pub movable: MovingEntityBundle,
}

impl MonsterBundle {
    pub fn new_goblin() -> Self {
        MonsterBundle {
            movable: MovingEntityBundle::new(RED, 'g'),
        }
    }

    pub fn new_orc() -> Self {
        Self {
            movable: MovingEntityBundle::new(RED, 'o'),
        }
    }

    pub fn get_from_index(index: u32) -> MonsterBundle {
        match index {
            1 => MonsterBundle::new_goblin(),
            2 => MonsterBundle::new_orc(),
            _ => MonsterBundle::new_goblin(),
        }
    }

    pub fn max_index() -> u32 {
        2
    }
}
