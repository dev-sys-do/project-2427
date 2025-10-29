//! Tic-Tac-Toe Game with AI

use std::io::{self, Write};
use tic_tac_toe::Game;

/// Board size constant
const BOARD_SIZE: usize = 3;

fn main() {
    println!("🎮 Welcome to Tic-Tac-Toe!");
    println!("You are playing as 'X' against the AI 'O'");
    println!("Enter your moves as coordinates (row, col) from 0-2");
    println!("Example: '1 2' places your mark at row 1, column 2");
    println!();

    let mut game = Game::new();

    loop {
        // Display the current board
        game.display_board();

        match game.current_player() {
            tic_tac_toe::Player::Human => match get_human_move() {
                Some((row, col)) => match game.make_human_move(row, col) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("❌ Invalid move: {}", e);
                        continue;
                    }
                },
                None => {
                    println!("👋 Thanks for playing!");
                    return;
                }
            },
            tic_tac_toe::Player::Ai => {
                println!("🤖 AI is thinking...");
                match game.make_ai_move() {
                    Ok(_) => println!("✅ AI made its move!"),
                    Err(e) => {
                        println!("❌ AI error: {}", e);
                        break;
                    }
                }
            }
        }

        if let Some(result) = game.check_game_over() {
            game.display_board();
            match result {
                tic_tac_toe::GameResult::HumanWin => println!("🎉 Congratulations! You won!"),
                tic_tac_toe::GameResult::AiWin => println!("🤖 AI wins! Better luck next time!"),
                tic_tac_toe::GameResult::Draw => println!("🤝 It's a draw! Good game!"),
            }
            break;
        }
    }
}

/// Get a move from the human player
fn get_human_move() -> Option<(usize, usize)> {
    loop {
        print!("Enter your move (row col) or 'quit' to exit: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim();

                if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("q") {
                    return None;
                }

                let parts: Vec<&str> = input.split_whitespace().collect();
                if parts.len() != 2 {
                    println!("❌ Please enter two numbers separated by a space (e.g., '1 2')");
                    continue;
                }

                match (parts[0].parse::<usize>(), parts[1].parse::<usize>()) {
                    (Ok(row), Ok(col)) => {
                        if row < BOARD_SIZE && col < BOARD_SIZE {
                            return Some((row, col));
                        } else {
                            println!("❌ Coordinates must be between 0 and {}", BOARD_SIZE - 1);
                        }
                    }
                    _ => {
                        println!("❌ Please enter valid numbers");
                    }
                }
            }
            Err(_) => {
                println!("❌ Error reading input");
                continue;
            }
        }
    }
}
