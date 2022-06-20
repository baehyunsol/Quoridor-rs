use crate::player::Player;
use super::Game;

pub struct GameSaveData {
    pub player1: Player,
    pub player2: Player,
    pub clock: f32,
    pub vertical_walls: Vec<Vec<bool>>,
    pub horizontal_walls: Vec<Vec<bool>>,
    pub cross_walls: Vec<Vec<bool>>,
    pub player1_turn: bool,
}

impl GameSaveData {

    pub fn dummy() -> Self {
        GameSaveData {
            player1: Player::new(true),
            player2: Player::new(false),
            clock: 0.0,
            vertical_walls: vec![vec![false; 9]; 10],
            horizontal_walls: vec![vec![false; 9]; 10],
            cross_walls: vec![vec![false; 10]; 10],
            player1_turn: true
        }
    }

    pub fn from_game(game: &Game) -> Self {
        GameSaveData {
            player1: game.player1.clone(),
            player2: game.player2.clone(),
            clock: game.clock,
            vertical_walls: game.vertical_walls.clone(),
            horizontal_walls: game.horizontal_walls.clone(),
            cross_walls: game.cross_walls.clone(),
            player1_turn: game.player1_turn
        }
    }
}