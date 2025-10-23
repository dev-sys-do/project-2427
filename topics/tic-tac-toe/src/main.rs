mod board;
mod game;

use game::{Game, GameState};

fn main() {
    println!("=== Tic-Tac-Toe ===");
    println!("Simple demonstration of the game logic\n");
    
    let mut game = Game::new();
    
    // Show initial empty board
    println!("Initial board:");
    game.display_board();
    
    // Make a few moves to demonstrate
    println!("Making some sample moves...");
    game.make_move(5); // X center
    game.make_move(1); // O top-left
    game.make_move(9); // X bottom-right
    
    game.display_board();
    println!("Current player: {:?}", game.get_current_player());
    println!("Game state: {:?}", game.get_state());
    println!("Available moves: {:?}", game.get_available_moves());
    
    println!("\n🎮 Run 'cargo test' to see all unit tests!");
}
