use bevy::{
    app::Plugin,
    state::{app::AppExtStates, state::States},
};

#[derive(States, Debug, Hash, Default, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    #[default]
    Exploring,
    Inventory,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.init_state::<GameState>();
    }
}
