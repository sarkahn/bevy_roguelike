use bevy::{ecs::schedule::ShouldRun, prelude::*};
use bevy_ascii_terminal::*;

use crate::{
    map::{Map, MapTile},
    movement::Position,
    player::Player,
    visibility::{MapMemory, MapView}, GameTerminal, combat::ActorKilledEvent,
};

pub const WALL_COLOR: Color = Color::Rgba{ red:0.866, green:0.866, blue:0.882, alpha: 1.0};
pub const FLOOR_COLOR: Color = Color::Rgba{ red:0.602, green:0.462, blue:0.325, alpha: 1.0};

pub const RENDER_SYSTEM_LABEL: &str = "GAME_RENDER_SYSTEM";

/// Plugin managing game rendering systems
pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(CoreStage::Last, 
            SystemSet::new()
            .with_run_criteria(should_render.system())
            .with_system(
                render
                .label(RENDER_SYSTEM_LABEL)
            ),
        )
        .add_plugin(TerminalPlugin);
    }
}

#[derive(Component, Debug)]
pub struct Renderable {
    pub fg_color: Color,
    pub bg_color: Color,
    pub glyph: char,
}

fn render(
    q_map: Query<&Map>,
    q_entities: Query<(&Renderable, &Position)>,
    q_player: Query<(Entity, &MapView), With<Player>>,
    q_memory: Query<&MapMemory>,
    mut q_render_terminal: Query<&mut Terminal, With<GameTerminal>>,
) {
    let mut term = match q_render_terminal.get_single_mut() {
        Ok(term) => term,
        Err(_) => return,
    };

    let map = match q_map.get_single() {
        Ok(term) => term,
        Err(_) => return,
    };

    if term.size() != map.0.size() {
        term.resize(map.0.size().into());
    }

    term.clear();

    if let Ok((entity, player_view)) = q_player.get_single() {
        if let Ok(memory) = q_memory.get(entity) {
            render_memory(memory, map, &mut term);
        }
        render_view(player_view, &mut term, map, q_entities.iter());
    } else {
        render_everything(map, &mut term, q_entities.iter());
    }

    term.draw_border_single();
}

// TODO: Should be handled by some kind of prefab/asset setup
impl From<MapTile> for Tile {
    fn from(t: MapTile) -> Self {
        match t {
            MapTile::Wall => Tile {
                glyph: '#',
                fg_color: WALL_COLOR,
                bg_color: Color::BLACK,
            },
            MapTile::Floor => Tile {
                glyph: '.',
                fg_color: FLOOR_COLOR,
                bg_color: Color::BLACK,
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
    render_actors_in_view(view, map, term, actors);
}

fn render_map_in_view(view: &MapView, map: &Map, term: &mut Terminal) {
    for (i, seen) in view.0.iter().enumerate() {
        if *seen {
            let p = map.0.index_to_pos(i);
            let tile = map.0[p];
            
            // Convert to terminal position
            term.put_tile(p.into(), tile.into());
        }
    }
}

fn render_actors_in_view<'a, Actors>(view: &MapView, map: &Map, term: &mut Terminal, actors: Actors)
where
    Actors: Iterator<Item = (&'a Renderable, &'a Position)>,
{
    for (renderable, pos) in actors {
        let i = map.0.pos_to_index( pos.0.into() );

        if view.0[i] {
            term.put_tile(pos.0.into(), Tile::from(renderable));
        }
    }
}

fn render_memory(memory: &MapMemory, map: &Map, term: &mut Terminal) {
    for (i, remembered) in memory.0.iter().enumerate() {
        if *remembered {
            let p = IVec2::from(term.to_xy(i));
            let tile = map.0[p];

            let mut tile: Tile = tile.into();
            tile.fg_color = greyscale(tile.fg_color);

            term.put_tile(p.into(), tile);
        }
    }
}

fn greyscale(c: Color) -> Color {
    let [r, g, b, _]: [f32; 4] = c.into();
    let grey = 0.2126 * r + 0.7152 * g + 0.0722 * b;
    let grey = grey / 8.0;
    Color::rgb(grey, grey, grey)
}

fn render_everything<'a, Actors>(map: &Map, term: &mut Terminal, actors: Actors)
where
    Actors: Iterator<Item = (&'a Renderable, &'a Position)>,
{
    render_full_map(map, term);
    render_all_entities(term, actors);
}
fn render_full_map(map: &Map, term: &mut Terminal) {
    for x in 0..map.0.width() as i32 {
        for y in 0..map.0.height() as i32 {
            let tile: Tile = match map.0[ [x as u32, y as u32] ] {
                MapTile::Wall => Tile {
                    glyph: '#',
                    fg_color: WALL_COLOR,
                    bg_color: Color::BLACK,
                },
                MapTile::Floor => Tile {
                    glyph: '.',
                    fg_color: FLOOR_COLOR,
                    bg_color: Color::BLACK,
                },
            };
            term.put_tile([x as i32, y as i32], tile);
        }
    }
}

fn render_all_entities<'a, Entities>(term: &mut Terminal, entities: Entities)
where
    Entities: Iterator<Item = (&'a Renderable, &'a Position)>,
{
    for (r, pos) in entities {
        let fmt = CharFormat::new(r.fg_color, r.bg_color);
        term.put_char_formatted(pos.0.into(), r.glyph, fmt);
    }
}

fn should_render(
    q_entities_changed: Query<(&Renderable, &Position), Changed<Position>>,
    q_map_changed: Query<&Map, Changed<Map>>,
    mut evt_killed: EventReader<ActorKilledEvent>,
) -> ShouldRun {
    let entities_changed = q_entities_changed.iter().next().is_some();
    let map_changed = q_map_changed.iter().next().is_some();
    let killed = evt_killed.iter().next().is_some();

    if map_changed || entities_changed || killed {
        return ShouldRun::Yes;
    }

    ShouldRun::No
}
