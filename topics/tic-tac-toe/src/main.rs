pub mod logic;
pub mod player;
pub mod types;

pub use types::Result;

fn main() {
    let p1 = player::terminal::TerminalPlayer::new();
    let p2 = player::ai_minmax::AIMinMax::new();
    let game = logic::game::Game::new(p1, p2);
    match game.play() {
        Ok(Some(winner)) => {
            println!("Player {:?} wins!", winner);
        }
        Ok(None) => {
            println!("It's a tie!");
        }
        Err(_e) => {
            eprintln!("An error occurred");
        }
    }
}
