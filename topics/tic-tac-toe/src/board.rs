//! Board module - Game state representation

use std::fmt;

/// Board size constant
const BOARD_SIZE: usize = 3;

/// Represents a cell on the tic-tac-toe board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    X,
    O,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " "),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O"),
        }
    }
}

/// Represents the 3x3 tic-tac-toe board
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    cells: [[Cell; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    /// Creates a new empty board
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; BOARD_SIZE]; BOARD_SIZE],
        }
    }

    /// Gets the cell at the specified position
    pub fn get(&self, row: usize, col: usize) -> Option<Cell> {
        if row < BOARD_SIZE && col < BOARD_SIZE {
            Some(self.cells[row][col])
        } else {
            None
        }
    }

    /// Sets the cell at the specified position
    /// Returns true if the move was valid (cell was empty), false otherwise
    pub fn set(&mut self, row: usize, col: usize, cell: Cell) -> bool {
        if row < BOARD_SIZE && col < BOARD_SIZE && self.cells[row][col] == Cell::Empty {
            self.cells[row][col] = cell;
            true
        } else {
            false
        }
    }

    /// Checks if the specified position is empty
    pub fn is_empty(&self, row: usize, col: usize) -> bool {
        self.get(row, col) == Some(Cell::Empty)
    }

    /// Returns true if the board is full
    pub fn is_full(&self) -> bool {
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.cells[row][col] == Cell::Empty {
                    return false;
                }
            }
        }
        true
    }

    /// Gets all empty positions on the board
    pub fn empty_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                if self.cells[row][col] == Cell::Empty {
                    positions.push((row, col));
                }
            }
        }
        positions
    }

    /// Checks if there's a winner and returns the winning cell type
    pub fn check_winner(&self) -> Option<Cell> {
        // Check rows
        for row in 0..BOARD_SIZE {
            if self.cells[row][0] != Cell::Empty
                && self.cells[row][0] == self.cells[row][1]
                && self.cells[row][1] == self.cells[row][2]
            {
                return Some(self.cells[row][0]);
            }
        }

        // Check columns
        for col in 0..BOARD_SIZE {
            if self.cells[0][col] != Cell::Empty
                && self.cells[0][col] == self.cells[1][col]
                && self.cells[1][col] == self.cells[2][col]
            {
                return Some(self.cells[0][col]);
            }
        }

        // Check main diagonal (top-left to bottom-right)
        if self.cells[0][0] != Cell::Empty
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][2]
        {
            return Some(self.cells[0][0]);
        }

        // Check anti-diagonal (top-right to bottom-left)
        if self.cells[0][2] != Cell::Empty
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[1][1] == self.cells[2][0]
        {
            return Some(self.cells[0][2]);
        }

        None
    }

    /// Returns true if the game is over (either someone won or board is full)
    pub fn is_game_over(&self) -> bool {
        self.check_winner().is_some() || self.is_full()
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  0   1   2")?;
        for row in 0..BOARD_SIZE {
            write!(f, "{} ", row)?;
            for col in 0..BOARD_SIZE {
                write!(f, "{}", self.cells[row][col])?;
                if col < BOARD_SIZE - 1 {
                    write!(f, " | ")?;
                }
            }
            writeln!(f)?;
            if row < BOARD_SIZE - 1 {
                writeln!(f, "  ---------")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        assert!(board.is_empty(0, 0));
        assert!(board.is_empty(2, 2));
        assert!(!board.is_full());
        assert!(board.check_winner().is_none());
    }

    #[test]
    fn test_set_and_get() {
        let mut board = Board::new();
        assert!(board.set(1, 1, Cell::X));
        assert_eq!(board.get(1, 1), Some(Cell::X));
        assert!(!board.set(1, 1, Cell::O)); // Can't overwrite
    }

    #[test]
    fn test_winner_detection() {
        let mut board = Board::new();

        // Set up a winning row
        board.set(0, 0, Cell::X);
        board.set(0, 1, Cell::X);
        board.set(0, 2, Cell::X);

        assert_eq!(board.check_winner(), Some(Cell::X));
    }

    #[test]
    fn test_all_winning_conditions() {
        // Test all 8 possible winning combinations

        // Test all rows
        for row in 0..BOARD_SIZE {
            let mut board = Board::new();
            for col in 0..BOARD_SIZE {
                board.set(row, col, Cell::O);
            }
            assert_eq!(
                board.check_winner(),
                Some(Cell::O),
                "Row {} should be a win",
                row
            );
        }

        // Test all columns
        for col in 0..BOARD_SIZE {
            let mut board = Board::new();
            for row in 0..BOARD_SIZE {
                board.set(row, col, Cell::X);
            }
            assert_eq!(
                board.check_winner(),
                Some(Cell::X),
                "Column {} should be a win",
                col
            );
        }

        // Test main diagonal (top-left to bottom-right)
        let mut board = Board::new();
        for i in 0..BOARD_SIZE {
            board.set(i, i, Cell::O);
        }
        assert_eq!(
            board.check_winner(),
            Some(Cell::O),
            "Main diagonal should be a win"
        );

        // Test anti-diagonal (top-right to bottom-left)
        let mut board = Board::new();
        for i in 0..BOARD_SIZE {
            board.set(i, BOARD_SIZE - 1 - i, Cell::X);
        }
        assert_eq!(
            board.check_winner(),
            Some(Cell::X),
            "Anti-diagonal should be a win"
        );
    }

    #[test]
    fn test_draw_detection() {
        let mut board = Board::new();

        // Create a draw scenario: X O X / O X O / O X O
        board.set(0, 0, Cell::X);
        board.set(0, 1, Cell::O);
        board.set(0, 2, Cell::X);
        board.set(1, 0, Cell::O);
        board.set(1, 1, Cell::X);
        board.set(1, 2, Cell::O);
        board.set(2, 0, Cell::O);
        board.set(2, 1, Cell::X);
        board.set(2, 2, Cell::O);

        assert!(board.is_full());
        assert!(board.check_winner().is_none());
        assert!(board.is_game_over());
    }
}
