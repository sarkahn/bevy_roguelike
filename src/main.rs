use bevy::prelude::*;

use bevy_ascii_terminal::TerminalBundle;
use bevy_tiled_camera::{TiledCameraBundle, TiledCameraPlugin};

mod bundle;
mod config;
mod map;
mod monster;
mod movement;
mod player;
mod render;
mod shapes;
mod visibility;
mod map_state;

use map::*;
use player::PlayerBundle;
use rand::{prelude::StdRng, SeedableRng};

fn setup(mut commands: Commands) {
    let settings = match config::try_get_map_settings() {
        Ok(settings) => settings,
        Err(e) => panic!("{}", e),
    };

    let size = settings.map_size;

    let rng = StdRng::seed_from_u64(settings.seed);

    let entities = MapGenEntities {
        player: PlayerBundle::default(),
    };

    MapGenerator::build(&mut commands, settings, rng, entities);

    //commands.spawn().insert(gen.map);

    commands.spawn_bundle(TerminalBundle::new().with_size(size));

    commands.spawn_bundle(TiledCameraBundle::new().with_tile_count(size));
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TiledCameraPlugin)
        .add_plugin(render::RenderPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_plugin(visibility::VisiblityPlugin)
        .add_plugin(player::PlayerPlugin)
        .add_plugin(map_state::MapStatePlugin)
        .add_startup_system(setup.system())
        .run();
}
