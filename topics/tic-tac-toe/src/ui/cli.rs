use crate::game::Board;
use std::io::{self, Write};

#[allow(clippy::upper_case_acronyms)]
pub struct CLI;

impl CLI {
    pub fn new() -> Self {
        Self
    }
    
    pub fn display_board(&self, board: &Board) {
        println!("Current board:");
        println!("{}", board);
    }
    
    pub fn get_human_move(&self, board: &Board) -> Result<usize, CLIError> {
        loop {
            print!("Enter your move (1-9): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .map_err(|e| CLIError::InputError(e.to_string()))?;
            
            let input = input.trim();
            
            match input.parse::<usize>() {
                Ok(num) if (1..=9).contains(&num) => {
                    let position = num - 1;
                    
                    if board.get_cell(position).is_none() {
                        return Ok(position);
                    } else {
                        println!("❌ Position {} is already occupied! Try another position.", num);
                    }
                }
                Ok(_) => {
                    println!("❌ Please enter a number between 1 and 9.");
                }
                Err(_) => {
                    println!("❌ Invalid input! Please enter a number between 1 and 9.");
                }
            }
        }
    }
    
    #[allow(dead_code)]
    pub fn show_instructions(&self) {
        println!("📋 How to play:");
        println!("   • Enter a number (1-9) to place your X on the board");
        println!("   • Numbers correspond to positions as shown:");
        println!("     1 | 2 | 3");
        println!("     ---------");
        println!("     4 | 5 | 6");  
        println!("     ---------");
        println!("     7 | 8 | 9");
        println!("   • Try to get three X's in a row!");
        println!("   • The AI will try to stop you and win with O's");
        println!();
    }
    
    #[allow(dead_code)]
    pub fn ask_play_again(&self) -> bool {
        loop {
            print!("🔄 Would you like to play again? (y/n): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => return true,
                    "n" | "no" => return false,
                    _ => println!("❌ Please enter 'y' for yes or 'n' for no."),
                }
            }
        }
    }
}

impl Default for CLI {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub enum CLIError {
    InputError(String),
}

impl std::fmt::Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CLIError::InputError(msg) => write!(f, "Input error: {}", msg),
        }
    }
}

impl std::error::Error for CLIError {}
