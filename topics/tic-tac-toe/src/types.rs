/// Represents a player in the game
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    /// Human player (X)
    Human,
    /// AI player (O)
    AI,
}

impl Player {
    /// Returns the opposite player
    pub fn opponent(&self) -> Player {
        match self {
            Player::Human => Player::AI,
            Player::AI => Player::Human,
        }
    }

    /// Returns the symbol representing this player
    pub fn symbol(&self) -> char {
        match self {
            Player::Human => 'X',
            Player::AI => 'O',
        }
    }
}

/// Represents a cell on the board
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Empty cell
    Empty,
    /// Cell occupied by a player
    Occupied(Player),
}

impl Cell {
    /// Returns true if the cell is empty
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    /// Returns the symbol representing this cell
    pub fn symbol(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Occupied(player) => player.symbol(),
        }
    }
}
