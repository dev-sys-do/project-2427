#[derive(Clone)]
pub struct Board {
    cells: [[Option<char>; 3]; 3]
}

impl Board {
    pub fn new() -> Self {
        Board {
            cells: [[None; 3]; 3]
        }
    }

    pub fn display(&self) {
        println!("Choose a position (1-9):");
        let mut position = 1;
        for row in &self.cells {
            for cell in row {
                match cell {
                    Some(symbol) => print!("| {} |", symbol),
                    None => print!("| {} |", position),
                }
                position += 1;
            }
            println!();
        }
    }

    pub fn place_symbol_by_position(&mut self, position: usize, symbol: char) -> bool {
        if position < 1 || position > 9 {
            return false;
        }
        
        let (row, col) = self.position_to_coords(position);
        if self.cells[row][col].is_none() {
            self.cells[row][col] = Some(symbol);
            true
        } else {
            false
        }
    }

    fn position_to_coords(&self, position: usize) -> (usize, usize) {
        let index = position - 1; // Convert to 0-based index
        let row = index / 3;
        let col = index % 3;
        (row, col)
    }

    pub fn is_full(&self) -> bool {
        for row in &self.cells {
            for cell in row {
                if cell.is_none() {
                    return false;
                }
            }
        }
        true
    }

    pub fn check_winner(&self) -> Option<char> {
        // Check rows
        for row in &self.cells {
            if let Some(symbol) = row[0] {
                if row[1] == Some(symbol) && row[2] == Some(symbol) {
                    return Some(symbol);
                }
            }
        }

        // Check columns
        for col in 0..3 {
            if let Some(symbol) = self.cells[0][col] {
                if self.cells[1][col] == Some(symbol) && self.cells[2][col] == Some(symbol) {
                    return Some(symbol);
                }
            }
        }

        // Check diagonals
        // Top-left to bottom-right
        if let Some(symbol) = self.cells[0][0] {
            if self.cells[1][1] == Some(symbol) && self.cells[2][2] == Some(symbol) {
                return Some(symbol);
            }
        }

        // Top-right to bottom-left
        if let Some(symbol) = self.cells[0][2] {
            if self.cells[1][1] == Some(symbol) && self.cells[2][0] == Some(symbol) {
                return Some(symbol);
            }
        }

        None
    }

    pub fn is_position_empty(&self, row: usize, col: usize) -> bool {
        if row < 3 && col < 3 {
            self.cells[row][col].is_none()
        } else {
            false
        }
    }
}