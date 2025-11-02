use super::Player;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Win(Player),
    Draw,
}

pub struct GameRules;

impl GameRules {
    const WINNING_COMBINATIONS: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    
    pub fn check_winner(board: &[Option<Player>; 9]) -> Option<Player> {
        for combination in &Self::WINNING_COMBINATIONS {
            if let (Some(a), Some(b), Some(c)) = (
                board[combination[0]],
                board[combination[1]], 
                board[combination[2]]
            ) {
                if a == b && b == c {
                    return Some(a);
                }
            }
        }
        None
    }
    
    pub fn is_board_full(board: &[Option<Player>; 9]) -> bool {
        board.iter().all(|cell| cell.is_some())
    }
    
    pub fn game_state(board: &[Option<Player>; 9]) -> GameState {
        if let Some(winner) = Self::check_winner(board) {
            GameState::Win(winner)
        } else if Self::is_board_full(board) {
            GameState::Draw
        } else {
            GameState::InProgress
        }
    }
    
    pub fn is_valid_move(board: &[Option<Player>; 9], position: usize) -> bool {
        position < 9 && board[position].is_none()
    }
}
