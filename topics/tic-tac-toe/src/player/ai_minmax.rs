use crate::types::{Grid, PlayerID};

use crate::{logic::grid, player::PlayerBehavior, types::Position};

/// A player simulated using the Min-Max algorithm
pub struct AIMinMax {
    ai_player: Option<PlayerID>, // (me)
}

impl Default for AIMinMax {
    fn default() -> Self {
        Self::new()
    }
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
#[cfg(test)]
mod tests {
    use super::*;

    fn empty_grid() -> Grid {
        [None; 9]
    }

    #[test]
    fn test_ai_chooses_winning_move() {
        let mut ai = AIMinMax::new();
        ai.game_start(PlayerID::Player1);
        let mut grid = empty_grid();
        grid[0] = Some(PlayerID::Player1);
        grid[1] = Some(PlayerID::Player1);
        grid[4] = Some(PlayerID::Player2);
        let mv = ai.play(grid).unwrap();
        assert_eq!(mv, 2);
    }

    #[test]
    fn test_ai_blocks_opponent_win() {
        // AI is Player2, must block Player1 at position 2
        let mut ai = AIMinMax::new();
        ai.game_start(PlayerID::Player2);
        let mut grid = empty_grid();
        grid[0] = Some(PlayerID::Player1);
        grid[1] = Some(PlayerID::Player1);
        grid[4] = Some(PlayerID::Player2);
        let mv = ai.play(grid).unwrap();
        assert_eq!(mv, 2);
    }

    #[test]
    fn test_ai_handles_full_board() {
        // Board is full, no moves left
        let mut ai = AIMinMax::new();
        ai.game_start(PlayerID::Player1);
        let grid = [
            Some(PlayerID::Player1),
            Some(PlayerID::Player2),
            Some(PlayerID::Player1),
            Some(PlayerID::Player1),
            Some(PlayerID::Player2),
            Some(PlayerID::Player1),
            Some(PlayerID::Player2),
            Some(PlayerID::Player1),
            Some(PlayerID::Player2),
        ];
        let res = ai.play(grid);
        assert!(res.is_err());
    }

    #[test]
    fn test_ai_chooses_draw_if_no_win_possible() {
        // AI is Player1, only move left leads to draw
        let mut ai = AIMinMax::new();
        ai.game_start(PlayerID::Player1);
        let grid = [
            Some(PlayerID::Player1),
            Some(PlayerID::Player2),
            Some(PlayerID::Player1),
            Some(PlayerID::Player1),
            Some(PlayerID::Player2),
            Some(PlayerID::Player2),
            Some(PlayerID::Player2),
            Some(PlayerID::Player1),
            None,
        ];
        let mv = ai.play(grid).unwrap();
        assert_eq!(mv, 8);
    }
}
