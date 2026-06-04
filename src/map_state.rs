use bevy::prelude::*;

use crate::{
    components::Position, map::{Map, MapTile}, xy_to_index
};


pub struct MapStatePlugin;

impl Plugin for MapStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<MapObstacles>()
        .init_resource::<MapActors>()
        .add_systems(PreUpdate, update_map_state_system);
    }
}

/// An entity that blocks pathfinding.
#[derive(Component, Default)]
pub struct PathBlocker;

#[derive(Resource, Default, Clone, Debug)]
pub struct MapObstacles(pub Vec<bool>);

#[derive(Resource, Default, Clone, Debug)]
pub struct MapActors(pub Vec<Option<Entity>>);

fn update_map_state_system(
    q_moved_actors: Query<&Position, (With<PathBlocker>, Changed<Position>)>,
    q_blockers: Query<(Entity, &Position), With<PathBlocker>>,
    q_changed_map: Query<&Map, Changed<Map>>,
    q_map: Query<&Map>,
    mut blockers: ResMut<MapObstacles>,
    mut entities: ResMut<MapActors>,
) {

    if q_moved_actors.is_empty() && q_changed_map.is_empty()
    && !blockers.is_changed() && !entities.is_changed() {
        return;
    }

    if let Ok(map) = q_map.single() {
        if blockers.0.len() != map.0.len() {
            blockers.0 = vec![false; map.0.len()]
        }

        if entities.0.len() != map.0.len() {
            entities.0 = vec![None; map.0.len()];
        }

        // Populate blockers from map tiles first
        for (i, t) in map.0.iter().enumerate() {
            blockers.0[i] = matches!(t, MapTile::Wall);
        }
        entities.0.fill(None);


        for (entity, pos) in q_blockers.iter() {
            let i = xy_to_index(pos.0);
            blockers.0[i] = true;
            entities.0[i] = Some(entity);
        }
    }
}