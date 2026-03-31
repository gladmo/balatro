use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    BlindSelect,
    SelectingHand,
    HandPlayed,
    RoundEval,
    DrawToHand,
    NewRound,
    Shop,
    BoosterPack,
    GameOver,
    Win,
}
