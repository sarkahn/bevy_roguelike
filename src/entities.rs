use bevy::prelude::*;
use crate::components::*;
use bevy_ascii_terminal::color::*;

pub fn movable_guy() -> impl Scene {
    bsn! {
        Renderable
        Position
        Movement
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

pub fn goblin() -> impl Scene {
    bsn! {
        #Goblin
        base_monster()
        Renderable { fg_color: css::RED, glyph: 'g' }
        HitPoints { current: 15, max: 15 }
        Strength { current: 1, max: 1 }
        AttackDice { dice: 1, faces: 4 }
        Speed(20)
    }
}

pub fn orc() -> impl Scene {
    bsn! {
        #Orc
        base_monster()
        Renderable { fg_color: css:: RED, glyph: 'o' }
        HitPoints { current: 25, max: 25 }
        Defense { current: 1, max: 1 }
        Strength { current: 3, max: 3 }
        AttackDice { dice: 2, faces: 6 }
        Speed(15)
    }
}

pub fn player() -> impl Scene {
    bsn! {
        #Player
        combat_guy()
        movable_guy()
        Renderable { fg_color: css::WHITE, }
        HitPoints { current: 60, max: 60 }
        AttackDice { dice: 5, faces: 3 }
        ViewRange(5)
        Defense { current: 1, max: 1 }
        Strength { current: 3, max: 3 }
        Speed(25)
        MapView
        MapMemory
    }
}

