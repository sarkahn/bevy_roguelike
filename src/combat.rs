use bevy::prelude::*;

use crate::{ui::PrintLog, map_state::{MapObstacles, MapActors}, movement::Position};

pub const RESOLVE_TARGET_EVENTS_SYSTEM_LABEL: &str = "resolve_target_events";
pub const DEATH_SYSTEM_LABEL: &str = "death_system";

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<TargetEvent>()
        .add_event::<ActorKilledEvent>()
        .add_system_to_stage(CoreStage::PostUpdate, resolve_target_events
            .label(RESOLVE_TARGET_EVENTS_SYSTEM_LABEL))
        .add_system_to_stage(CoreStage::PostUpdate, death_system
            .after(RESOLVE_TARGET_EVENTS_SYSTEM_LABEL)
            .label(DEATH_SYSTEM_LABEL));
    }
}

#[derive(Debug, Component)]
pub struct MaxHitPoints(pub i32);

#[derive(Debug, Component)]
pub struct HitPoints(pub i32);

#[derive(Default, Debug,Component)]
pub struct Defense(pub i32);

#[derive(Default, Debug, Component)]
pub struct AttackPower(pub i32);

#[derive(Debug, Bundle)]
pub struct CombatantBundle {
    pub hp: HitPoints,
    pub max_hp: MaxHitPoints,
    pub defense: Defense,
    pub attack: AttackPower,
}

pub enum ActorEffect {
    Heal(i32),
    Damage(i32),
}

pub struct TargetEvent {
    pub actor: Entity,
    pub target: Entity,
    pub effect: ActorEffect,
}

pub struct ActorKilledEvent {
    name: String,
}

fn resolve_target_events(
    q_names: Query<&Name>,
    q_attack: Query<&mut AttackPower>,
    mut q_defend: Query<(&mut HitPoints, &MaxHitPoints, &Defense)>,
    mut log: ResMut<PrintLog>,
    mut target_events: EventReader<TargetEvent>,
) {
    for ev in target_events.iter() {
        let tar = ev.target;
        let actor = ev.actor;
        match ev.effect {
            ActorEffect::Heal(amount) => {
                if let Ok((mut hp, max, _)) = q_defend.get_mut(tar) {
                    let amount = i32::min(amount, max.0 - hp.0);
                    if amount <= 0 {
                        continue;
                    }
                    hp.0 += amount;
                                  
                    // TODO: Move this into ui? No reason to handle it here, would make it simpler + cleaner
                    if let Ok(actor_name) = q_names.get(actor) {
                        if let Ok(target_name) = q_names.get(tar) {
                            log.push(format!("{} heals {} for {} damage.", actor_name.as_str(), target_name.as_str(), amount));
                        }
                    }
                }
            },
            ActorEffect::Damage(amount) => {
                if let Ok(attack) = q_attack.get(actor) {
                    if let Ok((mut hp, _, def)) = q_defend.get_mut(tar) {
                        let amount = amount - def.0;

                        if amount <= 0 {
                            continue;
                        }
                        hp.0 -= amount;

                    // TODO: Move this into ui? No reason to handle it here, would make it simpler + cleaner
                        if let Ok(actor_name) = q_names.get(actor) {
                            if let Ok(target_name) = q_names.get(tar) {

                                log.push(format!("{} attacks {} for {} damage.", actor_name.as_str(), target_name.as_str(), amount));
                            } 
                        } 
                    }
                }
            },
        };
    }
}

fn death_system(
    mut commands: Commands,
    mut log: ResMut<PrintLog>,
    mut obstacles: ResMut<MapObstacles>,
    mut blockers: ResMut<MapActors>,
    q_combatants: Query<(Entity, &HitPoints, &Position, &Name)>,
    mut evt_killed: EventWriter<ActorKilledEvent>,
) {
    for (entity, hp, pos, name) in q_combatants.iter() {
        if hp.0 <= 0 {
            commands.entity(entity).despawn();
            let pos = IVec2::from(pos.0).as_uvec2();
            obstacles.0[pos] = false;
            blockers.0[pos] = None;
            
            evt_killed.send(ActorKilledEvent{
                name: name.to_string()
            });
            // TODO: Move to UI
            log.push(format!("{} was killed!", name.as_str()));
        }
    } 
}