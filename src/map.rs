use std::ops::Range;

use anyhow::Result;
use bevy::{math::IVec2, prelude::*};
use rand::RngExt;

use crate::{GAME_SIZE, xy_to_index};

const MAX_PLACE_ATTEMPTS: u32 = 10;

pub struct MapGenSettings {
    seed: u64,
    iterations: u32,
    map_size: UVec2,
    room_size: Range<u32>,
    monsters_per_room: Range<u32>,
    items_per_room: Range<u32>,
}

impl Default for MapGenSettings {
    fn default() -> Self {
        Self {
            seed: Default::default(),
            iterations: 15,
            map_size: GAME_SIZE,
            room_size: 3..15,
            monsters_per_room: 0..4,
            items_per_room: 0..2,
        }
    }
}

/// A tile on the [Map].
#[derive(Eq, PartialEq, Clone, Copy, Default)]
pub enum MapTile {
    #[default]
    Wall,
    Floor,
}

#[derive(Component)]
pub struct Map(pub Vec<MapTile>);

pub struct MapData {
    pub map: Map,
    pub rooms: Vec<IRect>,
    pub entities: Vec<(IVec2, char)>,
}

pub fn build(settings: &MapGenSettings) -> Result<MapData> {
    let tile_count = GAME_SIZE.element_product() as usize;
    let mut map = MapData {
        map: Map(vec![MapTile::Wall; tile_count]),
        rooms: Vec::new(),
        entities: Vec::new(),
    };

    generate_rooms(&mut map, settings);
    place_player(&mut map);
    place_monsters(settings, &mut map);
    place_items(settings, &mut map);

    Ok(map)
}

fn generate_rooms(map: &mut MapData, settings: &MapGenSettings) {
    let mut rng = rand::rng();
    for _ in 0..settings.iterations {
        let w = rng.random_range(settings.room_size.clone());
        let h = rng.random_range(settings.room_size.clone());

        let p = IVec2::new(
            rand::random_range(1..GAME_SIZE.x - w - 2) as i32,
            rand::random_range(1..GAME_SIZE.y - h - 2) as i32,
        );

        let tr = p + IVec2::new(w as i32 - 1, h as i32 - 1);
        let new_room = IRect::from_corners(p, tr);

        if !map.rooms.iter().any(|r| overlaps(*r, new_room)) {
            //println!("Building new room!");
            place_room(&mut map.map, &new_room);

            if !map.rooms.is_empty() {
                let prev_room = &map.rooms[map.rooms.len() - 1];
                place_tunnels_between_rooms(&mut map.map, prev_room, &new_room);
            }

            map.rooms.push(new_room);
        }
    }
}

fn place_room(map: &mut Map, room: &IRect) {
    for p in iter_room_points(*room) {
        let i = xy_to_index(p);
        map.0[i] = MapTile::Floor;
    }
}

fn place_tunnels_between_rooms(map: &mut Map, room_a: &IRect, room_b: &IRect) {
    let (new_x, new_y) = room_b.center().into();
    let (prev_x, prev_y) = room_a.center().into();
    if rand::random_bool(0.5) {
        place_hor_tunnel(map, prev_x, new_x, prev_y);
        place_ver_tunnel(map, prev_y, new_y, new_x);
    } else {
        place_ver_tunnel(map, prev_y, new_y, prev_x);
        place_hor_tunnel(map, prev_x, new_x, new_y);
    }
}

fn place_hor_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    let min = x1.min(x2);
    let max = x1.max(x2);

    for x in min..=max {
        let i = xy_to_index([x, y]);
        map.0[i] = MapTile::Floor;
    }
}

fn place_ver_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    let min = y1.min(y2);
    let max = y1.max(y2);

    for y in min..=max {
        let i = xy_to_index([x, y]);
        map.0[i] = MapTile::Floor;
    }
}

fn place_player(map: &mut MapData) {
    let r = map
        .rooms
        .first()
        .expect("Attempting to place player but there's no rooms");
    let p = random_rect_point(*r);
    map.entities.push((p, '@'));
}

fn place_monsters(settings: &MapGenSettings, map: &mut MapData) {
    // The player starts in the first room
    for r in map.rooms.iter().skip(1) {
        for _ in 0..rand::random_range(settings.monsters_per_room.clone()) as i32 {
            let mut tries = 0;
            let mut p = random_rect_point(*r);
            while map.entities.iter().any(|v| v.0 == p) && tries < MAX_PLACE_ATTEMPTS {
                p = random_rect_point(*r);
                tries += 1;
            }
            let monster = if rand::random_bool(0.5) { 'g' } else { 'o' };
            map.entities.push((p, monster));
        }
    }
}

fn place_items(settings: &MapGenSettings, map: &mut MapData) {
    for r in map.rooms.iter() {
        for _ in 0..rand::random_range(settings.items_per_room.clone()) {
            let mut tries = 0;
            let mut p = random_rect_point(*r);
            while map.entities.iter().any(|v| v.0 == p) && tries < MAX_PLACE_ATTEMPTS {
                p = random_rect_point(*r);
                tries += 1;
            }
            let item = if rand::random_bool(0.5) { '¡' } else { ')' };
            map.entities.push((p, item));
        }
    }
}

fn random_rect_point(r: IRect) -> IVec2 {
    let x = rand::random_range(r.min.x..=r.max.x);
    let y = rand::random_range(r.min.y..=r.max.y);
    IVec2::new(x, y)
}

fn overlaps(l: IRect, r: IRect) -> bool {
    l.contains(r.min) || l.contains(r.max)
}

fn iter_room_points(r: IRect) -> impl Iterator<Item = IVec2> {
    (r.min.y..=r.max.y).flat_map(move |y| (r.min.x..=r.max.x).map(move |x| IVec2::new(x, y)))
}
