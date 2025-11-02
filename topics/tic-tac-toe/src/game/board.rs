use super::{Player, rules::{GameRules, GameState}};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Board {
    cells: [Option<Player>; 9],
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [None; 9],
        }
    }
    
    pub fn make_move(&mut self, position: usize, player: Player) -> Result<(), BoardError> {
        if !GameRules::is_valid_move(&self.cells, position) {
            return Err(BoardError::InvalidMove(position));
        }
        
        self.cells[position] = Some(player);
        Ok(())
    }
    
    pub fn game_state(&self) -> GameState {
        GameRules::game_state(&self.cells)
    }
    
    pub fn available_moves(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(idx, cell)| if cell.is_none() { Some(idx) } else { None })
            .collect()
    }
    
    pub fn get_cell(&self, position: usize) -> Option<Player> {
        if position < 9 {
            self.cells[position]
        } else {
            None
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.cells.iter().all(|cell| cell.is_none())
    }
    
    pub fn clone_for_simulation(&self) -> Self {
        self.clone()
    }
    
    #[allow(dead_code)]
    pub fn from_state(cells: [Option<Player>; 9]) -> Self {
        Self { cells }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..3 {
            write!(f, " ")?;
            for col in 0..3 {
                let idx = row * 3 + col;
                let symbol = match self.cells[idx] {
                    Some(player) => player.symbol(),
                    None => ' ',
                };
                write!(f, "{}", symbol)?;
                if col < 2 {
                    write!(f, " | ")?;
                }
            }
            writeln!(f)?;
            if row < 2 {
                writeln!(f, "-----------")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    InvalidMove(usize),
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::InvalidMove(pos) => {
                write!(f, "Invalid move: position {} is already occupied or out of bounds", pos + 1)
            }
        }
    }
}

impl std::error::Error for BoardError {}
