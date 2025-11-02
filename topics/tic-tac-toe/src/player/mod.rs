use crate::types::{Grid, PlayerID, Position};

pub mod ai_minmax;
pub mod terminal;

/// Represents a player that can play a [`crate::logic::game::Game`]
pub trait PlayerBehavior {
    fn game_start(&mut self, me: PlayerID);
    fn play(&mut self, grid: Grid) -> crate::Result<Position>;
    fn game_ended(&mut self, grid: Grid, winner: Option<PlayerID>);
}
