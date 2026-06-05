use bevy::prelude::*;
use bevy_ascii_terminal::color::css;

#[derive(Component, Clone, Default)]
pub struct HitPoints {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Default)]
pub struct Defense {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Default)]
pub struct Strength {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Default)]
pub struct AttackDice {
    pub dice: i32,
    pub faces: i32,
}

impl AttackDice {
    pub fn roll(&self) -> i32 {
        let mut i: i32 = 0;
        for _ in 0..self.dice {
            i += rand::random_range(0..self.faces);
        }
        i
    }
}

/// Component for tracking entity positions on the map.
#[derive(Component, Clone, Default)]
pub struct Position(pub IVec2);

/// Component for tracking entity movement.
#[derive(Component, Clone, Default)]
pub struct Movement(pub IVec2);

#[derive(Default, Debug, Component, Clone)]
pub struct Monster;

#[derive(Debug, Component, Clone)]
pub struct Renderable {
    pub fg_color: LinearRgba,
    pub bg_color: LinearRgba,
    pub glyph: char,
}

impl Default for Renderable {
    fn default() -> Self {
        Self {
            fg_color: css::WHITE,
            bg_color: css::BLACK,
            glyph: ' ',
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct MapMemory(pub Vec<bool>);

#[derive(Component, Debug, Default, Clone)]
pub struct MapView(pub Vec<bool>);

#[derive(Component, Debug, Default, Clone)]
pub struct ViewRange(pub u32);

/// An entity that blocks pathfinding.
#[derive(Component, Default, Clone)]
pub struct PathBlocker;
