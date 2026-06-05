use bevy::prelude::*;

use crate::combat::{AttackEvent, Strength};
use crate::monster::Monster;
use crate::{components::*, xy_to_index};

use crate::map_state::{MapActors, PathingData};
use crate::turn_system::{Energy, TakingATurn};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, player_input);
    }
}

#[derive(Default, Debug, Component, Clone)]
pub struct Player;

fn player_input(
    mut q_player: Query<
        (Entity, &Strength, &mut Position, &mut Energy),
        (With<Player>, With<TakingATurn>),
    >,
    q_monsters: Query<&Name, With<Monster>>,
    input: Res<ButtonInput<KeyCode>>,
    mut pathing: ResMut<PathingData>,
    mut actors: ResMut<MapActors>,
    mut commands: Commands,
) {
    if let Ok((entity, _attack, mut pos, mut energy)) = q_player.single_mut() {
        if read_wait(&input) {
            energy.0 = 0;
            return;
        }

        let move_input = read_movement(&input);
        if move_input.cmpeq(IVec2::ZERO).all() {
            return;
        }

        let curri = xy_to_index(pos.0);
        let nexti = xy_to_index(pos.0 + move_input);

        if pathing.0.obstacles.get_index(nexti) {
            if let Some(target) = actors.0[nexti] {
                if let Ok(_name) = q_monsters.get(target) {
                    commands.trigger(AttackEvent {
                        actor: entity,
                        target,
                    });
                    energy.0 = 0;
                }
            }
            return;
        }

        pos.0 = pos.0 + move_input;
        energy.0 = 0;
        actors.0[curri] = None;
        actors.0[nexti] = Some(entity);
        pathing.0.obstacles.set_index(curri, false);
        pathing.0.obstacles.set_index(nexti, true);
    }
}

fn read_movement(input: &ButtonInput<KeyCode>) -> IVec2 {
    use KeyCode::*;
    let right =
        input.any_just_pressed([ArrowRight, KeyE, KeyD, KeyC, Numpad9, Numpad6, Numpad3]) as i32;
    let left =
        input.any_just_pressed([ArrowLeft, KeyA, KeyQ, KeyZ, Numpad7, Numpad4, Numpad1]) as i32;
    let up = input.any_just_pressed([ArrowUp, KeyQ, KeyW, KeyE, Numpad7, Numpad8, Numpad9]) as i32;
    let down = input
        .any_just_pressed([ArrowDown, KeyZ, KeyX, KeyS, KeyC, Numpad1, Numpad2, Numpad3])
        as i32;

    IVec2::new(right - left, up - down)
}

fn read_wait(input: &ButtonInput<KeyCode>) -> bool {
    input.any_just_pressed([KeyCode::Numpad5, KeyCode::ControlLeft, KeyCode::Space])
}
