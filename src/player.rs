use bevy::prelude::*;

use crate::{components::*, xy_to_index};

use crate::map_state::{MapActors, MapObstacles};
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
    mut q_player: Query<(Entity, &Strength, &mut Position, &mut Energy, &AttackDice, &mut Movement), (With<Player>, With<TakingATurn>)>,
    q_monsters: Query<&Name, With<Monster>>,
    input: Res<ButtonInput<KeyCode>>,
    mut obstacles: ResMut<MapObstacles>,
    mut actors: ResMut<MapActors>,
    // _event_attack: MessageWriter<AttackEvent>,
    // mut evt_attack: MessageWriter<TargetEvent>,
    // mut rng: Local<DiceRng>,
) {
    if let Ok((entity, _attack, mut pos, mut energy, dice, mut movement)) = q_player.single_mut() {
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

        // let attack = rng.roll(dice.0);

        if obstacles.0[nexti] {
            if let Some(target) = actors.0[nexti] {
                if let Ok(_name) = q_monsters.get(target) {
                    // evt_attack.send( TargetEvent {
                    //     actor: entity,
                    //     target,
                    //     effect: ActorEffect::Damage(attack),
                    // });

                    energy.0 = 0;
                }
            }
            return;
        }

        // //println!("Player moved, ending their turn");
        pos.0 = pos.0 + move_input;
        energy.0 = 0;
        actors.0[curri] = None;
        actors.0[nexti] = Some(entity);
        obstacles.0[curri] = false;
        obstacles.0[nexti] = true;
        movement.0 = move_input;
    }
}

fn read_movement(input: &ButtonInput<KeyCode>) -> IVec2 {
    let right = input.any_just_pressed([KeyCode::ArrowRight, KeyCode::KeyD, KeyCode::Numpad6]) as i32;
    let left = input.any_just_pressed([KeyCode::ArrowLeft, KeyCode::KeyA, KeyCode::Numpad4]) as i32;
    let up = input.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW, KeyCode::Numpad8]) as i32;
    let down = input.any_just_pressed([KeyCode::ArrowDown, KeyCode::KeyS, KeyCode::Numpad2]) as i32;

    IVec2::new(right - left, up - down)
}

fn read_wait(input: &ButtonInput<KeyCode>) -> bool { 
    input.any_just_pressed([KeyCode::Numpad5, KeyCode::ControlLeft, KeyCode::Space])
}