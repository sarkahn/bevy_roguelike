use bevy::prelude::*;

use crate::{
    map::{Map, MapTile},
    movement::{Position, ACTOR_MOVE_SYSTEM_LABEL},
};

pub const UPDATE_MAP_STATE_SYSTEM_LABEL: &str = "update_map_state_system";

pub struct MapStatePlugin;

impl Plugin for MapStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(
            update_map_state_system
                .label(UPDATE_MAP_STATE_SYSTEM_LABEL)
                .after(ACTOR_MOVE_SYSTEM_LABEL),
        )
        .init_resource::<MapObstacles>()
        .init_resource::<MapActors>();
    }
}

#[derive(Component, Default)]
pub struct PathBlocker;

#[derive(Component, Default)]
pub struct MapObstacles(pub Vec<bool>);
#[derive(Component, Default)]
pub struct MapActors(pub Vec<Option<Entity>>);

fn update_map_state_system(
    q_moved_actors: Query<&Position, (With<PathBlocker>, Changed<Position>)>,
    q_all_actors: Query<(Entity, &Position), With<PathBlocker>>,
    q_changed_map: Query<&Map, Changed<Map>>,
    mut blockers: ResMut<MapObstacles>,
    mut entities: ResMut<MapActors>,
) {
    let actors_moved = q_moved_actors.iter().next().is_some();
    let map_changed = q_changed_map.iter().next().is_some();

    if !actors_moved && !map_changed {
        return;
    }

    if let Ok(map) = q_changed_map.get_single() {
        if blockers.0.len() != map.0.len() {
            blockers.0 = vec![false; map.0.len()];
        }

        if entities.0.len() != map.0.len() {
            entities.0 = vec![None; map.0.len()];
        }

        for (i, tile) in map.0.iter().enumerate() {
            blockers.0[i] = *tile == MapTile::Wall;
        }

        // Clear entity state
        for entry in entities.0.iter_mut() { 
            *entry = None;
        }
        for (entity, pos) in q_all_actors.iter() {
            let i = map.0.pos_to_index(pos.0.into());
            blockers.0[i] = true;
            entities.0[i] = Some(entity);
        }
    }
}

// fn should_update(
//     q_actors: Query<&Position, (With<PathBlocker>, Changed<Position>)>,
//     q_map: Query<&Map, Changed<Map>>,
// ) -> ShouldRun {
//     let actors_moved = q_actors.iter().next().is_some();
//     let map_changed = q_map.iter().next().is_some();

//     if map_changed || actors_moved {
//         return ShouldRun::Yes;
//     }

//     ShouldRun::No
// }
