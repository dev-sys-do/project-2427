use crate::types::{Cell, Player};

/// Represents the game board (3x3 grid)
#[derive(Debug, Clone)]
pub struct Board {
    /// Internal representation as a 1D array of 9 cells
    cells: [Cell; 9],
}

impl Board {
    /// Creates a new empty board
    pub fn new() -> Self {
        Board {
            cells: [Cell::Empty; 9],
        }
    }

    /// Returns the cell at the given position (0-8)
    pub fn get(&self, position: usize) -> Option<Cell> {
        self.cells.get(position).copied()
    }

    /// Places a player's mark at the given position
    /// Returns true if the move was successful, false otherwise
    pub fn make_move(&mut self, position: usize, player: Player) -> bool {
        if position >= 9 {
            return false;
        }

        if self.cells[position].is_empty() {
            self.cells[position] = Cell::Occupied(player);
            true
        } else {
            false
        }
    }

    /// Returns a list of all available moves (empty cell positions)
    pub fn available_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, cell)| cell.is_empty())
            .map(|(idx, _)| idx)
            .collect()
    }

    /// Returns true if the board is full (no available moves)
    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|cell| !cell.is_empty())
    }

    /// Display the board
    pub fn display(&self) {
        println!("\n");
        for row in 0..3 {
            print!(" ");
            for col in 0..3 {
                let idx = row * 3 + col;
                print!(" {} ", self.cells[idx].symbol());
                if col < 2 {
                    print!("|");
                }
            }
            println!();
            if row < 2 {
                println!(" -----------");
            }
        }
        println!("\n");
    }

    /// Returns the internal cells array (for testing purposes)
    pub fn cells(&self) -> &[Cell; 9] {
        &self.cells
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
