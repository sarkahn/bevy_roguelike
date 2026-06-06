use bevy::prelude::*;
use bevy_ascii_terminal::color::css;

/// Component for tracking entity positions on the map.
#[derive(Component, Clone, Default)]
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

/// Read by the ui system to print log messages to the terminal.
#[derive(Message)]
pub struct LogMessage(pub String);

/// A message to indicate the game terminal should redraw at the end of the frame
#[derive(Message)]
pub struct Redraw;
