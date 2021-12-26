use bevy::prelude::*;
use bevy_ascii_terminal::{TileColor, BLACK};

use crate::{render::Renderable, movement::{Position, Movement}};


#[derive(Debug, Bundle)]
pub struct MovingEntityBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub movement: Movement,
}

impl MovingEntityBundle {
    pub fn new(fg_color: TileColor, glyph: char) -> Self {
        Self {
            renderable: Renderable {
                fg_color,
                bg_color: BLACK,
                glyph,
            },
            position: Position::default(),
            movement: Movement::default(),
        }
    }
}