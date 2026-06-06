use bevy::prelude::*;

use crate::{
    GameState,
    combat::{AttackEvent, Strength},
    components::*,
    inventory::PickupItem,
    items::Item,
    map_state::{MapActors, PathingData},
    monster::Monster,
    turn_system::{Energy, TakingATurn},
    xy_to_index,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            player_input.run_if(in_state(GameState::Exploring)),
        );
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
    items: Query<(Entity, &Position), (With<Item>, Without<Player>)>,
    mut pathing: ResMut<PathingData>,
    mut actors: ResMut<MapActors>,
    mut commands: Commands,
) {
    if let Ok((entity, _attack, mut pos, mut energy)) = q_player.single_mut() {
        if read_wait(&input) {
            energy.0 = 0;
            return;
        }

        if input.just_pressed(KeyCode::KeyI) {
            commands.set_state(GameState::Inventory);
            return;
        }

        if input.just_pressed(KeyCode::Comma) {
            energy.0 = 0;
            for (e, p) in &items {
                if p.0 == pos.0 {
                    commands.trigger(PickupItem {
                        item: e,
                        picker_upper: entity,
                    });
                    return;
                }
            }

            commands.write_message(LogMessage("There was nothing to pick up.".to_string()));
            return;
        }

        let move_input = read_movement(&input);
        if move_input.cmpeq(IVec2::ZERO).all() {
            return;
        }

        let curri = xy_to_index(pos.0);
        let next = pos.0 + move_input;
        let nexti = xy_to_index(next);

        if pathing.0.obstacles.get_index(nexti) {
            if let Some(target) = actors.0.get(&next) {
                if let Ok(_name) = q_monsters.get(*target) {
                    commands.trigger(AttackEvent {
                        actor: entity,
                        target: *target,
                    });
                    energy.0 = 0;
                }
            }
            return;
        }

        energy.0 = 0;

        actors.0.remove(&pos.0);
        pos.0 = pos.0 + move_input;
        actors.0.insert(pos.0, entity);

        pathing.0.obstacles.set_index(curri, false);
        pathing.0.obstacles.set_index(nexti, true);
    }
}

fn read_movement(input: &ButtonInput<KeyCode>) -> IVec2 {
    let right = input.any_just_pressed(crate::input::RIGHT.iter().cloned()) as i32;
    let left = input.any_just_pressed(crate::input::LEFT.iter().cloned()) as i32;
    let up = input.any_just_pressed(crate::input::UP.iter().cloned()) as i32;
    let down = input.any_just_pressed(crate::input::DOWN.iter().cloned()) as i32;

    IVec2::new(right - left, up - down)
}

fn read_wait(input: &ButtonInput<KeyCode>) -> bool {
    input.any_just_pressed([KeyCode::Numpad5, KeyCode::ControlLeft, KeyCode::Space])
}
