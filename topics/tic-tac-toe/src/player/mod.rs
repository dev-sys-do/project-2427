use crate::types::{Grid, PlayerID, Position};

pub mod ai_minmax;
pub mod terminal;

/// Represents a player that can play a [`crate::logic::game::Game`]
pub trait PlayerBehavior {
    /// Called by Game() when the game starts
    fn game_start(&mut self, me: PlayerID);
    /// Called by Game() to get the player's next move. Implementation should return
    /// a valid (free and 0-8) position, or play() will be called again with the same grid. 
    fn play(&mut self, grid: Grid) -> crate::Result<Position>;
    /// Called by Game() when the game ends
    fn game_ended(&mut self, grid: Grid, winner: Option<PlayerID>);
}
