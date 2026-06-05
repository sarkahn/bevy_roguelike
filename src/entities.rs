use crate::{
    combat::{Defense, Dice, HitPoints, Strength},
    components::{Position, Renderable},
    map_state::PathBlocker,
    monster::Monster,
    player::Player,
    turn_system::{Actor, Energy, Speed},
    visibility::{MapMemory, MapView, ViewRange},
};
use bevy::prelude::*;
use bevy_ascii_terminal::color::*;

pub fn movable_guy() -> impl Scene {
    bsn! {
        Renderable
        Position
        Energy
        Speed
        Actor
    }
}

pub fn combat_guy() -> impl Scene {
    bsn! {
        HitPoints
        Defense
        Strength
    }
}

pub fn base_monster() -> impl Scene {
    bsn! {
        movable_guy()
        combat_guy()
        MapView
        ViewRange(4)
        PathBlocker
        Monster
    }
}

pub fn goblin(pos: IVec2) -> impl Scene {
    bsn! {
        #Goblin
        base_monster()
        Renderable { glyph: 'g', fg_color: css::RED }
        HitPoints { current: 15, max: 15 }
        Strength { current: 1, max: 1 }
        Dice { dice: 1, faces: 4 }
        Position(pos)
        Speed(20)
    }
}

pub fn orc(pos: IVec2) -> impl Scene {
    bsn! {
        #Orc
        base_monster()
        Renderable { glyph: 'o', fg_color: css:: RED }
        HitPoints { current: 25, max: 25 }
        Defense { current: 1, max: 1 }
        Strength { current: 3, max: 3 }
        Dice { dice: 2, faces: 6 }
        Speed(15)
        Position(pos)
    }
}

pub fn player(pos: IVec2) -> impl Scene {
    bsn! {
        #Player
        Player
        combat_guy()
        movable_guy()
        Renderable { glyph: '@', fg_color: css::WHITE }
        HitPoints { current: 60, max: 60 }
        Dice { dice: 5, faces: 3 }
        Defense { current: 1, max: 1 }
        Strength { current: 3, max: 3 }
        Speed(25)
        ViewRange(5)
        MapView
        MapMemory
        Position(pos)
    }
}
