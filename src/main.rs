use bevy::prelude::*;

use bevy_ascii_terminal::*;
use map::Map;

use crate::{map::MapGenSettings, turn_system::Actor};

mod combat;
mod components;
mod entities;
mod map;
mod map_state;
mod monster;
mod player;
mod render;
mod turn_system;
mod ui;
mod visibility;

#[derive(Component)]
pub struct GameTerminal;

#[derive(Message)]
pub struct Reset;

pub const VIEWPORT_SIZE: UVec2 = UVec2::new(80, GAME_SIZE.y + UI_SIZE.y);

pub const UI_SIZE: UVec2 = UVec2::new(80, 8);
// TODO: Map size should be separate?
pub const GAME_SIZE: UVec2 = UVec2::new(80, 32);

pub fn xy_to_index(xy: impl Into<IVec2>) -> usize {
    let [x, y] = xy.into().to_array();
    y as usize * GAME_SIZE.x as usize + x as usize
}

pub fn index_to_xy(index: usize) -> IVec2 {
    let x = index % GAME_SIZE.x as usize;
    let y = index / GAME_SIZE.x as usize;
    IVec2::new(x as i32, y as i32)
}

/// Random point within the game area
pub fn random_point() -> IVec2 {
    let x = rand::random_range(0..GAME_SIZE.x) as i32;
    let y = rand::random_range(0..GAME_SIZE.y) as i32;
    IVec2::new(x, y)
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Terminal::new(GAME_SIZE).with_string([0, 0], "Hello"),
        GameTerminal,
        TerminalMeshPivot::LeftBottom,
    ));

    commands.spawn(TerminalCamera::new());

    commands.write_message(Reset);
}

fn debuggo(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::Escape) {
        commands.write_message(AppExit::Success);
    }

    if input.just_pressed(KeyCode::Space) {
        commands.write_message(Reset);
    }
}

fn reset(
    mut commands: Commands,
    map: Option<Single<Entity, With<Map>>>,
    actors: Query<Entity, With<Actor>>,
) {
    for e in &actors {
        commands.entity(e).despawn();
    }

    if let Some(map) = map {
        commands.entity(*map).despawn();
    }
    let settings = MapGenSettings::default();
    let map = map::build(&settings).expect("Invalid map data");
    for (p, c) in map.entities {
        if c == '@' {
            commands.spawn_scene(entities::player(p));
        }
        if c == 'g' {
            commands.spawn_scene(entities::goblin(p));
        }
        if c == 'o' {
            commands.spawn_scene(entities::orc(p));
        }
    }

    commands.spawn(map.map);
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(ClearColor(Color::BLACK))
        .add_message::<Reset>()
        .add_plugins(ui::UiPlugin)
        .add_plugins(visibility::VisiblityPlugin)
        .add_plugins(combat::CombatPlugin)
        .add_plugins(turn_system::TurnSystemPlugin)
        .add_plugins(map_state::MapStatePlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(render::RenderPlugin)
        .add_plugins(monster::MonstersPlugin)
        .add_systems(First, reset.run_if(on_message::<Reset>))
        .add_systems(Update, debuggo)
        .add_systems(Startup, setup)
        .run();
}
