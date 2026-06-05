use bevy::state::state::States;

#[derive(States, Debug, Hash, Default, Eq, PartialEq, Clone, Copy)]
pub enum GameState {
    #[default]
    Exploring,
    Inventory,
}
