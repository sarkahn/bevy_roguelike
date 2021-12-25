use std::ops::{Index, IndexMut, Range};

use bevy::math::UVec2;
use rand::{
    distributions::Uniform,
    prelude::{Distribution, StdRng, ThreadRng},
    Rng,
};

use crate::{config::MapGenSettings, shapes::Rect};

/// A tile on the [Map].
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum MapTile {
    Wall,
    Floor,
}

impl Default for MapTile {
    fn default() -> Self {
        Self::Wall
    }
}

/// Map of [MapTile].
pub struct Map {
    tiles: Vec<MapTile>,
    size: UVec2,
}

impl Map {
    pub fn with_size(size: (u32, u32)) -> Self {
        let (width, height) = size;
        let len = (width * height) as usize;
        Map {
            tiles: vec![MapTile::default(); len],
            size: UVec2::from(size),
        }
    }

    pub fn width(&self) -> u32 {
        self.size.x
    }

    pub fn height(&self) -> u32 {
        self.size.y
    }

    pub fn size(&self) -> (u32, u32) {
        self.size.into()
    }

    // #[inline]
    // pub fn to_xy(&self, index: usize) -> (u32,u32) {
    //     let index = index as u32;
    //     (index % self.size.x, index / self.size.y)
    // }

    #[inline]
    pub fn to_index(&self, xy: (u32, u32)) -> usize {
        let (x, y) = xy;
        (y * self.size.x + x) as usize
    }
}

impl Index<(u32, u32)> for Map {
    type Output = MapTile;

    fn index(&self, pos: (u32, u32)) -> &Self::Output {
        &self.tiles[self.to_index(pos)]
    }
}

impl IndexMut<(u32, u32)> for Map {
    fn index_mut(&mut self, pos: (u32, u32)) -> &mut Self::Output {
        let i = self.to_index(pos);
        &mut self.tiles[i]
    }
}

pub struct MapGenerator {
    pub map: Map,
    pub rooms: Vec<Rect>,
}

impl MapGenerator {
    pub fn build(settings: MapGenSettings, mut rng: StdRng) -> Self {
        let mut map = Map::with_size(settings.map_size);
        let mut rooms: Vec<Rect> = Vec::with_capacity(50);

        generate_rooms(&mut map, &settings, &mut rng, &mut rooms);

        MapGenerator { map, rooms }
    }
}

fn generate_rooms(
    map: &mut Map,
    settings: &MapGenSettings,
    rng: &mut StdRng,
    rooms: &mut Vec<Rect>,
) {
    for _ in 0..settings.iterations {
        let w = rng.gen_range(settings.room_size.clone());
        let h = rng.gen_range(settings.room_size.clone());

        let x = rng.gen_range(1..map.size.x - w - 1);
        let y = rng.gen_range(1..map.size.y - h - 1);

        let new_room = Rect::from_position_size((x, y), (w, h));

        //println!("Creating room {}", new_room);

        let mut ok = true;

        for room in rooms.iter() {
            if new_room.overlaps(room) {
                //println!("New room overlaps {}!", room);
                ok = false;
                break;
            }
        }

        if ok {
            //println!("Building new room!");
            build_room(map, &new_room);

            if rooms.len() > 0 {
                let prev_room = &rooms[rooms.len() - 1];
                build_tunnels_between_rooms(map, rng, prev_room, &new_room);
            }

            rooms.push(new_room);
        }
    }
}

fn build_room(map: &mut Map, room: &Rect) {
    for pos in room.iter() {
        map[pos] = MapTile::Floor;
    }
}

fn build_tunnels_between_rooms(map: &mut Map, rng: &mut StdRng, room_a: &Rect, room_b: &Rect) {
    let (new_x, new_y) = room_b.center();
    let (prev_x, prev_y) = room_a.center();

    if rng.gen_bool(0.5) {
        build_horizontal_tunnel(map, prev_x, new_x, prev_y);
        build_vertical_tunnel(map, prev_y, new_y, new_x);
    } else {
        build_vertical_tunnel(map, prev_y, new_y, prev_x);
        build_horizontal_tunnel(map, prev_x, new_x, new_y);
    }
}

fn build_horizontal_tunnel(map: &mut Map, x1: u32, x2: u32, y: u32) {
    let min = x1.min(x2);
    let max = x1.max(x2);

    for x in min..=max {
        map[(x, y)] = MapTile::Floor;
    }
}

fn build_vertical_tunnel(map: &mut Map, y1: u32, y2: u32, x: u32) {
    let min = y1.min(y2);
    let max = y1.max(y2);

    for y in min..=max {
        map[(x, y)] = MapTile::Floor;
    }
}
