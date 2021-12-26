use bevy::prelude::*;

use crate::map::*;
use serde::Deserialize;

/// Component for tracking entity positions on the map.
#[derive(Debug, Deserialize, Default)]
pub struct Position(pub (i32, i32));

/// Component for tracking entity movement.
#[derive(Debug, Deserialize, Default)]
pub struct Movement(pub (i32, i32));

impl From<(i32, i32)> for Position {
    fn from(p: (i32, i32)) -> Self {
        Position(p)
    }
}

impl From<IVec2> for Position {
    fn from(v: IVec2) -> Self {
        Position(v.into())
    }
}

/// Plugin for movement related systems.
pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(movement_system.system());
    }
}

fn movement_system(
    q_map: Query<&Map>,
    mut q_move: Query<(&mut Position, &mut Movement), Changed<Movement>>,
) {
    let map = match q_map.single() {
        Ok(map) => map,
        Err(_) => return,
    };

    for (mut pos, mut movement) in q_move.iter_mut() {
        let p_vec = IVec2::from(pos.0);
        let m_vec = IVec2::from(movement.0);
        let next = p_vec + m_vec;

        if map[next.as_u32().into()] == MapTile::Floor {
            pos.0 = next.into();
        }

        movement.0 = (0, 0);
    }
}

#[cfg(test)]
mod test {
    use bevy::prelude::*;

    use crate::map::{Map, MapTile};

    use super::{movement_system, Movement, Position};

    #[test]
    fn can_move_into_floors() {
        let mut world = World::default();

        let mut map = Map::with_size((10, 10));
        map[(0, 0)] = MapTile::Floor;
        map[(1, 0)] = MapTile::Floor;
        map[(2, 0)] = MapTile::Wall;

        world.spawn().insert(map);

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(movement_system.system());

        let mover = world
            .spawn()
            .insert(Position((0, 0)))
            .insert(Movement((1, 0)))
            .id();

        update_stage.run(&mut world);

        let pos = world.get::<Position>(mover).unwrap();
        let movement = world.get::<Movement>(mover).unwrap();

        assert_eq!(pos.0, (1, 0));
        assert_eq!(movement.0, (0, 0));
    }

    #[test]
    fn cant_move_into_walls() {
        let mut world = World::default();

        let mut map = Map::with_size((10, 10));
        map[(0, 0)] = MapTile::Floor;
        map[(1, 0)] = MapTile::Wall;

        world.spawn().insert(map);

        let mut update_stage = SystemStage::parallel();
        update_stage.add_system(movement_system.system());

        let mover = world
            .spawn()
            .insert(Position((0, 0)))
            .insert(Movement((1, 0)))
            .id();

        {
            let mut movement = world.get_mut::<Movement>(mover).unwrap();
            movement.0 = (1, 0);
        }

        update_stage.run(&mut world);

        let pos = world.get::<Position>(mover).unwrap();
        let movement = world.get::<Movement>(mover).unwrap();

        assert_eq!(movement.0, (0, 0));
        assert_eq!(pos.0, (0, 0));
    }
}
