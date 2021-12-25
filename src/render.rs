use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_ascii_terminal::*;
use serde::Deserialize;

use crate::{map::{Map, MapTile}, movement::Position};


/// Plugin managing game rendering systems
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(should_render.system())
                .with_system(render.system())
        )
        .add_plugin(TerminalPlugin);
    }
}


#[derive(Debug)]
pub struct Renderable {
    pub fg_color: TileColor,
    pub bg_color: TileColor,
    pub glyph: char,
}

fn render(
    q_map: Query<&Map>,
    q_entities: Query<(&Renderable, &Position)>,
    mut q_render_terminal: Query<&mut Terminal>,
) {

    let mut term = match q_render_terminal.single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };

    let map = match q_map.single() {
        Ok(term) => term,
        Err(_) => return,
    };

    if term.size() != map.size() {
        term.resize(map.size());
    }

    term.clear();

    //let entities = q_entities.iter();

    //render_all_entities(&mut term, entities);
    render_full_map(map, &mut term);
}

fn render_full_map(
    map: &Map,
    term: &mut Terminal,
) {
    for x in 0..map.width() {
        for y in 0..map.height() {
            let tile: Tile = match map[(x,y)] {
                MapTile::Wall => Tile {
                    glyph: '#',
                    fg_color: GREEN,
                    bg_color: BLACK,
                },
                MapTile::Floor => Tile {
                    glyph: '.',
                    fg_color: WHITE,
                    bg_color: BLACK,
                },
            };
            term.put_tile((x as i32,y as i32), tile); 
        }
    }
}

fn render_all_entities<'a, Entities>(
    term: &mut Terminal, 
    entities: Entities,
) where
    Entities: Iterator<Item = (&'a Renderable, &'a Position)>,
{
    for (r, pos) in entities {
        term.put_char(pos.0, r.glyph);
    }
}

fn should_render(
    q_entities_changed: Query<(&Renderable, &Position), Changed<Position>>,
    q_map_changed: Query<&Map, Changed<Map>>,
) -> ShouldRun {
    let entities_changed = q_entities_changed.iter().next().is_some();
    let map_changed = q_map_changed.iter().next().is_some();

    if map_changed || entities_changed {
        return ShouldRun::Yes;
    }

    ShouldRun::No
}