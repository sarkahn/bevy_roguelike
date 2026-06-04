use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::{
    GAME_SIZE, GameTerminal, Reset, components::*, index_to_xy, map::{Map, MapTile}, player::Player, xy_to_index
    // movement::Position,
    // player::Player,
    // visibility::{MapMemory, MapView}, GameTerminal, combat::ActorKilledEvent,
};

pub const WALL_COLOR: LinearRgba = LinearRgba{ red:0.866, green:0.866, blue:0.882, alpha: 1.0};
pub const FLOOR_COLOR: LinearRgba = LinearRgba{ red:0.602, green:0.462, blue:0.325, alpha: 1.0};


/// Plugin managing game rendering systems
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Last, render.run_if(should_render));
    }
}

fn render(
    q_entities: Query<(&Renderable, &Position)>,
    q_player: Query<(Entity, &MapView), With<Player>>,
    q_memory: Query<&MapMemory>,
    map: Single<&Map>,
    mut term: Single<&mut Terminal, With<GameTerminal>>,
) {
    term.clear();
    term.set_pivot(Pivot::LeftBottom);
    let map = map.into_inner();

    if let Ok((entity, player_view)) = q_player.single() {
        if let Ok(memory) = q_memory.get(entity) {
            render_memory(memory, map, &mut term);
        }
        render_view(player_view, &mut term, map, q_entities.iter());
    } else {
        render_everything(map, &mut term, q_entities.iter());
    }
}

// TODO: Should be handled by some kind of prefab/asset setup
impl From<MapTile> for Tile {
    fn from(t: MapTile) -> Self {
        match t {
            MapTile::Wall => Tile {
                glyph: '#',
                fg_color: WALL_COLOR,
                bg_color: LinearRgba::BLACK,
            },
            MapTile::Floor => Tile {
                glyph: '.',
                fg_color: FLOOR_COLOR,
                bg_color: LinearRgba::BLACK,
            },
        }
    }
}

impl From<&Renderable> for Tile {
    fn from(r: &Renderable) -> Self {
        Tile {
            glyph: r.glyph,
            fg_color: r.fg_color,
            bg_color: r.bg_color,
        }
    }
}

fn render_view<'a, Actors>(view: &MapView, term: &mut Terminal, map: &Map, actors: Actors)
where
    Actors: Iterator<Item = (&'a Renderable, &'a Position)>,
{
    render_map_in_view(view, map, term);
    render_actors_in_view(view, term, actors);
}

fn render_map_in_view(view: &MapView, map: &Map, term: &mut Terminal) {
    for (i, seen) in view.0.iter().enumerate() {
        if *seen {
            // NOTE: Assumes map size = view size = GAME_SIZE constant
            let tile = map.0[i];
            
            let p = index_to_xy(i);
            term.put_tile(p, Tile::from(tile));
        }
    }
}

fn render_actors_in_view<'a, Actors>(view: &MapView, term: &mut Terminal, actors: Actors)
where
    Actors: Iterator<Item = (&'a Renderable, &'a Position)>,
{
    for (renderable, pos) in actors {
        if view.0.is_empty() {
            return;
        }
        // NOTE: Assumes map size = GAME_SIZE constant
        let i = xy_to_index(pos.0);

        if view.0[i] {
            term.put_tile(pos.0, Tile::from(renderable));
        }
    }
}

fn render_memory(memory: &MapMemory, map: &Map, term: &mut Terminal) {
    for (i, remembered) in memory.0.iter().enumerate() {
        if *remembered {
            let tile = map.0[i];
            let mut tile: Tile = tile.into();
            tile.fg_color = greyscale(tile.fg_color);

            let p = index_to_xy(i);
            term.put_tile(p, tile);
        }
    }
}

fn greyscale(c: LinearRgba) -> LinearRgba {
    let [r, g, b, _] = c.to_f32_array();
    let grey = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let grey = grey / 8.0;
    LinearRgba::rgb(grey, grey, grey)
}

fn render_everything<'a, Actors>(map: &Map, term: &mut Terminal, actors: Actors)
where
    Actors: Iterator<Item = (&'a Renderable, &'a Position)>,
{
    render_full_map(map, term);
    render_all_entities(term, actors);
}
fn render_full_map(map: &Map, term: &mut Terminal) {
    for i in 0..GAME_SIZE.element_product()  as usize{
        let t = match map.0[i] {
            MapTile::Wall => Tile {
                glyph: '#',
                fg_color: WALL_COLOR,
                bg_color: LinearRgba::BLACK,
            },
            MapTile::Floor => Tile {
                glyph: '.',
                fg_color: FLOOR_COLOR,
                bg_color: LinearRgba::BLACK,
            },
        };
        let p = index_to_xy(i);
        term.put_tile(p, t);
    }
}

fn render_all_entities<'a, Entities>(term: &mut Terminal, entities: Entities)
where
    Entities: Iterator<Item = (&'a Renderable, &'a Position)>,
{
    for (r, pos) in entities {
        term.put_tile(pos.0, Tile::from(r));
    }
}

fn should_render(
    q_entities_changed: Query<(&Renderable, &Position), Changed<Position>>,
    q_map_changed: Query<&Map, Changed<Map>>,
    // mut evt_killed: MessageReader<ActorKilled>,
    reset: MessageReader<Reset>,
) -> bool {
    let entities_changed = !q_entities_changed.is_empty();
    let map_changed = !q_map_changed.is_empty();
    let killed = false;//evt_killed.iter().next().is_some();
    let reset = !reset.is_empty();

    map_changed || entities_changed || killed || reset
}
