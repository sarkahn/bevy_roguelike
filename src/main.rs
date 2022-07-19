use bevy::{prelude::*};

use bevy_ascii_terminal::{TerminalBundle, TiledCameraBundle};

mod bundle;
mod config;
mod map;
mod map_state;
mod monster;
mod movement;
mod player;
mod render;
mod shapes;
mod visibility;
mod ui;
mod events;
//mod web_resize;
mod turn_system;
mod combat;
mod rng;

#[derive(Component)]
pub struct GameTerminal;


pub const VIEWPORT_SIZE: [u32;2] = [80,40];

pub const UI_SIZE: [u32;2] = [VIEWPORT_SIZE[0],8];
// TODO: Map size should be separate
pub const GAME_SIZE: [u32;2] = [VIEWPORT_SIZE[0], VIEWPORT_SIZE[1] - UI_SIZE[1]];


fn setup(mut commands: Commands) {
    //commands.spawn().insert(gen.map);

    let term_y = VIEWPORT_SIZE[1] as f32 / 2.0 - GAME_SIZE[1] as f32 / 2.0; 
    let term_bundle = TerminalBundle {
        transform: Transform::from_xyz(0.0, term_y, 0.0),
        ..TerminalBundle::new().with_size([GAME_SIZE[0], GAME_SIZE[1] + 2])
    };
    //term_bundle.transform = Transform::from_xyz(0.0, 0.0, UI_SIZE[1] as f32 * 2.0);
    commands.spawn_bundle(term_bundle).insert(GameTerminal);

    let totalx = GAME_SIZE[0];
    let totaly = GAME_SIZE[1] + UI_SIZE[1];
    commands.spawn_bundle(TiledCameraBundle::new().with_tile_count([totalx, totaly]));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(map::MapGenPlugin)
        .add_plugin(render::RenderPlugin)
        .add_plugin(events::EventsPlugin)
        .add_plugin(visibility::VisiblityPlugin)
        .add_plugin(map_state::MapStatePlugin)
        //.add_plugin(web_resize::FullViewportPlugin)
        .add_plugin(turn_system::TurnSystemPlugin)
        .add_plugin(monster::MonstersPlugin)
        .add_plugin(combat::CombatPlugin)
        .add_plugin(ui::UiPlugin)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}
