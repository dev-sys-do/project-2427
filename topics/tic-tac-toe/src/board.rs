#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Occupied(Player),
}

#[derive(Debug, Clone)]
pub struct Board {
    /// Internal representation as a 1D array for simplicity
    /// Positions 0-8 correspond to board positions 1-9 for user input
    cells: [Cell; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [Cell::Empty; 9],
        }
    }

    /// Places a player's mark at the specified position (1-9)
    /// Returns true if the move was successful, false if invalid position or occupied
    pub fn place_move(&mut self, position: usize, player: Player) -> bool {
        // Validate position (1-9)
        if position == 0 || position > 9 {
            return false;
        }
        
        let index = position - 1;
        
        if self.cells[index] == Cell::Empty {
            self.cells[index] = Cell::Occupied(player);
            true
        } else {
            false
        }
    }

    /// Checks if a position is empty
    pub fn is_empty(&self, position: usize) -> bool {
        if position == 0 || position > 9 {
            return false;
        }
        self.cells[position - 1] == Cell::Empty
    }

    /// Gets the cell at a specific position (1-9)
    pub fn get_cell(&self, position: usize) -> Option<Cell> {
        if position == 0 || position > 9 {
            return None;
        }
        Some(self.cells[position - 1])
    }

    /// Returns all empty positions (1-9)
    pub fn get_empty_positions(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(i, &cell)| {
                if cell == Cell::Empty {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Checks if the board is full
    pub fn is_full(&self) -> bool {
        self.cells.iter().all(|&cell| cell != Cell::Empty)
    }

    /// Displays the board in a readable format
    pub fn display(&self) {
        println!();
        for row in 0..3 {
            for col in 0..3 {
                let position = row * 3 + col + 1;
                let display_char = match self.get_cell(position) {
                    Some(Cell::Empty) => " ".to_string(),
                    Some(Cell::Occupied(Player::X)) => "X".to_string(),
                    Some(Cell::Occupied(Player::O)) => "O".to_string(),
                    None => "?".to_string(),
                };
                
                print!(" {} ", display_char);
                if col < 2 {
                    print!("|");
                }
            }
            println!();
            if row < 2 {
                println!("-----------");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        
        for position in 1..=9 {
            assert_eq!(board.get_cell(position), Some(Cell::Empty));
            assert!(board.is_empty(position));
        }
        
        assert!(!board.is_full());
        
        assert_eq!(board.get_empty_positions().len(), 9);
        assert_eq!(board.get_empty_positions(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_place_move_valid() {
        let mut board = Board::new();
        
        assert!(board.place_move(5, Player::X));
        assert_eq!(board.get_cell(5), Some(Cell::Occupied(Player::X)));
        assert!(!board.is_empty(5));
        
        assert!(board.place_move(1, Player::O));
        assert_eq!(board.get_cell(1), Some(Cell::Occupied(Player::O)));
        assert!(!board.is_empty(1));
        
        assert_eq!(board.get_empty_positions().len(), 7);
        assert!(!board.get_empty_positions().contains(&1));
        assert!(!board.get_empty_positions().contains(&5));
    }

    #[test]
    fn test_place_move_invalid_position() {
        let mut board = Board::new();
        
        assert!(!board.place_move(0, Player::X));
        assert!(!board.place_move(10, Player::X));
        
        assert_eq!(board.get_empty_positions().len(), 9);
    }

    #[test]
    fn test_place_move_occupied_position() {
        let mut board = Board::new();
        
        assert!(board.place_move(5, Player::X));
        
        assert!(!board.place_move(5, Player::O));
        
        assert_eq!(board.get_cell(5), Some(Cell::Occupied(Player::X)));
    }

    #[test]
    fn test_get_cell_invalid_position() {
        let board = Board::new();
        
        assert_eq!(board.get_cell(0), None);
        assert_eq!(board.get_cell(10), None);
    }

    #[test]
    fn test_is_empty_invalid_position() {
        let board = Board::new();
        
        assert!(!board.is_empty(0));
        assert!(!board.is_empty(10));
    }

    #[test]
    fn test_is_full() {
        let mut board = Board::new();
        
        assert!(!board.is_full());
        
        for position in 1..=9 {
            let player = if position % 2 == 1 { Player::X } else { Player::O };
            board.place_move(position, player);
        }
        
        assert!(board.is_full());
        assert_eq!(board.get_empty_positions().len(), 0);
    }

    #[test]
    fn test_get_empty_positions() {
        let mut board = Board::new();
        
        let mut expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(board.get_empty_positions(), expected);
        
        board.place_move(1, Player::X);
        board.place_move(5, Player::O);
        board.place_move(9, Player::X);
        
        expected.retain(|&x| x != 1 && x != 5 && x != 9);
        assert_eq!(board.get_empty_positions(), expected);
        assert_eq!(board.get_empty_positions(), vec![2, 3, 4, 6, 7, 8]);
    }

    #[test]
    fn test_board_clone() {
        let mut original = Board::new();
        original.place_move(1, Player::X);
        original.place_move(5, Player::O);
        
        let cloned = original.clone();
        
        assert_eq!(cloned.get_cell(1), Some(Cell::Occupied(Player::X)));
        assert_eq!(cloned.get_cell(5), Some(Cell::Occupied(Player::O)));
        assert_eq!(cloned.get_empty_positions().len(), 7);
    }

    #[test]
    fn test_player_equality() {
        assert_eq!(Player::X, Player::X);
        assert_eq!(Player::O, Player::O);
        assert_ne!(Player::X, Player::O);
    }

    #[test]
    fn test_cell_equality() {
        assert_eq!(Cell::Empty, Cell::Empty);
        assert_eq!(Cell::Occupied(Player::X), Cell::Occupied(Player::X));
        assert_eq!(Cell::Occupied(Player::O), Cell::Occupied(Player::O));
        assert_ne!(Cell::Empty, Cell::Occupied(Player::X));
        assert_ne!(Cell::Occupied(Player::X), Cell::Occupied(Player::O));
    }
}