use bevy::{math::IVec2, prelude::*};
use sark_pathfinding::bit_grid::BitGrid;

use crate::{
    GAME_SIZE,
    components::*,
    map::{Map, MapTile},
    xy_to_index,
};

use adam_fov_rs::*;

pub struct VisiblityPlugin;

impl Plugin for VisiblityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (view_system, view_memory_system));
    }
}

#[derive(Component, Debug, Default, Clone, Deref, DerefMut)]
pub struct MapMemory(pub BitGrid);

#[derive(Component, Debug, Default, Clone)]
pub struct MapView {
    pub grid: BitGrid,
    pub range: i32,
}

impl MapView {
    pub fn iter_visible_indices(&self) -> impl Iterator<Item = usize> {
        self.grid
            .bits()
            .iter()
            .enumerate()
            .filter_map(|(i, b)| b.then(|| i))
    }

    pub fn iter_visible_points(&self) -> impl Iterator<Item = IVec2> {
        self.grid.iter_xy().filter_map(|(p, b)| b.then(|| p))
    }
}

#[allow(clippy::type_complexity)]
fn view_system(
    mut q_view: Query<(&mut MapView, &Position), (Changed<Position>, Without<MapMemory>)>,
    q_map: Query<&Map>,
) {
    if let Ok(map) = q_map.single() {
        // TODO: Don't assume map size = view size = game size
        for (mut view, pos) in q_view.iter_mut() {
            let range = view.range as usize;

            if view.grid.len() != map.0.len() {
                (*view).grid = BitGrid::new(GAME_SIZE);
            }

            view.grid.set_all(false);

            let blocks_vision = |p: IVec2| map.0[xy_to_index(p)] == MapTile::Wall;
            let mark_visible = |p: IVec2| view.grid.set(p, true);

            compute_fov(pos.0, range, GAME_SIZE, blocks_vision, mark_visible);
        }
    }
}

fn view_memory_system(
    mut q_view: Query<(&mut MapView, &mut MapMemory, &Position), Changed<Position>>,
    q_map: Query<&Map>,
) {
    if let Ok(map) = q_map.single() {
        // TODO: Don't assume map size = view size = game size
        for (mut view, mut memory, pos) in q_view.iter_mut() {
            let range = view.range as usize;

            if view.grid.len() != map.0.len() {
                view.grid = BitGrid::new(GAME_SIZE);
            }
            if memory.len() != map.0.len() {
                memory.0 = BitGrid::new(GAME_SIZE);
            }

            // Reset view but not memory
            view.grid.set_all(false);

            // NOTE: Assumes map size = view size = GAME_SIZE
            let blocks_vision = |p: IVec2| map.0[xy_to_index(p)] == MapTile::Wall;
            let mark_visible = |p: IVec2| {
                let i = xy_to_index(p);
                view.grid.set_index(i, true);
                memory.set_index(i, true);
            };

            compute_fov(pos.0, range, GAME_SIZE, blocks_vision, mark_visible);
        }
    }
}
