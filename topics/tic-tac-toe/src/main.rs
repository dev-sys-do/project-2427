mod ai;
mod board;
mod game;
mod types;

use ai::AI;
use game::{Game, GameState};
use std::io::{self, Write};
use types::Player;

fn main() {
    println!("=================================");
    println!("   Welcome to Tic-Tac-Toe!");
    println!("=================================");
    println!();
    println!("You are X, AI is O");
    println!("Enter positions 1-9 as shown:");
    println!();
    display_position_guide();
    println!();

    let mut game = Game::new();
    let ai = AI::new();

    loop {
        // Display the current board
        game.board().display();

        // Check game state
        match game.state() {
            GameState::Won(Player::Human) => {
                println!("Congratulations! You won!");
                break;
            }
            GameState::Won(Player::AI) => {
                println!("AI wins! Better luck next time!");
                break;
            }
            GameState::Draw => {
                println!("It's a draw! Well played!");
                break;
            }
            GameState::InProgress => {
                // Game continues
            }
        }

        // Current player's turn
        if game.current_player() == Player::Human {
            // Human turn
            println!("Your turn (X)");
            let position = get_human_move(&game);

            if !game.make_move(position) {
                println!("Invalid move! Try again.");
                continue;
            }
        } else {
            // AI turn
            println!("AI is thinking...");

            if let Some(position) = ai.find_best_move(&game) {
                game.make_move(position);
                println!("AI played position {}", position + 1);
            } else {
                println!("Error: AI couldn't find a move!");
                break;
            }
        }
    }

    println!();
    println!("Thanks for playing!");
}

/// Gets a valid move from the human player
fn get_human_move(game: &Game) -> usize {
    loop {
        print!("Enter position (1-9): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Try to parse the input
        match input.trim().parse::<usize>() {
            Ok(num) if (1..=9).contains(&num) => {
                let position = num - 1; // Convert to 0-indexed

                // Check if position is available
                if game.available_moves().contains(&position) {
                    return position;
                } else {
                    println!("That position is already taken! Try another.");
                }
            }
            _ => {
                println!("Invalid input! Please enter a number between 1 and 9.");
            }
        }
    }
}

/// Displays the position guide (how positions are numbered)
fn display_position_guide() {
    println!("   1 | 2 | 3");
    println!("  -----------");
    println!("   4 | 5 | 6");
    println!("  -----------");
    println!("   7 | 8 | 9");
}
