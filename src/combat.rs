use bevy::prelude::*;

use crate::{
    components::Position,
    //    ui::PrintLog,
    map_state::{MapActors, PathBlocker, PathingData},
    ui::LogMessage,
    xy_to_index,
};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_attack).add_observer(on_actor_killed);
    }
}

#[derive(Component, Clone, Default)]
pub struct HitPoints {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Default)]
pub struct Defense {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Default)]
pub struct Strength {
    pub current: i32,
    pub max: i32,
}

#[derive(Component, Clone, Default, Debug)]
pub struct Dice {
    pub dice: i32,
    pub faces: i32,
}

impl Dice {
    pub fn roll(&self) -> i32 {
        assert!(self.dice > 0 && self.faces > 0);

        let mut i: i32 = 0;
        for _ in 0..self.dice {
            i += rand::random_range(1..=self.faces);
        }
        i
    }
}

#[derive(Event)]
pub struct AttackEvent {
    pub actor: Entity,
    pub target: Entity,
}

#[derive(Event)]
pub struct ActorKilledEvent {
    actor: Entity,
}

fn on_attack(
    e: On<AttackEvent>,
    q_attacker: Query<(&Strength, &Dice)>,
    mut q_target: Query<(&mut HitPoints, &Defense)>,
    q_name: Query<&Name>,
    mut commands: Commands,
) {
    let Ok((mut tar_hp, tar_def)) = q_target.get_mut(e.target) else {
        // TODO: Proper error handling
        panic!("Target even initiated with no target");
    };
    let Ok((attack, dice)) = q_attacker.get(e.actor) else {
        // TODO: Proper error handling
        panic!("Attack event initiated with no attacker");
    };

    let damage = attack.current + dice.roll() - tar_def.current;

    if damage <= 0 {
        if let Ok(actor_name) = q_name.get(e.actor)
            && let Ok(target_name) = q_name.get(e.target)
        {
            commands.write_message(LogMessage(format!(
                "{} tried to attack {} but did no damage.",
                actor_name, target_name,
            )));
            return;
        }
    }

    tar_hp.current = (tar_hp.current - damage).max(0);
    if let Ok(actor_name) = q_name.get(e.actor)
        && let Ok(target_name) = q_name.get(e.target)
    {
        commands.write_message(LogMessage(format!(
            "{} attacks {} for <fg=red>{}</fg> damage.",
            actor_name, target_name, damage
        )));
    }
    if tar_hp.current == 0 {
        commands.trigger(ActorKilledEvent { actor: e.target });
    }
}

// TODO: Move to somewhere else?
fn on_actor_killed(
    e: On<ActorKilledEvent>,
    mut commands: Commands,
    mut pathing: ResMut<PathingData>,
    mut actors: ResMut<MapActors>,
    q_name: Query<&Name>,
    q_pos: Query<&Position>,
    q_blocker: Query<&PathBlocker>,
) {
    if let Ok(pos) = q_pos.get(e.actor) {
        let i = xy_to_index(pos.0);
        actors.0[i] = None;
        if q_blocker.get(e.actor).is_ok() {
            pathing.0.remove_obstacle(pos.0);
        }
    }

    if let Ok(name) = q_name.get(e.actor) {
        commands.write_message(LogMessage(format!("{} was killed!", name)));
    }
    commands.entity(e.actor).despawn();
}
