use bevy::prelude::*;

use bevy_ascii_terminal::{TerminalBundle, TerminalPlugin};
use bevy_tiled_camera::{TiledCameraBundle, TiledCameraPlugin};

mod config;
mod entity;
mod map;
mod movement;
mod render;
mod shapes;

use map::*;
use rand::{prelude::StdRng, SeedableRng};

fn setup(mut commands: Commands) {
    let settings = match config::try_get_map_settings() {
        Ok(settings) => settings,
        Err(e) => panic!("{}", e),
    };

    let size = settings.map_size;

    let rng = StdRng::seed_from_u64(settings.seed);

    let gen = MapGenerator::build(settings, rng);

    commands.spawn().insert(gen.map);

    commands.spawn_bundle(TerminalBundle::new().with_size(size));

    commands.spawn_bundle(TiledCameraBundle::new().with_tile_count(size));
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TiledCameraPlugin)
        .add_plugin(render::RenderPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_startup_system(setup.system())
        .run();
}
