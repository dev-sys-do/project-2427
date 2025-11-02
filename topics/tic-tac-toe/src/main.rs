use clap::Parser;

mod game;
mod ai;
mod ui;

use game::{Board, Player, GameState};
use ai::MiniMax;
use ui::CLI;

#[derive(Parser)]
#[command(name = "tic-tac-toe")]
#[command(about = "Play Tic-Tac-Toe against an intelligent AI opponent")]
struct Args {
    #[arg(short, long)]
    debug: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    println!("🎮 Welcome to Tic-Tac-Toe!");
    println!("You are X, AI is O\n");
    
    let mut board = Board::new();
    let mut current_player = Player::Human;
    let mut ai = MiniMax::new(args.debug);
    let cli = CLI::new();
    
    loop {
        cli.display_board(&board);
        
        match board.game_state() {
            GameState::Win(player) => {
                match player {
                    Player::Human => println!("🎉 Congratulations! You won!"),
                    Player::AI => println!("🤖 AI wins! Better luck next time!"),
                }
                break;
            }
            GameState::Draw => {
                println!("🤝 It's a draw! Well played!");
                break;
            }
            GameState::InProgress => {
            }
        }
        
        match current_player {
            Player::Human => {
                match cli.get_human_move(&board) {
                    Ok(position) => {
                        board.make_move(position, Player::Human)?;
                        current_player = Player::AI;
                    }
                    Err(e) => {
                        println!("❌ Error: {}", e);
                        continue;
                    }
                }
            }
            Player::AI => {
                println!("🤖 AI is thinking...");
                let ai_move = ai.get_best_move(&board);
                board.make_move(ai_move, Player::AI)?;
                println!("🤖 AI plays: {}\n", ai_move + 1);
                current_player = Player::Human;
            }
        }
    }
    
    println!("\n👋 Thanks for playing!");
    Ok(())
}
