use crate::board::{Board, Player, Cell};

/// Represents the game state
#[derive(Debug, PartialEq)]
pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

/// Main game controller that manages the tic-tac-toe game logic
pub struct Game {
    board: Board,
    current_player: Player,
    state: GameState,
}

impl Game {
    /// Creates a new game instance
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            current_player: Player::X, // X always starts first
            state: GameState::InProgress,
        }
    }

    /// Places a move for the current player at the specified position
    /// Returns true if the move was successful, false otherwise
    pub fn make_move(&mut self, position: usize) -> bool {
        if self.state != GameState::InProgress {
            return false;
        }

        if self.board.place_move(position, self.current_player) {
            self.update_game_state();
            if self.state == GameState::InProgress {
                self.switch_player();
            }
            true
        } else {
            false
        }
    }

    /// Gets the current game state
    pub fn get_state(&self) -> &GameState {
        &self.state
    }

    /// Gets the current player
    pub fn get_current_player(&self) -> Player {
        self.current_player
    }

    /// Gets a reference to the board
    pub fn get_board(&self) -> &Board {
        &self.board
    }

    /// Checks if a position is valid and empty
    pub fn is_valid_move(&self, position: usize) -> bool {
        self.state == GameState::InProgress && self.board.is_empty(position)
    }

    /// Gets all available moves
    pub fn get_available_moves(&self) -> Vec<usize> {
        if self.state != GameState::InProgress {
            Vec::new()
        } else {
            self.board.get_empty_positions()
        }
    }

    /// Resets the game to initial state
    pub fn reset(&mut self) {
        self.board = Board::new();
        self.current_player = Player::X;
        self.state = GameState::InProgress;
    }

    /// Displays the current board
    pub fn display_board(&self) {
        self.board.display();
    }

    /// Checks if there's a winner and returns the winning player
    pub fn check_winner(&self) -> Option<Player> {
        let winning_combinations = [
            // Rows
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            // Columns
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            // Diagonals
            [0, 4, 8], [2, 4, 6],
        ];

        for combo in &winning_combinations {
            let pos1 = combo[0] + 1;
            let pos2 = combo[1] + 1;
            let pos3 = combo[2] + 1;
            
            if let (Some(Cell::Occupied(p1)), Some(Cell::Occupied(p2)), Some(Cell::Occupied(p3))) = 
                (self.board.get_cell(pos1), self.board.get_cell(pos2), self.board.get_cell(pos3)) {
                if p1 == p2 && p2 == p3 {
                    return Some(p1);
                }
            }
        }
        None
    }

    /// Checks if the game is over (either someone won or board is full)
    pub fn is_game_over(&self) -> bool {
        self.check_winner().is_some() || self.board.is_full()
    }

    /// Updates the game state based on current board
    fn update_game_state(&mut self) {
        if let Some(winner) = self.check_winner() {
            self.state = GameState::Won(winner);
        } else if self.board.is_full() {
            self.state = GameState::Draw;
        }
    }

    /// Switches to the next player
    fn switch_player(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_game() {
        let game = Game::new();
        assert_eq!(game.current_player, Player::X);
        assert_eq!(game.state, GameState::InProgress);
        assert_eq!(game.get_available_moves().len(), 9);
    }

    #[test]
    fn test_make_valid_move() {
        let mut game = Game::new();
        
        // X plays first
        assert!(game.make_move(5));
        assert_eq!(game.current_player, Player::O);
        assert_eq!(game.state, GameState::InProgress);
        
        // O plays next
        assert!(game.make_move(1));
        assert_eq!(game.current_player, Player::X);
        assert_eq!(game.state, GameState::InProgress);
    }

    #[test]
    fn test_make_invalid_move() {
        let mut game = Game::new();
        
        // Valid move first
        assert!(game.make_move(5));
        
        // Try to play same position
        assert!(!game.make_move(5));
        assert_eq!(game.current_player, Player::O);
        
        // Try invalid position
        assert!(!game.make_move(0));
        assert!(!game.make_move(10));
    }

    #[test]
    fn test_winning_game() {
        let mut game = Game::new();
        
        // X wins in top row
        game.make_move(1); // X
        game.make_move(4); // O
        game.make_move(2); // X
        game.make_move(5); // O
        game.make_move(3); // X wins
        
        assert_eq!(game.state, GameState::Won(Player::X));
        assert_eq!(game.get_available_moves().len(), 0);
        
        assert!(!game.make_move(6));
    }

    #[test]
    fn test_draw_game() {
        let mut game = Game::new();
        
        // Create a true draw scenario - no three in a row for anyone
        // X O X
        // O X X
        // O X O
        game.make_move(1); // X
        game.make_move(2); // O
        game.make_move(3); // X
        game.make_move(4); // O
        game.make_move(5); // X
        game.make_move(9); // O
        game.make_move(6); // X
        game.make_move(7); // O
        game.make_move(8); // X
        
        assert_eq!(game.state, GameState::Draw);
        assert_eq!(game.get_available_moves().len(), 0);
    }

    #[test]
    fn test_is_valid_move() {
        let mut game = Game::new();
        
        for pos in 1..=9 {
            assert!(game.is_valid_move(pos));
        }
        
        game.make_move(5);
        assert!(!game.is_valid_move(5));
        assert!(game.is_valid_move(1));
        
        // Invalid positions
        assert!(!game.is_valid_move(0));
        assert!(!game.is_valid_move(10));
    }

    #[test]
    fn test_reset_game() {
        let mut game = Game::new();
        
        game.make_move(1);
        game.make_move(2);
        game.make_move(3);
        
        game.reset();
        
        assert_eq!(game.current_player, Player::X);
        assert_eq!(game.state, GameState::InProgress);
        assert_eq!(game.get_available_moves().len(), 9);
    }

    #[test]
    fn test_game_state_after_win() {
        let mut game = Game::new();
        
        // X wins diagonally
        game.make_move(1); // X
        game.make_move(2); // O
        game.make_move(5); // X
        game.make_move(3); // O
        game.make_move(9); // X wins
        
        assert_eq!(game.state, GameState::Won(Player::X));
        
        assert!(!game.is_valid_move(4));
        assert_eq!(game.get_available_moves().len(), 0);
    }

    #[test]
    fn test_check_winner_horizontal() {
        let mut game = Game::new();
        
        game.make_move(1); // X
        game.make_move(4); // O
        game.make_move(2); // X
        game.make_move(5); // O
        game.make_move(3); // X wins
        assert_eq!(game.check_winner(), Some(Player::X));
        
        let mut game = Game::new();
        game.make_move(1); // X
        game.make_move(4); // O
        game.make_move(2); // X
        game.make_move(5); // O
        game.make_move(7); // X
        game.make_move(6); // O wins
        assert_eq!(game.check_winner(), Some(Player::O));
        
        let mut game = Game::new();
        game.make_move(7); // X
        game.make_move(1); // O
        game.make_move(8); // X
        game.make_move(2); // O
        game.make_move(9); // X wins
        assert_eq!(game.check_winner(), Some(Player::X));
    }

    #[test]
    fn test_check_winner_vertical() {
        let mut game = Game::new();
        game.make_move(1); // X
        game.make_move(2); // O
        game.make_move(4); // X
        game.make_move(3); // O
        game.make_move(7); // X wins
        assert_eq!(game.check_winner(), Some(Player::X));
        
        let mut game = Game::new();
        game.make_move(1); // X
        game.make_move(2); // O
        game.make_move(3); // X
        game.make_move(5); // O
        game.make_move(4); // X
        game.make_move(8); // O wins
        assert_eq!(game.check_winner(), Some(Player::O));
        
        let mut game = Game::new();
        game.make_move(3); // X
        game.make_move(1); // O
        game.make_move(6); // X
        game.make_move(2); // O
        game.make_move(9); // X wins
        assert_eq!(game.check_winner(), Some(Player::X));
    }

    #[test]
    fn test_check_winner_diagonal() {
        let mut game = Game::new();
        game.make_move(1); // X
        game.make_move(2); // O
        game.make_move(5); // X
        game.make_move(3); // O
        game.make_move(9); // X wins
        assert_eq!(game.check_winner(), Some(Player::X));
        
        let mut game = Game::new();
        game.make_move(1); // X
        game.make_move(3); // O
        game.make_move(2); // X
        game.make_move(5); // O
        game.make_move(6); // X
        game.make_move(7); // O wins
        assert_eq!(game.check_winner(), Some(Player::O));
    }

    #[test]
    fn test_check_winner_no_winner() {
        let mut game = Game::new();
        
        assert_eq!(game.check_winner(), None);
        
        game.make_move(1); // X
        game.make_move(2); // O
        game.make_move(4); // X
        assert_eq!(game.check_winner(), None);
        
        game.make_move(3); // O
        assert_eq!(game.check_winner(), None);
    }

    #[test]
    fn test_is_game_over() {
        let mut game = Game::new();
        
        assert!(!game.is_game_over());
        
        game.make_move(1); // X
        game.make_move(5); // O
        assert!(!game.is_game_over());
        
        game.make_move(2); // X
        game.make_move(6); // O
        game.make_move(3); // X wins
        assert!(game.is_game_over());
        assert_eq!(game.check_winner(), Some(Player::X));
    }

    #[test]
    fn test_is_game_over_full_board() {
        let mut game = Game::new();
        
        // X O X
        // O X X
        // O X O
        game.make_move(1); // X
        game.make_move(2); // O
        game.make_move(3); // X
        game.make_move(4); // O
        game.make_move(5); // X
        game.make_move(9); // O
        game.make_move(6); // X
        game.make_move(7); // O
        game.make_move(8); // X
        
        assert!(game.is_game_over());
        assert_eq!(game.check_winner(), None);
        assert!(game.get_board().is_full());
    }
}