use bevy::prelude::*;

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
    pub max: i32
}

#[derive(Component, Clone, Default)]
pub struct AttackDice {
    pub dice: i32,
    pub faces: i32,
}

/// Component for tracking entity positions on the map.

#[derive(Component, Clone, Default)]
pub struct Position(pub IVec2);

/// Component for tracking entity movement.

#[derive(Component, Clone, Default)]
pub struct Movement(pub IVec2);

/// When an actor's energy reaches or exceeds 100, it will be given a turn.
#[derive(Default, Debug, Component, Clone)]
pub struct Energy(pub i32);

/// Determines how frequently an actor gets to take their turn,
/// relative to other actors.
#[derive(Debug, Component, Clone, Default)]
pub struct Speed(pub i32);

/// A tag for actors that can perform actions and take turns.
#[derive(Default, Debug, Component, Clone)]
pub struct Actor;

/// A tag for actors that can perform actions and take turns.
#[derive(Default, Debug, Component, Clone)]
pub struct Player;

#[derive(Default, Debug, Component, Clone)]
pub struct Monster;

#[derive(Default, Debug, Component, Clone,)]
pub struct Renderable {
    pub fg_color: Color,
    pub bg_color: Color,
    pub glyph: char,
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
