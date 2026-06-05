use bevy::{math::IVec2, prelude::*};

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

#[derive(Component, Debug, Default, Clone)]
pub struct MapMemory(pub Vec<bool>);

#[derive(Component, Debug, Default, Clone)]
pub struct MapView(pub Vec<bool>);

#[derive(Component, Debug, Default, Clone)]
pub struct ViewRange(pub u32);

#[allow(clippy::type_complexity)]
fn view_system(
    mut q_view: Query<
        (&mut MapView, &Position, &ViewRange),
        (Changed<Position>, Without<MapMemory>),
    >,
    q_map: Query<&Map>,
) {
    if let Ok(map) = q_map.single() {
        for (mut view, pos, range) in q_view.iter_mut() {
            if view.0.len() != map.0.len() {
                (*view).0 = vec![false; map.0.len()];
            }

            view.0.fill(false);

            // NOTE: Assumes map size = view size = GAME_SIZE
            let blocks_vision = |p: IVec2| map.0[xy_to_index(p)] == MapTile::Wall;
            let mark_visible = |p: IVec2| view.0[xy_to_index(p)] = true;

            compute_fov(
                pos.0,
                range.0 as usize,
                GAME_SIZE,
                blocks_vision,
                mark_visible,
            );
        }
    }
}

fn view_memory_system(
    mut q_view: Query<(&mut MapView, &mut MapMemory, &Position, &ViewRange), Changed<Position>>,
    q_map: Query<&Map>,
) {
    if let Ok(map) = q_map.single() {
        for (mut view, mut memory, pos, range) in q_view.iter_mut() {
            if view.0.len() != map.0.len() {
                (*view).0 = vec![false; map.0.len()];
            }
            if memory.0.len() != map.0.len() {
                (*memory).0 = vec![false; map.0.len()];
            }

            // Reset view but not memory
            view.0.fill(false);

            // NOTE: Assumes map size = view size = GAME_SIZE
            let blocks_vision = |p: IVec2| map.0[xy_to_index(p)] == MapTile::Wall;
            let mark_visible = |p: IVec2| {
                let i = xy_to_index(p);
                view.0[i] = true;
                memory.0[i] = true;
            };

            compute_fov(
                pos.0,
                range.0 as usize,
                GAME_SIZE,
                blocks_vision,
                mark_visible,
            );
        }
    }
}
