use crate::{player::PlayerBehavior, types::{Grid, PlayerID, Position}};

/// A player that interacts via a terminal (stdout and stdin)
pub struct TerminalPlayer;

impl PlayerBehavior for TerminalPlayer {
    fn game_start(&self) {
        println!("Game starts");
    }

    fn play(&self, grid: Grid) -> crate::Result<Position> {
        todo!();
    }

    fn game_ended(&self, grid: Grid, winner: bool) {
        todo!()
    }
}