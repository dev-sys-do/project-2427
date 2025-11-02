use std::io::{Write, stdin, stdout};

use crate::{
    player::PlayerBehavior,
    types::{Error, Grid, PlayerID, Position},
};

/// A player that interacts via a terminal (stdout and stdin)
pub struct TerminalPlayer;

impl TerminalPlayer {
    pub fn new() -> Self {
        TerminalPlayer
    }

    fn read_position(&self) -> crate::Result<Position> {
        print!("Enter your move (0-8): ");
        stdout().flush().map_err(|e| Error::Other(e.to_string()))?;

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .map_err(|e| Error::Other(e.to_string()))?;

        match input.trim().parse::<u8>() {
            Ok(num) if num < 9 => return Ok(num),
            _ => Err(Error::InvalidInput),
        }
    }

    fn reset_screen(&self) {
        // Clear the terminal screen
        print!("\x1B[2J\x1B[H");
        stdout().flush().unwrap();
    }

    fn print_grid(&self, grid: Grid) {
        self.reset_screen();

        println!("You are playing Tic-Tac-Toe!");
        println!("X = You  |  O = Other player  |  numbers = Available Positions");
        println!();

        for i in 0..9 {
            match grid[i] {
                Some(PlayerID::Player1) => print!(" X "),
                Some(PlayerID::Player2) => print!(" O "),
                None => print!(" {} ", i),
            }
            if i % 3 == 2 {
                println!();
            }
        }
    }
}

impl PlayerBehavior for TerminalPlayer {
    fn game_start(&mut self, _me: PlayerID) {
        println!("Game starts");
    }

    fn play(&self, grid: Grid) -> crate::Result<Position> {
        self.print_grid(grid);
        loop {
            match self.read_position() {
                Ok(pos) => {
                    if grid[pos as usize].is_none() {
                        return Ok(pos);
                    } else {
                        println!("Position {} is already taken. Try again.", pos);
                    }
                }
                Err(Error::InvalidInput) => {
                    println!("Invalid input. Please enter a number between 0 and 8.");
                }
                Err(e) => return Err(e),
            }
        }
    }

    fn game_ended(&self, grid: Grid, winner: bool) {
        self.print_grid(grid);
        if winner {
            println!("You won!");
        } else {
            println!("You lost!");
        }
    }
}
