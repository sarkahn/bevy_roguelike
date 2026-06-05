use bevy::prelude::*;
use bevy_ascii_terminal::color::css;

/// Component for tracking entity positions on the map.
#[derive(Component, Clone, Default)]
pub struct Position(pub IVec2);

/// Component for tracking entity movement.
#[derive(Component, Clone, Default)]
pub struct Movement(pub IVec2);

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
