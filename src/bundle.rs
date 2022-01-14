use bevy::prelude::*;

use crate::{
    movement::{Movement, Position},
    render::Renderable, turn_system::{Speed, Energy, Actor},
};

#[derive(Debug, Bundle)]
pub struct MovingEntityBundle {
    pub renderable: Renderable,
    pub position: Position,
    pub movement: Movement,
    pub energy: Energy,
    pub speed: Speed,
    pub actor: Actor,
}

impl MovingEntityBundle {
    pub fn new(fg_color: Color, glyph: char, speed: i32) -> Self {
        Self {
            renderable: Renderable {
                fg_color,
                bg_color: Color::BLACK,
                glyph,
            },
            speed: Speed(speed),
            position: Position::default(),
            movement: Movement::default(),
            energy: Default::default(),
            actor: Default::default(),
        }
    }
}
