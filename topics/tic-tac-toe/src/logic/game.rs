use crate::{
    logic::grid,
    player::PlayerBehavior,
    types::{Grid, PlayerID},
};

pub struct Game<T1: PlayerBehavior, T2: PlayerBehavior> {
    pub grid: Grid,
    pub player1: T1,
    pub player2: T2,
}

impl<T1: PlayerBehavior, T2: PlayerBehavior> Game<T1, T2> {
    pub fn new(player1: T1, player2: T2) -> Self {
        Game {
            grid: [None; 9],
            player1,
            player2,
        }
    }

    pub fn play(mut self) -> crate::Result<Option<PlayerID>> {
        self.player1.game_start(crate::types::PlayerID::Player1);
        self.player2.game_start(crate::types::PlayerID::Player2);

        let mut current_player: &mut dyn PlayerBehavior = &mut self.player1;
        let mut current_player_id = crate::types::PlayerID::Player1;

        loop {
            // Make current player play
            match current_player.play(self.grid) {
                Ok(position) => {
                    // Validate move
                    if self.grid[position as usize].is_none() {
                        self.grid[position as usize] = Some(current_player_id);

                        // Win
                        if let Some(winner) = grid::is_there_a_win(self.grid) {
                            return Ok(Some(winner));
                        }

                        // Tie
                        if !crate::logic::grid::are_there_moves_left(self.grid) {
                            return Ok(None);
                        }

                        // Switch players
                        if current_player_id == crate::types::PlayerID::Player1 {
                            current_player = &mut self.player2;
                            current_player_id = crate::types::PlayerID::Player2;
                        } else {
                            current_player = &mut self.player1;
                            current_player_id = crate::types::PlayerID::Player1;
                        }
                    } else {
                        // If move is invalid, ask the same player to play again
                        continue;
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
}
