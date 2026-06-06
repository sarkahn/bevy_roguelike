use bevy::prelude::*;
use bevy_ascii_terminal::color::css;

/// Component for tracking entity positions on the map.
#[derive(Component, Debug, Clone, Default)]
pub struct Position(pub IVec2);

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

#[derive(Component, Debug, Clone, Default)]
pub struct TargetingRange(pub i32);

#[derive(Event)]
pub struct BeginTargeting {
    pub source: Entity,
    pub range: i32,
    pub effect_haver: Entity,
}

#[derive(Component, Debug, Clone)]
pub struct Targeting {
    pub source: Entity,
    pub effect_haver: Entity,
    pub source_pos: IVec2,
    pub range: i32,
    pub points: Vec<IVec2>,
}

/// Read by the ui system to print log messages to the terminal.
#[derive(Message)]
pub struct LogMessage(pub String);

/// A message to indicate the game terminal should redraw at the end of the frame
#[derive(Message)]
pub struct Redraw;
