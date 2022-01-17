use bevy::prelude::*;

use crate::map::*;
use serde::Deserialize;

/// Component for tracking entity positions on the map.
#[derive(Component, Debug, Deserialize, Default)]
pub struct Position(pub IVec2);

/// Component for tracking entity movement.
#[derive(Component, Debug, Deserialize, Default)]
pub struct Movement(pub IVec2);

impl From<[i32;2]> for Position {
    fn from(p: [i32;2]) -> Self {
        Position(IVec2::from(p))
    }
}

impl From<IVec2> for Position {
    fn from(v: IVec2) -> Self {
        Position(v.into())
    }
}
