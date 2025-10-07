#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

/// Represents the current state of the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Ongoing,
    Win(Mark),
    Draw,
}

/// Represents a player's mark on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mark {
    X,
    O,
}

impl Mark {
    /// Returns the opponent's mark
    #[must_use]
    pub const fn opponent(self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

/// Represents a cell on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Filled(Mark),
}

/// Represents the game board state
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [Cell; 9],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /// Creates a new empty board
    #[must_use]
    pub const fn new() -> Self {
        Self {
            cells: [Cell::Empty; 9],
        }
    }

    /// Returns a string representation of the board for display
    /// Format: 3x3 grid with X, O, or position numbers (0-8) for empty cells
    #[must_use]
    pub fn display(&self) -> String {
        let mut result = String::new();
        for row in 0..3 {
            for col in 0..3 {
                let pos = row * 3 + col;
                let symbol = match self.cells[pos] {
                    Cell::Empty => pos.to_string(),
                    Cell::Filled(Mark::X) => "X".to_string(),
                    Cell::Filled(Mark::O) => "O".to_string(),
                };
                result.push_str(&symbol);
                if col < 2 {
                    result.push_str(" | ");
                }
            }
            if row < 2 {
                result.push_str("\n---------\n");
            }
        }
        result
    }

    /// Places a mark at the given position (0-8)
    ///
    /// # Errors
    /// Returns an error if the position is out of bounds (>= 9) or already occupied
    pub fn place_mark(&mut self, position: usize, mark: Mark) -> Result<(), &'static str> {
        if position >= 9 {
            return Err("Position out of bounds");
        }
        if self.cells[position] != Cell::Empty {
            return Err("Position already occupied");
        }
        self.cells[position] = Cell::Filled(mark);
        Ok(())
    }

    /// Checks if there is a winner and returns the winning mark
    #[must_use]
    pub fn check_winner(&self) -> Option<Mark> {
        // Define all winning lines (rows, columns, diagonals)
        const LINES: [[usize; 3]; 8] = [
            [0, 1, 2], // top row
            [3, 4, 5], // middle row
            [6, 7, 8], // bottom row
            [0, 3, 6], // left column
            [1, 4, 7], // middle column
            [2, 5, 8], // right column
            [0, 4, 8], // diagonal \
            [2, 4, 6], // diagonal /
        ];

        for line in &LINES {
            if let (Cell::Filled(a), Cell::Filled(b), Cell::Filled(c)) = (
                self.cells[line[0]],
                self.cells[line[1]],
                self.cells[line[2]],
            ) {
                if a == b && b == c {
                    return Some(a);
                }
            }
        }
        None
    }

    /// Returns true if all cells are filled
    #[must_use]
    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Cell::Empty)
    }

    /// Returns true if the game is a draw (board full with no winner)
    #[must_use]
    pub fn is_draw(&self) -> bool {
        self.is_full() && self.check_winner().is_none()
    }

    /// Returns the current game state
    #[must_use]
    pub fn game_state(&self) -> GameState {
        if let Some(winner) = self.check_winner() {
            GameState::Win(winner)
        } else if self.is_full() {
            GameState::Draw
        } else {
            GameState::Ongoing
        }
    }

    /// Returns a list of empty positions (legal moves)
    #[must_use]
    pub fn legal_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(
                |(i, &cell)| {
                    if cell == Cell::Empty { Some(i) } else { None }
                },
            )
            .collect()
    }

    /// Minimax algorithm: returns the best score for the current player
    /// Maximizing when it's the player's turn, minimizing when it's the opponent's turn
    #[must_use]
    fn minimax(&self, player: Mark, is_maximizing: bool) -> i32 {
        // Base case: if game is over, return evaluation
        if let Some(winner) = self.check_winner() {
            return if winner == player { 1 } else { -1 };
        }
        if self.is_full() {
            return 0;
        }

        let legal_moves = self.legal_moves();

        if is_maximizing {
            let mut best_score = i32::MIN;
            for position in legal_moves {
                let mut new_board = self.clone();
                new_board.place_mark(position, player).unwrap();
                let score = new_board.minimax(player, false);
                best_score = best_score.max(score);
            }
            best_score
        } else {
            let mut best_score = i32::MAX;
            let opponent = player.opponent();
            for position in legal_moves {
                let mut new_board = self.clone();
                new_board.place_mark(position, opponent).unwrap();
                let score = new_board.minimax(player, true);
                best_score = best_score.min(score);
            }
            best_score
        }
    }

    /// Finds the best move for the given player using Minimax algorithm
    /// Returns None if no legal moves are available
    ///
    /// # Panics
    /// This function should not panic as it only places marks on known legal positions
    #[must_use]
    pub fn best_move(&self, player: Mark) -> Option<usize> {
        let legal_moves = self.legal_moves();
        if legal_moves.is_empty() {
            return None;
        }

        let mut best_score = i32::MIN;
        let mut best_position = None;

        for position in legal_moves {
            let mut new_board = self.clone();
            new_board.place_mark(position, player).unwrap();
            let score = new_board.minimax(player, false);

            if score > best_score {
                best_score = score;
                best_position = Some(position);
            }
        }

        best_position
    }
}

fn main() {
    println!("Tic-Tac-Toe Agent !");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board_is_empty() {
        let board = Board::new();
        assert_eq!(board.cells, [Cell::Empty; 9]);
    }

    #[test]
    fn test_place_mark_success() {
        let mut board = Board::new();
        assert!(board.place_mark(0, Mark::X).is_ok());
        assert_eq!(board.cells[0], Cell::Filled(Mark::X));
    }

    #[test]
    fn test_place_mark_out_of_bounds() {
        let mut board = Board::new();
        assert!(board.place_mark(9, Mark::X).is_err());
    }

    #[test]
    fn test_place_mark_occupied() {
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        assert!(board.place_mark(0, Mark::O).is_err());
    }

    #[test]
    fn test_check_winner_row() {
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(1, Mark::X).unwrap();
        board.place_mark(2, Mark::X).unwrap();
        assert_eq!(board.check_winner(), Some(Mark::X));
    }

    #[test]
    fn test_check_winner_column() {
        let mut board = Board::new();
        board.place_mark(0, Mark::O).unwrap();
        board.place_mark(3, Mark::O).unwrap();
        board.place_mark(6, Mark::O).unwrap();
        assert_eq!(board.check_winner(), Some(Mark::O));
    }

    #[test]
    fn test_check_winner_diagonal() {
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(4, Mark::X).unwrap();
        board.place_mark(8, Mark::X).unwrap();
        assert_eq!(board.check_winner(), Some(Mark::X));
    }

    #[test]
    fn test_no_winner() {
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(1, Mark::O).unwrap();
        assert_eq!(board.check_winner(), None);
    }

    #[test]
    fn test_is_full() {
        let mut board = Board::new();
        assert!(!board.is_full());
        for i in 0..9 {
            board.place_mark(i, Mark::X).unwrap();
        }
        assert!(board.is_full());
    }

    #[test]
    fn test_is_draw() {
        let mut board = Board::new();
        // Create a draw scenario: X O X / O X X / O X O
        let moves = [
            (0, Mark::X),
            (1, Mark::O),
            (2, Mark::X),
            (3, Mark::O),
            (4, Mark::X),
            (5, Mark::X),
            (6, Mark::O),
            (7, Mark::X),
            (8, Mark::O),
        ];
        for (pos, mark) in &moves {
            board.place_mark(*pos, *mark).unwrap();
        }
        assert!(board.is_draw());
        assert!(board.check_winner().is_none());
    }

    #[test]
    fn test_not_draw_when_winner_exists() {
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(1, Mark::X).unwrap();
        board.place_mark(2, Mark::X).unwrap();
        assert!(!board.is_draw());
    }

    #[test]
    fn test_legal_moves_empty_board() {
        let board = Board::new();
        assert_eq!(board.legal_moves(), vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_legal_moves_partial_board() {
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(4, Mark::O).unwrap();
        board.place_mark(8, Mark::X).unwrap();
        assert_eq!(board.legal_moves(), vec![1, 2, 3, 5, 6, 7]);
    }

    #[test]
    fn test_legal_moves_full_board() {
        let mut board = Board::new();
        for i in 0..9 {
            board.place_mark(i, Mark::X).unwrap();
        }
        assert_eq!(board.legal_moves(), Vec::<usize>::new());
    }

    #[test]
    fn test_mark_opponent() {
        assert_eq!(Mark::X.opponent(), Mark::O);
        assert_eq!(Mark::O.opponent(), Mark::X);
    }

    #[test]
    fn test_game_state_ongoing() {
        let mut board = Board::new();
        assert_eq!(board.game_state(), GameState::Ongoing);

        board.place_mark(0, Mark::X).unwrap();
        assert_eq!(board.game_state(), GameState::Ongoing);
    }

    #[test]
    fn test_game_state_win() {
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(1, Mark::X).unwrap();
        board.place_mark(2, Mark::X).unwrap();
        assert_eq!(board.game_state(), GameState::Win(Mark::X));
    }

    #[test]
    fn test_game_state_draw() {
        let mut board = Board::new();
        let moves = [
            (0, Mark::X),
            (1, Mark::O),
            (2, Mark::X),
            (3, Mark::O),
            (4, Mark::X),
            (5, Mark::X),
            (6, Mark::O),
            (7, Mark::X),
            (8, Mark::O),
        ];
        for (pos, mark) in &moves {
            board.place_mark(*pos, *mark).unwrap();
        }
        assert_eq!(board.game_state(), GameState::Draw);
    }

    #[test]
    fn test_display_empty_board() {
        let board = Board::new();
        let display = board.display();
        assert!(display.contains("0 | 1 | 2"));
        assert!(display.contains("3 | 4 | 5"));
        assert!(display.contains("6 | 7 | 8"));
        assert!(display.contains("---------"));
    }

    #[test]
    fn test_display_partial_board() {
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(4, Mark::O).unwrap();
        board.place_mark(8, Mark::X).unwrap();

        let display = board.display();
        assert!(display.contains("X | 1 | 2"));
        assert!(display.contains("3 | O | 5"));
        assert!(display.contains("6 | 7 | X"));
    }

    #[test]
    fn test_display_full_board() {
        let mut board = Board::new();
        let moves = [
            (0, Mark::X),
            (1, Mark::O),
            (2, Mark::X),
            (3, Mark::O),
            (4, Mark::X),
            (5, Mark::X),
            (6, Mark::O),
            (7, Mark::X),
            (8, Mark::O),
        ];
        for (pos, mark) in &moves {
            board.place_mark(*pos, *mark).unwrap();
        }

        let display = board.display();
        assert!(display.contains("X | O | X"));
        assert!(display.contains("O | X | X"));
        assert!(display.contains("O | X | O"));
    }

    #[test]
    fn test_best_move_win_immediately() {
        // X X _ / O O _ / _ _ _
        // X should play position 2 to win
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(1, Mark::X).unwrap();
        board.place_mark(3, Mark::O).unwrap();
        board.place_mark(4, Mark::O).unwrap();

        let best = board.best_move(Mark::X);
        assert_eq!(best, Some(2));
    }

    #[test]
    fn test_best_move_block_opponent() {
        // O O _ / X _ _ / _ _ _
        // X should play position 2 to block O from winning
        let mut board = Board::new();
        board.place_mark(0, Mark::O).unwrap();
        board.place_mark(1, Mark::O).unwrap();
        board.place_mark(3, Mark::X).unwrap();

        let best = board.best_move(Mark::X);
        assert_eq!(best, Some(2));
    }

    #[test]
    fn test_best_move_empty_board() {
        // On an empty board, any move is optimal
        // Common strategy: center (4) or corner (0, 2, 6, 8)
        let board = Board::new();
        let best = board.best_move(Mark::X);
        assert!(best.is_some());
    }

    #[test]
    fn test_best_move_full_board() {
        let mut board = Board::new();
        for i in 0..9 {
            board.place_mark(i, Mark::X).unwrap();
        }
        let best = board.best_move(Mark::X);
        assert_eq!(best, None);
    }

    #[test]
    fn test_minimax_detects_win() {
        // X X _ / _ _ _ / _ _ _
        // X to move: should evaluate to +1 (can win)
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(1, Mark::X).unwrap();

        let score = board.minimax(Mark::X, true);
        assert_eq!(score, 1);
    }

    #[test]
    fn test_minimax_detects_loss() {
        // O O _ / X _ _ / _ _ _
        // X to move: should evaluate to -1 if O gets to move next in that branch
        // But X can block, so let's test a losing position
        // O O O / X X _ / _ _ _ - O already won
        let mut board = Board::new();
        board.place_mark(0, Mark::O).unwrap();
        board.place_mark(1, Mark::O).unwrap();
        board.place_mark(2, Mark::O).unwrap();
        board.place_mark(3, Mark::X).unwrap();
        board.place_mark(4, Mark::X).unwrap();

        let score = board.minimax(Mark::X, true);
        assert_eq!(score, -1);
    }

    #[test]
    fn test_minimax_detects_draw() {
        // Near-draw position where best outcome is draw
        // X O X / O X X / O X _
        let mut board = Board::new();
        board.place_mark(0, Mark::X).unwrap();
        board.place_mark(1, Mark::O).unwrap();
        board.place_mark(2, Mark::X).unwrap();
        board.place_mark(3, Mark::O).unwrap();
        board.place_mark(4, Mark::X).unwrap();
        board.place_mark(5, Mark::X).unwrap();
        board.place_mark(6, Mark::O).unwrap();
        board.place_mark(7, Mark::X).unwrap();

        let score = board.minimax(Mark::O, true);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_ai_never_loses() {
        // Test that if AI plays optimally from start, it never loses
        // X (AI) plays first, O plays suboptimally but AI should draw or win
        let mut board = Board::new();

        // AI (X) plays first move
        let best = board.best_move(Mark::X).unwrap();
        board.place_mark(best, Mark::X).unwrap();

        // O plays a corner (only if not already taken)
        let o_move = if board.cells[0] == Cell::Empty { 0 } else { 2 };
        board.place_mark(o_move, Mark::O).unwrap();

        // AI plays second move
        let best = board.best_move(Mark::X).unwrap();
        board.place_mark(best, Mark::X).unwrap();

        // Check that from this position, AI can at least draw
        let score = board.minimax(Mark::X, true);
        assert!(score >= 0, "AI should never lose from optimal play");
    }
}
