use crate::board::Board;
use crate::types::{Cell, Player};

/// Represents the current state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    /// Game is still in progress
    InProgress,
    /// A player has won
    Won(Player),
    /// Game ended in a draw
    Draw,
}

/// Represents the game logic and state
pub struct Game {
    board: Board,
    current_player: Player,
    state: GameState,
}

impl Game {
    /// Creates a new game with the human player starting
    pub fn new() -> Self {
        Game {
            board: Board::new(),
            current_player: Player::Human,
            state: GameState::InProgress,
        }
    }

    /// Creates a game from an existing board state
    pub fn from_board(board: Board, current_player: Player) -> Self {
        let mut game = Game {
            board,
            current_player,
            state: GameState::InProgress,
        };
        game.update_state();
        game
    }

    /// Returns a reference to the current board
    pub fn board(&self) -> &Board {
        &self.board
    }

    /// Returns the current player
    pub fn current_player(&self) -> Player {
        self.current_player
    }

    /// Returns the current game state
    pub fn state(&self) -> GameState {
        self.state
    }

    /// Makes a move at the given position for the current player
    /// Returns true if the move was successful, false otherwise
    pub fn make_move(&mut self, position: usize) -> bool {
        // Check if game is already over
        if self.state != GameState::InProgress {
            return false;
        }

        // Try to make the move
        if !self.board.make_move(position, self.current_player) {
            return false;
        }

        // Update game state
        self.update_state();

        // Switch player if game is still in progress
        if self.state == GameState::InProgress {
            self.current_player = self.current_player.opponent();
        }

        true
    }

    /// Updates the game state by checking for wins or draws
    fn update_state(&mut self) {
        // Check if current player won
        if self.check_winner(self.current_player) {
            self.state = GameState::Won(self.current_player);
            return;
        }

        // Check for draw (board is full and no winner)
        if self.board.is_full() {
            self.state = GameState::Draw;
        }
    }

    /// Checks if the given player has won the game
    pub fn check_winner(&self, player: Player) -> bool {
        let cells = self.board.cells();
        let target = Cell::Occupied(player);

        // Check rows
        for row in 0..3 {
            if cells[row * 3] == target
                && cells[row * 3 + 1] == target
                && cells[row * 3 + 2] == target
            {
                return true;
            }
        }

        // Check columns
        for col in 0..3 {
            if cells[col] == target && cells[col + 3] == target && cells[col + 6] == target {
                return true;
            }
        }

        // Check diagonals
        // Top-left to bottom-right
        if cells[0] == target && cells[4] == target && cells[8] == target {
            return true;
        }

        // Top-right to bottom-left
        if cells[2] == target && cells[4] == target && cells[6] == target {
            return true;
        }

        false
    }

    /// Returns a list of available moves
    pub fn available_moves(&self) -> Vec<usize> {
        self.board.available_moves()
    }

    /// Evaluates the current board state for the minimax algorithm
    /// Returns: +10 for AI win, -10 for Human win, 0 for draw or in progress
    pub fn evaluate(&self) -> i32 {
        if self.check_winner(Player::AI) {
            10
        } else if self.check_winner(Player::Human) {
            -10
        } else {
            0
        }
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
        assert_eq!(game.state(), GameState::InProgress);
        assert_eq!(game.current_player(), Player::Human);
    }

    #[test]
    fn test_horizontal_win() {
        let mut game = Game::new();
        // Human wins with top row
        game.make_move(0); // Human X
        game.make_move(3); // AI O
        game.make_move(1); // Human X
        game.make_move(4); // AI O
        game.make_move(2); // Human X - wins!

        assert_eq!(game.state(), GameState::Won(Player::Human));
    }

    #[test]
    fn test_vertical_win() {
        let mut game = Game::new();
        // AI wins with left column
        game.make_move(1); // Human X
        game.make_move(0); // AI O
        game.make_move(2); // Human X
        game.make_move(3); // AI O
        game.make_move(4); // Human X
        game.make_move(6); // AI O - wins!

        assert_eq!(game.state(), GameState::Won(Player::AI));
    }

    #[test]
    fn test_diagonal_win() {
        let mut game = Game::new();
        // Human wins with diagonal
        game.make_move(0); // Human X
        game.make_move(1); // AI O
        game.make_move(4); // Human X
        game.make_move(2); // AI O
        game.make_move(8); // Human X - wins!

        assert_eq!(game.state(), GameState::Won(Player::Human));
    }

    #[test]
    fn test_draw() {
        let mut game = Game::new();
        // Create a draw scenario
        game.make_move(0); // Human X
        game.make_move(1); // AI O
        game.make_move(2); // Human X
        game.make_move(4); // AI O
        game.make_move(3); // Human X
        game.make_move(5); // AI O
        game.make_move(7); // Human X
        game.make_move(6); // AI O
        game.make_move(8); // Human X

        assert_eq!(game.state(), GameState::Draw);
    }

    #[test]
    fn test_invalid_move() {
        let mut game = Game::new();
        game.make_move(0); // Human X
        assert!(!game.make_move(0)); // Try to play same position
    }
}
