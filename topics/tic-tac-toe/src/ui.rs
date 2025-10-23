use crate::game::{Game, GameState};
use crate::board::Player;
use std::io::{self, Write};

/// Handles user interface operations for the tic-tac-toe game
pub struct UI;

impl UI {
    /// Starts the interactive game loop
    pub fn play_game() {
        println!("🎮 Welcome to Tic-Tac-Toe! 🎮");
        println!("You are X, computer will be O.");
        println!("Enter positions 1-9 corresponding to board positions:\n");
        
        Self::show_position_guide();
        
        loop {
            let mut game = Game::new();
            Self::play_round(&mut game);
            
            if !Self::ask_play_again() {
                println!("Thanks for playing! 👋");
                break;
            }
        }
    }

    /// Plays a single round of the game
    fn play_round(game: &mut Game) {
        println!("\n🆕 Starting new game!\n");
        
        while *game.get_state() == GameState::InProgress {
            // Display current board
            game.display_board();
            
            match game.get_current_player() {
                Player::X => {
                    // Human player's turn
                    Self::handle_human_turn(game);
                }
                Player::O => {
                    // For now, let's make it human vs human
                    // Later we'll add AI here
                    Self::handle_human_turn_for_o(game);
                }
            }
        }
        
        // Game over - show final state
        game.display_board();
        Self::show_game_result(game.get_state());
    }

    /// Handles human player X's turn
    fn handle_human_turn(game: &mut Game) {
        loop {
            let position = Self::get_user_input("Player X, enter your move (1-9): ");
            
            if game.make_move(position) {
                break;
            } else {
                if position >= 1 && position <= 9 {
                    println!("❌ Position {} is already occupied! Try again.", position);
                } else {
                    println!("❌ Invalid position! Please enter a number between 1 and 9.");
                }
            }
        }
    }

    /// Handles human player O's turn (temporary - will be replaced by AI)
    fn handle_human_turn_for_o(game: &mut Game) {
        loop {
            let position = Self::get_user_input("Player O, enter your move (1-9): ");
            
            if game.make_move(position) {
                break;
            } else {
                if position >= 1 && position <= 9 {
                    println!("❌ Position {} is already occupied! Try again.", position);
                } else {
                    println!("❌ Invalid position! Please enter a number between 1 and 9.");
                }
            }
        }
    }

    /// Gets user input and validates it
    fn get_user_input(prompt: &str) -> usize {
        loop {
            print!("{}", prompt);
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.trim().parse::<usize>() {
                        Ok(num) => return num,
                        Err(_) => {
                            println!("❌ Please enter a valid number!");
                        }
                    }
                }
                Err(_) => {
                    println!("❌ Error reading input! Please try again.");
                }
            }
        }
    }

    /// Shows the position guide to help users understand the board layout
    fn show_position_guide() {
        println!("Board positions:");
        println!(" 1 | 2 | 3 ");
        println!("-----------");
        println!(" 4 | 5 | 6 ");
        println!("-----------");
        println!(" 7 | 8 | 9 ");
        println!();
    }

    /// Shows the game result
    fn show_game_result(state: &GameState) {
        match state {
            GameState::Won(Player::X) => {
                println!("🎉 Congratulations! Player X wins! 🎉");
            }
            GameState::Won(Player::O) => {
                println!("🎉 Congratulations! Player O wins! 🎉");
            }
            GameState::Draw => {
                println!("🤝 It's a draw! Well played both players! 🤝");
            }
            GameState::InProgress => {
                println!("🤔 Game is still in progress...");
            }
        }
    }

    /// Asks if the user wants to play again
    fn ask_play_again() -> bool {
        loop {
            print!("\n🔄 Would you like to play again? (y/n): ");
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    match input.trim().to_lowercase().as_str() {
                        "y" | "yes" => return true,
                        "n" | "no" => return false,
                        _ => println!("❌ Please enter 'y' for yes or 'n' for no."),
                    }
                }
                Err(_) => {
                    println!("❌ Error reading input! Please try again.");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_game_flow_integration() {
        // Test that UI can create and interact with game properly
        let mut game = Game::new();
        
        // Simulate a simple game sequence
        assert_eq!(*game.get_state(), GameState::InProgress);
        assert_eq!(game.get_current_player(), Player::X);
        
        // Make moves like UI would
        assert!(game.make_move(1)); // X
        assert_eq!(game.get_current_player(), Player::O);
        
        assert!(game.make_move(2)); // O  
        assert_eq!(game.get_current_player(), Player::X);
        
        // Verify game state is still in progress
        assert_eq!(*game.get_state(), GameState::InProgress);
    }
    
    #[test]
    fn test_ui_handles_invalid_moves() {
        let mut game = Game::new();
        
        // Valid move first
        assert!(game.make_move(5));
        
        // UI should handle these invalid moves gracefully
        assert!(!game.make_move(5));  // Already occupied
        assert!(!game.make_move(0));  // Invalid position
        assert!(!game.make_move(10)); // Invalid position
        
        // Player should still be O's turn after failed moves
        assert_eq!(game.get_current_player(), Player::O);
    }
    
    #[test]
    fn test_ui_game_completion() {
        let mut game = Game::new();
        
        // Play a complete game (X wins)
        game.make_move(1); // X
        game.make_move(4); // O
        game.make_move(2); // X
        game.make_move(5); // O
        game.make_move(3); // X wins
        
        // UI should recognize game is over
        assert_eq!(*game.get_state(), GameState::Won(Player::X));
        
        // No more moves should be possible
        assert!(!game.make_move(6));
        assert!(!game.make_move(7));
    }
    
    #[test]
    fn test_ui_draw_scenario() {
        let mut game = Game::new();
        
        // Play to a draw
        game.make_move(1); // X
        game.make_move(2); // O
        game.make_move(3); // X
        game.make_move(4); // O
        game.make_move(5); // X
        game.make_move(9); // O
        game.make_move(6); // X
        game.make_move(7); // O
        game.make_move(8); // X
        
        // UI should recognize draw
        assert_eq!(*game.get_state(), GameState::Draw);
        assert!(!game.make_move(1)); // No more moves possible
    }
}