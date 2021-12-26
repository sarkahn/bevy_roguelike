use bevy::prelude::*;
use bevy_ascii_terminal::*;

use crate::{bundle::MovingEntityBundle, visibility::{MapView, ViewRange, MapMemory}, movement::{Movement}};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(player_input.system());
    }
}

#[derive(Default, Debug)]
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
    mut q_player: Query<&mut Movement, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    if let Ok(mut movement) = q_player.single_mut() {
        let input = read_movement(&input);

        if input.cmpeq(IVec2::ZERO).all() {
            return;
        }

        movement.0 = input.into();
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