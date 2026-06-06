use bevy::prelude::*;

pub struct TurnSystemPlugin;

impl Plugin for TurnSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, turn_begin_system)
            .add_systems(PostUpdate, turn_end_system);
    }
}

/// When an actor's energy reaches or exceeds 100, it will be given a turn.
#[derive(Default, Debug, Component, Clone)]
pub struct Energy(pub i32);

/// Determines how frequently an actor gets to take their turn,
/// relative to other actors.
#[derive(Debug, Component, Clone, Default)]
pub struct Speed(pub i32);

/// A tag for actors that can perform actions and take turns.
#[derive(Default, Debug, Component, Clone)]
pub struct Actor;

/// A component that gets added to an actor when it's time for it to take it's turn.
#[derive(Debug, Component, Clone, Default)]
pub struct TakingATurn;

fn turn_begin_system(
    mut commands: Commands,
    mut q_waiting_actors: Query<
        (Entity, &mut Energy, &Speed, Option<&Name>),
        (With<Actor>, Without<TakingATurn>),
    >,
    q_acting_actors: Query<&Actor, (With<Energy>, With<Speed>, With<TakingATurn>)>,
) {
    if !q_acting_actors.is_empty() {
        return;
    }

    let mut actor_acting = false;
    while !actor_acting && !q_waiting_actors.is_empty() {
        for (entity, mut energy, speed, _name) in q_waiting_actors.iter_mut() {
            assert!(speed.0 > 0);
            energy.0 += speed.0;

            if energy.0 >= 100 {
                actor_acting = true;
                commands.entity(entity).insert(TakingATurn);
            }
        }
    }
}

fn turn_end_system(
    mut commands: Commands,
    q_actors: Query<(Entity, &Energy, Option<&Name>), (With<Actor>, With<TakingATurn>)>,
) {
    for (entity, energy, _name) in q_actors.iter() {
        if energy.0 < 100 {
            commands.entity(entity).remove::<TakingATurn>();
        }
    }
}
