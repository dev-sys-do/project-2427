use crate::game::{Board, Player, GameState};

pub struct Strategy;

impl Strategy {
    pub fn evaluate_terminal_state(state: &GameState) -> i32 {
        match state {
            GameState::Win(Player::AI) => 1,
            GameState::Win(Player::Human) => -1,
            GameState::Draw => 0,
            GameState::InProgress => 0,
        }
    }
    
    pub fn move_priority(position: usize) -> i32 {
        match position {
            4 => 3,
            0 | 2 | 6 | 8 => 2,
            1 | 3 | 5 | 7 => 1,
            _ => 0,
        }
    }
    
    pub fn order_moves(moves: Vec<usize>) -> Vec<usize> {
        let mut ordered_moves = moves;
        ordered_moves.sort_by(|a, b| Self::move_priority(*b).cmp(&Self::move_priority(*a)));
        ordered_moves
    }
    
    pub fn heuristic_evaluation(board: &Board) -> i32 {
        let state = board.game_state();
        match state {
            GameState::Win(_) | GameState::Draw => Self::evaluate_terminal_state(&state),
            GameState::InProgress => {
                0
            }
        }
    }
}
