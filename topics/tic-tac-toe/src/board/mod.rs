pub mod lines;

/// Represents a player in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Human,
    Ai,
}

impl Player {
    pub fn opponent(self) -> Self {
        match self {
            Player::Human => Player::Ai,
            Player::Ai => Player::Human,
        }
    }

    /// Returns the symbol for this player
    pub fn symbol(self) -> char {
        match self {
            Player::Human => 'X',
            Player::Ai => 'O',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

impl Cell {
    /// Returns true if the cell is empty
    pub fn is_empty(self) -> bool {
        matches!(self, Cell::Empty)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    cells: [Cell; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [Cell::Empty; 9],
        }
    }

    /// Returns the cell at the given position (0-8)
    pub fn get(&self, position: usize) -> Option<Cell> {
        self.cells.get(position).copied()
    }

    /// Sets the cell at the given position
    /// Returns true if the move was valid (cell was empty), false otherwise
    pub fn set(&mut self, position: usize, player: Player) -> bool {
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

    /// Returns an iterator over all empty positions on the board
    pub fn empty_positions(&self) -> impl Iterator<Item = usize> + '_ {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, cell)| if cell.is_empty() { Some(i) } else { None })
    }

    /// Returns true if the board is full
    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|cell| !cell.is_empty())
    }

    /// Displays the board (simple ASCII version)
    #[allow(dead_code)]
    pub fn display(&self) {
        println!(
            " {} | {} | {}",
            self.cell_char(0),
            self.cell_char(1),
            self.cell_char(2)
        );
        println!("-----------");
        println!(
            " {} | {} | {}",
            self.cell_char(3),
            self.cell_char(4),
            self.cell_char(5)
        );
        println!("-----------");
        println!(
            " {} | {} | {}",
            self.cell_char(6),
            self.cell_char(7),
            self.cell_char(8)
        );
    }

    /// Returns the character to display for a cell
    fn cell_char(&self, position: usize) -> char {
        match self.cells[position] {
            Cell::Empty => (b'1' + position as u8) as char,
            Cell::Occupied(player) => player.symbol(),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}
