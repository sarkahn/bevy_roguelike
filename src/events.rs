use bevy::prelude::*;

pub struct AttackEvent {
    pub attacker_name: String,
    pub defender_name: String,
}

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AttackEvent>();
    }
}
