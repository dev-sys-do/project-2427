use std::io::{Write, stdin, stdout};

use crate::{
    player::PlayerBehavior,
    types::{Error, Grid, PlayerID, Position},
};

/// A player that interacts via a terminal (stdout and stdin)
pub struct TerminalPlayer;

fn read_position() -> crate::Result<Position> {
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

impl PlayerBehavior for TerminalPlayer {
    fn game_start(&mut self, _me: PlayerID) {
        println!("Game starts");
    }

    fn play(&self, grid: Grid) -> crate::Result<Position> {
        loop {
            match read_position() {
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
        if winner {
            println!("You won!");
        } else {
            println!("You lost!");
        }
    }
}
