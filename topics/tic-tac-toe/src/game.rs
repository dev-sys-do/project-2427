//! Game module - Main game logic

use crate::ai::AiAgent;
use crate::board::{Board, Cell};
use std::fmt;

/// Board size constant
const BOARD_SIZE: usize = 3;

/// Represents the two players in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Human,
    Ai,
}

/// Represents the possible game outcomes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameResult {
    HumanWin,
    AiWin,
    Draw,
}

/// Represents errors that can occur during gameplay
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameError {
    InvalidPosition,
    PositionOccupied,
    GameOver,
    WrongPlayer,
}

impl fmt::Display for GameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameError::InvalidPosition => write!(f, "Invalid position (must be 0-2)"),
            GameError::PositionOccupied => write!(f, "Position is already occupied"),
            GameError::GameOver => write!(f, "Game is already over"),
            GameError::WrongPlayer => write!(f, "Not your turn"),
        }
    }
}

impl std::error::Error for GameError {}

/// Main game controller that manages the tic-tac-toe game
pub struct Game {
    board: Board,
    current_player: Player,
    ai_agent: AiAgent,
}

impl Game {
    /// Creates a new game with the human player going first
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::Human,
            ai_agent: AiAgent::new(),
        }
    }

    /// Returns the current player
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    /// Returns a reference to the current board
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Displays the current board state
    pub fn display_board(&self) {
        println!("{}", self.board);
    }

    /// Makes a move for the human player
    pub fn make_human_move(&mut self, row: usize, col: usize) -> Result<(), GameError> {
        // Check if game is over
        if self.board.is_game_over() {
            return Err(GameError::GameOver);
        }

        // Check if it's the human player's turn
        if self.current_player != Player::Human {
            return Err(GameError::WrongPlayer);
        }

        // Validate position
        if row >= BOARD_SIZE || col >= BOARD_SIZE {
            return Err(GameError::InvalidPosition);
        }

        // Check if position is empty
        if !self.board.is_empty(row, col) {
            return Err(GameError::PositionOccupied);
        }

        // Make the move
        self.board.set(row, col, Cell::X);

        // Switch to AI player if game is not over
        if !self.board.is_game_over() {
            self.current_player = Player::Ai;
        }

        Ok(())
    }

    /// Makes a move for the AI player
    pub fn make_ai_move(&mut self) -> Result<(), GameError> {
        // Check if game is over
        if self.board.is_game_over() {
            return Err(GameError::GameOver);
        }

        // Check if it's the AI player's turn
        if self.current_player != Player::Ai {
            return Err(GameError::WrongPlayer);
        }

        // Get the best move from the AI
        if let Some((row, col)) = self.ai_agent.get_best_move(&self.board) {
            self.board.set(row, col, Cell::O);

            // Switch to human player if game is not over
            if !self.board.is_game_over() {
                self.current_player = Player::Human;
            }

            Ok(())
        } else {
            // This should not happen if the game logic is correct
            Err(GameError::GameOver)
        }
    }

    /// Checks if the game is over and returns the result
    pub fn check_game_over(&self) -> Option<GameResult> {
        if let Some(winner) = self.board.check_winner() {
            match winner {
                Cell::X => Some(GameResult::HumanWin),
                Cell::O => Some(GameResult::AiWin),
                Cell::Empty => None, // This should never happen
            }
        } else if self.board.is_full() {
            Some(GameResult::Draw)
        } else {
            None
        }
    }

    /// Resets the game to initial state
    pub fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = Player::Human;
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert_eq!(game.current_player(), Player::Human);
        assert!(game.check_game_over().is_none());
    }

    #[test]
    fn test_human_move() {
        let mut game = Game::new();
        assert!(game.make_human_move(1, 1).is_ok());
        assert_eq!(game.current_player(), Player::Ai);
        assert_eq!(game.board().get(1, 1), Some(Cell::X));
    }

    #[test]
    fn test_invalid_moves() {
        let mut game = Game::new();

        // Test invalid position
        assert_eq!(game.make_human_move(3, 3), Err(GameError::InvalidPosition));

        // Test occupied position
        game.make_human_move(0, 0).unwrap();
        game.make_ai_move().unwrap(); // Switch turns
        assert_eq!(game.make_human_move(0, 0), Err(GameError::PositionOccupied));
    }

    #[test]
    fn test_wrong_player_errors() {
        let mut game = Game::new();

        // Test human trying to move when it's AI's turn
        game.make_human_move(1, 1).unwrap(); // Human moves first
        assert_eq!(game.current_player(), Player::Ai);
        assert_eq!(game.make_human_move(0, 0), Err(GameError::WrongPlayer));

        // Test AI trying to move when it's human's turn
        game.make_ai_move().unwrap(); // AI moves
        assert_eq!(game.current_player(), Player::Human);
        assert_eq!(game.make_ai_move(), Err(GameError::WrongPlayer));
    }

    #[test]
    fn test_game_over_scenarios() {
        // Manually set up a winning condition by directly manipulating the board
        // This bypasses the AI logic and ensures we can test game over behavior
        let mut board = Board::new();
        board.set(0, 0, Cell::X);
        board.set(0, 1, Cell::X);
        board.set(0, 2, Cell::X); // X wins

        // Create a new game with this winning board
        let mut winning_game = Game::new();
        winning_game.board = board;

        // Verify game is over
        assert_eq!(winning_game.check_game_over(), Some(GameResult::HumanWin));

        // Try to make moves after game is over
        assert_eq!(winning_game.make_human_move(1, 1), Err(GameError::GameOver));
        assert_eq!(winning_game.make_ai_move(), Err(GameError::GameOver));
    }

    #[test]
    fn test_game_reset() {
        let mut game = Game::new();
        game.make_human_move(1, 1).unwrap();
        game.reset();

        assert_eq!(game.current_player(), Player::Human);
        assert!(game.board().is_empty(1, 1));
    }

    #[test]
    fn test_complete_game_flow_ai_win() {
        let mut game = Game::new();

        // Simulate a game where AI wins
        // This specific sequence should result in AI victory
        game.make_human_move(0, 0).unwrap(); // X at (0,0)
        assert_eq!(game.current_player(), Player::Ai);

        game.make_ai_move().unwrap(); // AI takes center or strategic position
        assert_eq!(game.current_player(), Player::Human);

        game.make_human_move(0, 1).unwrap(); // X at (0,1)
        game.make_ai_move().unwrap(); // AI blocks or creates threat

        game.make_human_move(1, 0).unwrap(); // X at (1,0)
        game.make_ai_move().unwrap(); // AI should win or block

        // Continue until game ends
        while game.check_game_over().is_none() {
            if game.current_player() == Player::Human {
                // Find any valid move for human
                let empty_positions = game.board().empty_positions();
                if let Some((row, col)) = empty_positions.first() {
                    let _ = game.make_human_move(*row, *col);
                }
            } else {
                let _ = game.make_ai_move();
            }
        }

        // Game should end in AI win or draw (AI should never lose)
        let result = game.check_game_over().unwrap();
        assert!(result == GameResult::AiWin || result == GameResult::Draw);
    }

    #[test]
    fn test_draw_game_flow() {
        let mut game = Game::new();

        // Test that AI can handle full game scenarios properly
        // Let the AI and human alternate moves until game ends
        let mut move_count = 0;
        const MAX_MOVES: i32 = 9; // Maximum possible moves in tic-tac-toe

        while game.check_game_over().is_none() && move_count < MAX_MOVES {
            if game.current_player() == Player::Human {
                // Find first available empty position
                let empty_positions = game.board().empty_positions();
                if let Some((row, col)) = empty_positions.first() {
                    if game.make_human_move(*row, *col).is_ok() {
                        move_count += 1;
                    }
                } else {
                    break; // No more moves available
                }
            } else {
                if game.make_ai_move().is_ok() {
                    move_count += 1;
                } else {
                    break; // AI can't move
                }
            }
        }

        // Game should end with either a result or a full board
        let result = game.check_game_over();
        assert!(result.is_some() || game.board().is_full());

        // With optimal AI play, human should never win
        if let Some(game_result) = result {
            assert!(game_result != GameResult::HumanWin);
        }
    }
}
