mod game;

use crate::game::game::Game;

// Main game entry point
fn main() {
    println!("Starting Tic Tac Toe game..." );
    let mut game = Game::new();

    game.play();
}
