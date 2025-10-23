mod board;

use board::{Board, Player};

fn main() {
    println!("=== Tic-Tac-Toe Board Demo ===\n");
    
    let mut board = Board::new();
    
    println!("Empty board:");
    board.display();
    
    // Demo: Place some moves to show the board in action
    println!("Demo: Placing some moves...");
    board.place_move(5, Player::X);
    board.place_move(1, Player::O);
    board.place_move(9, Player::X);
    board.display();
    
    println!("Run 'cargo test' to see all the unit tests for the board functionality!");
}
