use crate::{
    player::PlayerBehavior,
    types::{Grid, Position},
};

/// A player simulated using the Min-Max algorithm
pub struct AIMinMax;

impl PlayerBehavior for AIMinMax {
    fn game_start(&self) {}

    fn play(&self, grid: Grid) -> crate::Result<Position> {
        todo!();
    }

    fn game_ended(&self, grid: Grid, winner: bool) {
        todo!()
    }
}
