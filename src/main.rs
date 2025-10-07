#![deny(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]

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
}
