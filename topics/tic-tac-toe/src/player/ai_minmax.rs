use crate::{
    logic::grid,
    player::PlayerBehavior,
    types::{Grid, PlayerID, Position},
};

/// A player simulated using the Min-Max algorithm
pub struct AIMinMax {
    ai_player: Option<PlayerID>, // (me)
}

impl AIMinMax {
    pub fn new() -> Self {
        AIMinMax { ai_player: None }
    }

    fn ai_player(&self) -> PlayerID {
        self.ai_player
            .expect("self.ai_player should be set by game_start()")
    }

    fn opponent(&self) -> PlayerID {
        match self.ai_player() {
            PlayerID::Player1 => PlayerID::Player2,
            PlayerID::Player2 => PlayerID::Player1,
        }
    }

    /// Minimax algorithm implementation
    fn minimax(&self, mut grid: Grid, depth: i32, is_maximizing: bool) -> i32 {
        // Check if there is a winner yet
        match grid::is_there_a_win(grid) {
            // If AI has won, return score minus depth to prefer quicker wins
            Some(winner) if winner == self.ai_player() => {
                return 10 - depth;
            }
            // If opponent has won, return score plus depth to delay losses
            Some(_) => {
                return -10 + depth;
            }
            _ => {}
        };

        // If no moves left, it's a tie
        if !grid::are_there_moves_left(grid) {
            return 0;
        }

        if is_maximizing {
            let mut best = i32::MIN;

            for i in 0..9 {
                if grid[i].is_none() {
                    grid[i] = self.ai_player;
                    let value = self.minimax(grid, depth + 1, false);
                    grid[i] = None;
                    best = best.max(value);
                }
            }
            best
        } else {
            let mut best = i32::MAX;

            for i in 0..9 {
                if grid[i].is_none() {
                    grid[i] = Some(self.opponent());
                    let value = self.minimax(grid, depth + 1, true);
                    grid[i] = None;
                    best = best.min(value);
                }
            }
            best
        }
    }

    /// Find the best move using minimax algorithm
    fn find_best_move(&self, mut grid: Grid) -> Option<Position> {
        let mut best_val = i32::MIN;
        let mut best_move = None;

        for i in 0..9 {
            if grid[i].is_none() {
                grid[i] = self.ai_player; // Simulate AI move
                // After AI move, it's opponent's turn (so start with minimizing)
                let move_val = self.minimax(grid, 0, false);
                grid[i] = None; // Reset move

                if move_val > best_val {
                    best_move = Some(i as Position);
                    best_val = move_val;
                }
            }
        }

        best_move
    }
}

impl PlayerBehavior for AIMinMax {
    fn game_start(&mut self, me: PlayerID) {
        self.ai_player = Some(me);
    }

    fn play(&mut self, grid: Grid) -> crate::Result<Position> {
        if let Some(best_move) = self.find_best_move(grid) {
            Ok(best_move)
        } else {
            Err(crate::types::Error::Other(
                "No valid moves available".to_string(),
            ))
        }
    }

    fn game_ended(&mut self, _grid: Grid, _winner: Option<PlayerID>) {
        // AI doesn't need to do anything when game ends
    }
}
