use bevy::{platform::collections::HashMap, prelude::*};
use sark_pathfinding::{PathMap2d, grid::SizedGrid};

use crate::{
    GAME_SIZE,
    components::Position,
    map::{Map, MapTile},
    xy_to_index,
};

pub struct MapStatePlugin;

impl Plugin for MapStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, update_map_state_system);
    }
}

/// An entity that blocks pathfinding.
#[derive(Component, Default, Clone, Debug)]
pub struct PathBlocker;

#[derive(Resource, Default, Clone, Debug)]
pub struct PathingData(pub PathMap2d);

#[derive(Resource, Default, Clone, Debug, Deref, DerefMut)]
pub struct MapActors(pub HashMap<IVec2, Entity>);

fn update_map_state_system(
    q_moved_actors: Query<&Position, (With<PathBlocker>, Changed<Position>)>,
    q_blockers: Query<(Entity, &Position), With<PathBlocker>>,
    q_changed_map: Query<&Map, Changed<Map>>,
    q_map: Query<&Map>,
    mut pathing: ResMut<PathingData>,
    mut entities: ResMut<MapActors>,
) {
    if q_moved_actors.is_empty()
        && q_changed_map.is_empty()
        && !pathing.is_changed()
        && !entities.is_changed()
    {
        return;
    }

    if let Ok(map) = q_map.single() {
        if pathing.0.area() != map.0.len() {
            pathing.0.obstacles.resize(GAME_SIZE);
        }

        if entities.0.len() != map.0.len() {
            entities.0 = HashMap::new();
        }

        // Populate blockers from map tiles first
        for (i, t) in map.0.iter().enumerate() {
            pathing.0.obstacles.set_index(i, matches!(t, MapTile::Wall));
        }
        entities.0.clear();

        for (entity, pos) in q_blockers.iter() {
            let i = xy_to_index(pos.0);
            pathing.0.obstacles.set_index(i, true);
            entities.0.insert(pos.0, entity);
        }
    }
}
