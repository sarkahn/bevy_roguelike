use bevy::{
    DefaultPlugins,
    app::{App, AppExit, First, Startup, Update},
    camera::ClearColor,
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        message::Message,
        query::With,
        schedule::{IntoScheduleConfigs, common_conditions::on_message},
        system::{Commands, Query, Res, Single},
    },
    input::{ButtonInput, keyboard::KeyCode},
    math::{IVec2, UVec2},
    scene::CommandsSceneExt,
    state::{app::AppExtStates, state::States},
};
use bevy_ascii_terminal::*;
use map::Map;

use crate::{
    combat::ActorKilled,
    components::{LogMessage, Redraw},
    inventory::PickupItem,
    map::MapGenSettings,
    map_state::{MapActors, PathingData},
    player::Player,
    turn_system::Actor,
};

mod combat;
mod components;
mod entities;
mod input;
mod inventory;
mod items;
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

fn debuggo(
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    q_player: Option<Single<Entity, With<Player>>>,
) {
    if input.pressed(KeyCode::ControlLeft) && input.just_pressed(KeyCode::KeyZ) {
        commands.write_message(AppExit::Success);
    }

    if input.just_pressed(KeyCode::KeyR) {
        commands.write_message(Reset);
    }

    if input.just_pressed(KeyCode::KeyG) {
        if let Some(player) = q_player.map(|v| v.into_inner()) {
            let item = if rand::random_bool(0.5) {
                commands.spawn_scene(items::minor_healing_potion()).id()
            } else {
                commands.spawn_scene(items::scroll_of_magic_missile()).id()
            };
            commands.trigger(PickupItem {
                item,
                picker_upper: player,
            });
        }
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
        if c == '¡' {
            commands.spawn_scene(items::minor_healing_potion_pos(p));
        }
        if c == ')' {
            commands.spawn_scene(items::scroll_of_magic_missile_pos(p));
        }
    }

    commands.spawn(map.map);
}

#[derive(States, Debug, Hash, Default, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    #[default]
    Exploring,
    Inventory,
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .insert_resource(ClearColor(Color::BLACK))
        .add_message::<Reset>()
        .add_message::<LogMessage>()
        .add_message::<Redraw>()
        .add_message::<ActorKilled>()
        .init_state::<GameState>()
        .init_resource::<PathingData>()
        .init_resource::<MapActors>()
        .add_plugins(ui::UiPlugin)
        .add_plugins(inventory::InventoryPlugin)
        .add_plugins(items::ItemsPlugin)
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
