use bevy::prelude::*;

use bevy_ascii_terminal::*;
use rand::RngExt;

use crate::map::MapGenSettings;

mod entities;
mod components;
mod map;
// mod bundle;
// mod config;
// mod map;
// mod map_state;
// mod monster;
// mod movement;
// mod player;
// mod render;
// mod shapes;
// mod visibility;
// mod ui;
// mod events;
// //mod web_resize;
// mod turn_system;
// mod combat;
// mod rng;

#[derive(Component)]
pub struct GameTerminal;


#[derive(Component)]
pub struct UiTerminal;

pub const VIEWPORT_SIZE: UVec2 = UVec2::new(80,GAME_SIZE.y + UI_SIZE.y);

pub const UI_SIZE: UVec2 = UVec2::new(80, 8);
// TODO: Map size should be separate
pub const GAME_SIZE: UVec2 = UVec2::new(80, 32);

pub fn xy_to_index(xy: impl Into<IVec2>) -> usize {
    let [x,y] = xy.into().to_array();
    y as usize * GAME_SIZE[0] as usize + x as usize
}

/// Random point within the game area
pub fn random_point() -> IVec2 {
    let x = rand::random_range(0..GAME_SIZE.x - 1) as i32;
    let y = rand::random_range(0..GAME_SIZE.y - 1) as i32;
    IVec2::new(x,y)
}

fn setup(mut commands: Commands) {
    commands.spawn((
      Terminal::new(GAME_SIZE).with_string([0,0], "Hello"),
      GameTerminal,
      TerminalMeshPivot::LeftBottom
    ));
    commands.spawn((
        Terminal::new(UI_SIZE).with_border(BoxStyle::SINGLE_LINE).with_string([0,0], "UI CITY"),
        UiTerminal,
        TerminalMeshPivot::LeftTop
    ));

    commands.spawn(TerminalCamera::new());

    let settings = MapGenSettings::default();
    let map = map::build(&settings).unwrap();


}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugins))
        .add_systems(Startup, setup)
        // .add_plugin(player::PlayerPlugin)
        // .add_plugin(map::MapGenPlugin)
        // .add_plugin(render::RenderPlugin)
        // .add_plugin(events::EventsPlugin)
        // .add_plugin(visibility::VisiblityPlugin)
        // .add_plugin(map_state::MapStatePlugin)
        // //.add_plugin(web_resize::FullViewportPlugin)
        // .add_plugin(turn_system::TurnSystemPlugin)
        // .add_plugin(monster::MonstersPlugin)
        // .add_plugin(combat::CombatPlugin)
        // .add_plugin(ui::UiPlugin)
        // .add_startup_system(setup)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
