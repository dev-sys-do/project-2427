use std::io::{Write, stdin, stdout};

use crate::{
    player::PlayerBehavior,
    types::{Error, Grid, PlayerID, Position},
};

const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_BOLD: &str = "\x1b[1m";
const COLOR_RESET: &str = "\x1b[0m";

/// A player that interacts via a terminal (stdout and stdin)
pub struct TerminalPlayer {
    old_grid: Option<Grid>,
    me: Option<PlayerID>,
}

impl TerminalPlayer {
    pub fn new() -> Self {
        TerminalPlayer {
            old_grid: None,
            me: None,
        }
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

    fn prepare_cell_for_print(&self, grid: Grid, index: usize) -> String {
        let me = self.me.expect("self.me should be set by game_start()");
        let s = match grid[index] {
            // me
            Some(player) if player == me => format!("{}X{}", COLOR_GREEN, COLOR_RESET),
            // other player
            Some(_) => format!("{}O{}", COLOR_RED, COLOR_RESET),
            // not player (early return)
            None => return format!("{}", index),
        };

        // If it was just placed, make it bold
        if let Some(old_grid) = self.old_grid
            && old_grid[index].is_none()
        {
            format!("{}{}", COLOR_BOLD, s)
        } else {
            s
        }
    }

    fn print_grid(&self, grid: Grid) {
        self.reset_screen();

        println!("You are playing Tic-Tac-Toe!");
        println!("X = You  |  O = Other player  |  numbers = Available Positions");
        println!();

        for i in 0..9 {
            print!(" {} ", self.prepare_cell_for_print(grid, i));
            if i % 3 == 2 {
                println!();
            }
        }
    }
}

impl PlayerBehavior for TerminalPlayer {
    fn game_start(&mut self, me: PlayerID) {
        println!("Game starts");
        self.me = Some(me);
    }

    fn play(&mut self, grid: Grid) -> crate::Result<Position> {
        self.print_grid(grid);
        loop {
            match self.read_position() {
                Ok(pos) => {
                    if grid[pos as usize].is_none() {
                        // Position validated
                        self.old_grid = Some(grid);
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

    fn game_ended(&mut self, grid: Grid, winner: bool) {
        self.print_grid(grid);
        if winner {
            println!("You won!");
        } else {
            println!("You lost!");
        }
    }
}
