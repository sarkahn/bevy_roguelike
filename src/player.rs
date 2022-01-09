use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::{
    bundle::MovingEntityBundle,
    map::Map,
    map_state::{MapActors, MapObstacles},
    monster::Monster,
    movement::{Movement, Position},
    visibility::{MapMemory, MapView, ViewRange},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_input.system());
    }
}

#[derive(Component, Default, Debug)]
pub struct Player;

#[derive(Debug, Bundle)]
pub struct PlayerBundle {
    #[bundle]
    pub move_bundle: MovingEntityBundle,
    pub player: Player,
    pub view: MapView,
    pub memory: MapMemory,
    pub view_range: ViewRange,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            move_bundle: MovingEntityBundle::new(WHITE, '@'),
            player: Default::default(),
            view: Default::default(),
            memory: Default::default(),
            view_range: ViewRange(5),
        }
    }
}

fn player_input(
    mut q_player: Query<(&Position, &mut Movement), With<Player>>,
    q_map: Query<&Map>,
    q_monsters: Query<&Name, With<Monster>>,
    input: Res<Input<KeyCode>>,
    obstacles: Res<MapObstacles>,
    actors: Res<MapActors>,
) {
    if let Ok((pos, mut movement)) = q_player.get_single_mut() {
        if let Ok(map) = q_map.get_single() {
            let input = read_movement(&input);

            if input.cmpeq(IVec2::ZERO).all() {
                return;
            }

            let curr = IVec2::from(pos.0);

            let next = curr + input;

            let next_i = map.0.pos_to_index(next.into());

            if obstacles.0[next_i] {
                if let Some(entity) = actors.0[next_i] {
                    if let Ok(name) = q_monsters.get(entity) {
                        println!("You bumped into {}", name.as_str());
                        return;
                    }
                }
            }

            movement.0 = input.into();
        }
    }
}

fn read_movement(input: &Input<KeyCode>) -> IVec2 {
    let mut p = IVec2::ZERO;

    if input.just_pressed(KeyCode::Numpad1) {
        p.x = -1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad2) {
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad3) {
        p.x = 1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad4) {
        p.x = -1;
    }
    if input.just_pressed(KeyCode::Numpad6) {
        p.x = 1;
    }
    if input.just_pressed(KeyCode::Numpad7) {
        p.x = -1;
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad8) {
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad9) {
        p.x = 1;
        p.y = 1;
    }
    p
}
