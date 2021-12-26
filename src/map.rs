use std::{ops::{Index, IndexMut}, slice::Iter};

use bevy::{
    math::{IVec2, UVec2},
    prelude::*,
    utils::HashSet,
};
use rand::{prelude::StdRng, Rng};

use crate::{config::MapGenSettings, monster::MonsterBundle, player::PlayerBundle, shapes::Rect};

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

    pub fn len(&self) -> usize {
        (self.size.x * self.size.y) as usize
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
    
    // #[inline]
    // pub fn to_xy(&self, i: usize) -> UVec2 {
    //     let i = i as u32;
    //     let x = i % self.size.x;
    //     let y = i / self.size.x;
    //     UVec2::new(x,y)
    // }

    pub fn is_in_bounds(&self, p: IVec2) -> bool {
        p.cmpge(IVec2::ZERO).all() && p.cmplt(self.size.as_i32()).all()
    }

    pub fn get(&self, p: IVec2) -> MapTile {
        self.tiles[self.to_index(p.as_u32().into())]
    }

    pub fn iter(&self) -> Iter<MapTile> {
        self.tiles.iter()
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

pub struct MapGenEntities {
    pub player: PlayerBundle,
    //pub monsters: Vec<MonsterBundle>,
}

pub struct MapGenerator {
    pub map: Map,
    pub rooms: Vec<Rect>,
}

impl MapGenerator {
    pub fn build(
        commands: &mut Commands,
        settings: MapGenSettings,
        mut rng: StdRng,
        entities: MapGenEntities,
    ) {
        let mut map = Map::with_size(settings.map_size);
        let mut rooms: Vec<Rect> = Vec::with_capacity(50);

        generate_rooms(&mut map, &settings, &mut rng, &mut rooms);

        let map = MapGenerator { map, rooms };

        map.place_player(commands, entities.player);

        let mut placed: HashSet<IVec2> = HashSet::default();

        map.place_monsters(commands, &settings, &mut rng, &mut placed);

        commands.spawn().insert(map.map);
    }

    pub fn place_player(&self, commands: &mut Commands, mut player: PlayerBundle) {
        let p = self.rooms[0].center();
        player.move_bundle.position = p.into();
        commands.spawn().insert_bundle(player);
    }

    pub fn place_monsters(
        &self,
        commands: &mut Commands,
        settings: &MapGenSettings,
        rng: &mut StdRng,
        placed: &mut HashSet<IVec2>,
    ) {
        // The first room is the player's room
        for room in self.rooms.iter().skip(1) {
            let count = rng.gen_range(settings.monsters_per_room.clone());

            for _ in 0..=count {
                for _ in 0..2 {
                    // If the first try fails, try again
                    let p = get_random_ivec(rng, room.min, room.max);

                    if placed.contains(&p) {
                        continue;
                    }

                    let monster_index = rng.gen_range(0..MonsterBundle::max_index());
                    let mut monster = MonsterBundle::get_from_index(monster_index);
                    monster.movable.position = p.into();
                    placed.insert(p);

                    commands.spawn_bundle(monster);

                    break;
                }
            }
        }
    }
}

fn get_random_ivec(rng: &mut StdRng, min: IVec2, max: IVec2) -> IVec2 {
    let p_x = rng.gen_range(min.x..max.x);
    let p_y = rng.gen_range(min.y..max.y);

    IVec2::new(p_x, p_y)
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

        let new_room = Rect::from_position_size((x as i32, y as i32), (w as i32, h as i32));

        // //println!("Creating room {}", new_room);

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

            if !rooms.is_empty() {
                let prev_room = &rooms[rooms.len() - 1];
                build_tunnels_between_rooms(map, rng, prev_room, &new_room);
            }

            rooms.push(new_room);
        }
    }
}

fn build_room(map: &mut Map, room: &Rect) {
    for pos in room.iter() {
        map[pos.as_u32().into()] = MapTile::Floor;
    }
}

fn build_tunnels_between_rooms(map: &mut Map, rng: &mut StdRng, room_a: &Rect, room_b: &Rect) {
    let (new_x, new_y) = room_b.center().into();
    let (prev_x, prev_y) = room_a.center().into();

    if rng.gen_bool(0.5) {
        build_horizontal_tunnel(map, prev_x, new_x, prev_y);
        build_vertical_tunnel(map, prev_y, new_y, new_x);
    } else {
        build_vertical_tunnel(map, prev_y, new_y, prev_x);
        build_horizontal_tunnel(map, prev_x, new_x, new_y);
    }
}

fn build_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    let min = x1.min(x2);
    let max = x1.max(x2);

    for x in min..=max {
        map[(x as u32, y as u32)] = MapTile::Floor;
    }
}

fn build_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    let min = y1.min(y2);
    let max = y1.max(y2);

    for y in min..=max {
        map[(x as u32, y as u32)] = MapTile::Floor;
    }
}
