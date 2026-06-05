use bevy::prelude::*;
use sark_pathfinding::Pathfinder;

use crate::{
    combat::{AttackDice, AttackEvent},
    components::Position,
    map_state::{MapActors, PathingData},
    player::Player,
    turn_system::{Energy, TakingATurn},
    ui::LogMessage,
    visibility::MapView,
    xy_to_index,
};

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, monster_ai);
    }
}

#[derive(Default, Debug, Component, Clone)]
pub struct Monster;

fn monster_ai(
    mut commands: Commands,
    mut pathing: ResMut<PathingData>,
    mut entities: ResMut<MapActors>,
    q_player: Query<(Entity, &Position), With<Player>>,
    mut q_monster: Query<
        (
            Entity,
            &mut Position,
            &mut Energy,
            &AttackDice,
            &MapView,
            &Name,
        ),
        (With<Monster>, Without<Player>, With<TakingATurn>),
    >,
    // mut attack_events: EventW<TargetMessage>,
    // mut rng: Local<DiceRng>,
    mut finder: Local<Pathfinder>,
) {
    for (entity, mut pos, mut energy, dice, view, name) in q_monster.iter_mut() {
        let mut posi = xy_to_index(pos.0);

        if let Ok((player, player_pos)) = q_player.single() {
            let player_pos = player_pos.0;
            let player_posi = xy_to_index(player_pos);

            // If Monster can see the player
            if view.0[player_posi] {
                // Open the player and monster positions so pathfinding doesn't see them as obstacles
                pathing.0.obstacles.set_index(posi, false);
                pathing.0.obstacles.set_index(player_posi, false);

                // let mut astar = AStar::new(5);

                // if let Some(path) = astar.find_path(&pathing.0, *pos, player_pos) {
                if let Some(path) = finder.astar(&pathing.0, pos.0, player_pos) {
                    if path.len() == 2 {
                        // let damage = rng.roll(dice.0);
                        // attack_events.send(TargetEvent {
                        //     actor: entity,
                        //     target: player,
                        //     effect: ActorEffect::Damage(damage),
                        // });

                        commands.trigger(AttackEvent {
                            actor: entity,
                            target: player,
                        });
                    } else {
                        entities.0[posi] = None;
                        (*pos).0 = path[1];
                        posi = xy_to_index(pos.0);
                        entities.0[posi] = Some(entity);
                    }
                }

                pathing.0.set_obstacle(pos.0, true);
                pathing.0.set_obstacle(player_pos, true);
            }
        }

        energy.0 = 0;
    }
}
