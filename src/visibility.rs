use bevy::{math::IVec2, prelude::*};
use sark_grids::Grid;

use crate::{
    map::{Map, MapTile},
    movement::Position,
};

use adam_fov_rs::{self, fov};

pub const VIEW_SYSTEM_LABEL: &str = "VIEW_SYSTEM";

pub struct VisiblityPlugin;

impl Plugin for VisiblityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(view_system.label(VIEW_SYSTEM_LABEL))
            .add_system(view_memory_system.before(VIEW_SYSTEM_LABEL));
    }
}

#[derive(Component, Debug, Default)]
pub struct MapMemory(pub Vec<bool>);

#[derive(Component, Debug, Default)]
pub struct MapView(pub Grid<bool>);

#[derive(Component, Debug, Default)]
pub struct ViewRange(pub u32);

pub struct VisibilityMap<'a> {
    map: &'a Map,
    view: &'a mut MapView,
    memory: Option<&'a mut MapMemory>,
}

impl<'a> adam_fov_rs::VisibilityMap for VisibilityMap<'a> {
    fn is_opaque(&self, p: IVec2) -> bool {
        if !self.map.0.is_in_bounds(p) {
            return true;
        }
        self.map.0[p] == MapTile::Wall
    }

    fn is_in_bounds(&self, p: IVec2) -> bool {
        self.map.0.is_in_bounds(p)
    }

    fn set_visible(&mut self, p: IVec2) {
        let i = self.map.0.pos_to_index(p.into());

        self.view.0[i] = true;

        if let Some(memory) = &mut self.memory {
            memory.0[i] = true;
        }
    }

    fn dist(&self, a: IVec2, b: IVec2) -> f32 {
        a.as_vec2().distance(b.as_vec2())
    }
}

#[allow(clippy::type_complexity)]
fn view_system(
    mut q_view: Query<
        (&mut MapView, &Position, &ViewRange),
        (Changed<Position>, Without<MapMemory>),
    >,
    q_map: Query<&Map>,
) {
    if let Ok(map) = q_map.get_single() {
        for (mut view, pos, range) in q_view.iter_mut() {
            //println!("Updating mapview");
            let view_vec = &mut view.0;

            if view_vec.len() != map.0.len() {
                *view_vec = Grid::default(map.0.size().into());//vec![false; map.0.len()];
            }

            for b in view_vec.iter_mut() {
                *b = false;
            }

            let mut fov_map = VisibilityMap {
                map,
                view: &mut view,
                memory: None,
            };

            fov::compute(pos.0.into(), range.0 as i32, &mut fov_map);
        }
    }
}

fn view_memory_system(
    mut q_view: Query<(&mut MapView, &mut MapMemory, &Position, &ViewRange), Changed<Position>>,
    q_map: Query<&Map>,
) {
    if let Ok(map) = q_map.get_single() {
        for (mut view, mut memory, pos, range) in q_view.iter_mut() {
            //println!("Updating mapview");
            let view_vec = &mut view.0;

            if view_vec.len() != map.0.len() {
                *view_vec = Grid::default(map.0.size().into());//vec![false; map.0.len()];
            }

            // Reset our view but not our memory
            for b in view_vec.iter_mut() {
                *b = false;
            }

            let mem_vec = &mut memory.0;

            if mem_vec.len() != map.0.len() {
                *mem_vec = vec![false; map.0.len()];
            }

            let mut fov_map = VisibilityMap {
                map,
                view: &mut view,
                memory: Some(&mut memory),
            };

            fov::compute(pos.0.into(), range.0 as i32, &mut fov_map);
        }
    }
}
