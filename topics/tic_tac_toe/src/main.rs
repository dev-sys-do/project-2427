/// Module pour la gestion du plateau de jeu
mod board;
/// Module pour la gestion des joueurs (humain et IA)
mod player;
/// Module pour la gestion du déroulement du jeu
mod game;

use std::io::{self, Write};
use rand::Rng;
use board::Cell;
use player::human::HumanPlayer;
use player::ai::AIPlayer;
use game::{Game, GameState};

/// Point d'entrée du programme
fn main() {
    println!("Welcome to Tic Tac Toe");
    println!("You'll be playing against an unbeatable AI using the Minimax algorithm.");
    
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim().to_string();
    
    let mut rng = rand::thread_rng();
    let human_goes_first = rng.gen_bool(0.5);
    
    let players: Vec<Box<dyn player::Player>> = if human_goes_first {
        println!("{} (X) goes first", name);
        vec![
            Box::new(HumanPlayer::new(Cell::X, name)),
            Box::new(AIPlayer::new(Cell::O, "AI".to_string())),
        ]
    } else {
        println!("AI (X) goes first");
        vec![
            Box::new(AIPlayer::new(Cell::X, "AI".to_string())),
            Box::new(HumanPlayer::new(Cell::O, name)),
        ]
    };
    
    let mut game = Game::new(players);
    
    while matches!(game.state(), GameState::InProgress) {
        game.play_turn();
    }
    
    game.display_result();
}
