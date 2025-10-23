use crate::board::{Board, Player, Cell};
use crate::game::Game;

/// AI player that uses the Minimax algorithm to play optimally
pub struct AI {
    player: Player,
}

impl AI {
    /// Creates a new AI player
    pub fn new(player: Player) -> Self {
        Self { player }
    }

    /// Gets the best move for the AI using the Minimax algorithm
    pub fn get_best_move(&self, game: &Game) -> Option<usize> {
        let available_moves = game.get_available_moves();
        if available_moves.is_empty() {
            return None;
        }

        let mut best_score = i32::MIN;
        let mut best_move = None;

        for &position in &available_moves {
            let mut game_copy = self.clone_game(game);
            
            game_copy.make_move(position);
            
            let score = self.minimax(&game_copy, 0, false);
            
            if score > best_score {
                best_score = score;
                best_move = Some(position);
            }
        }

        best_move
    }

    /// Minimax algorithm implementation with depth-first search
    /// Returns the score for the current game state from the AI's perspective
    /// is_maximizing: true when it's AI's turn (maximize), false when opponent's turn (minimize)
    fn minimax(&self, game: &Game, depth: u32, is_maximizing: bool) -> i32 {
        // Check terminal states (game over)
        if let Some(winner) = game.check_winner() {
            return if winner == self.player {
                10 - depth as i32  // AI wins: prefer winning sooner (higher score for fewer moves)
            } else {
                depth as i32 - 10  // AI loses: prefer losing later (less negative score)
            };
        }

        if game.is_game_over() {
            return 0;
        }

        let available_moves = game.get_available_moves();
        
        if is_maximizing {
            // AI's turn - maximize the score
            let mut max_score = i32::MIN;
            
            for &position in &available_moves {
                let mut game_copy = self.clone_game(game);
                game_copy.make_move(position);
                
                let score = self.minimax(&game_copy, depth + 1, false);
                max_score = max_score.max(score);
            }
            
            max_score
        } else {
            // Opponent's turn - minimize the score (from AI's perspective)
            let mut min_score = i32::MAX;
            
            for &position in &available_moves {
                let mut game_copy = self.clone_game(game);
                game_copy.make_move(position);
                
                let score = self.minimax(&game_copy, depth + 1, true);
                min_score = min_score.min(score);
            }
            
            min_score
        }
    }

    /// Creates a copy of the game state for simulation
    /// This is necessary since Game doesn't implement Clone
    fn clone_game(&self, game: &Game) -> Game {
        let mut new_game = Game::new();
        
        let board = game.get_board();
        
        let mut moves = Vec::new();
        for position in 1..=9 {
            if let Some(Cell::Occupied(player)) = board.get_cell(position) {
                moves.push((position, player));
            }
        }
        

        let mut x_moves = Vec::new();
        let mut o_moves = Vec::new();
        
        for (pos, player) in moves {
            match player {
                Player::X => x_moves.push(pos),
                Player::O => o_moves.push(pos),
            }
        }
        
        let mut move_sequence = Vec::new();
        let max_len = x_moves.len().max(o_moves.len());
        
        for i in 0..max_len {
            if i < x_moves.len() {
                move_sequence.push(x_moves[i]);
            }
            if i < o_moves.len() {
                move_sequence.push(o_moves[i]);
            }
        }
        
        for &position in &move_sequence {
            new_game.make_move(position);
        }
        
        new_game
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::Game;

    #[test]
    fn test_ai_creation() {
        let ai = AI::new(Player::O);
        assert_eq!(ai.player, Player::O);
        
        let ai_x = AI::new(Player::X);
        assert_eq!(ai_x.player, Player::X);
    }

    #[test]
    fn test_ai_blocks_winning_move() {
        let mut game = Game::new();
        let ai = AI::new(Player::O);
        
        // X threatens to win in top row
        game.make_move(1); // X
        game.make_move(4); // O (AI's previous move)
        game.make_move(2); // X
        // Now X threatens to win at position 3
        
        let ai_move = ai.get_best_move(&game);
        assert_eq!(ai_move, Some(3)); // AI should block at position 3
    }

    #[test]
    fn test_ai_takes_winning_move() {
        let mut game = Game::new();
        let ai = AI::new(Player::O);
        
        // Set up a scenario where AI can win
        game.make_move(1); // X
        game.make_move(4); // O
        game.make_move(2); // X
        game.make_move(5); // O
        game.make_move(7); // X
        // Now O can win at position 6
        
        let ai_move = ai.get_best_move(&game);
        assert_eq!(ai_move, Some(6)); // AI should win at position 6
    }

    #[test]
    fn test_ai_chooses_valid_move_on_empty_board() {
        let game = Game::new();
        let ai = AI::new(Player::X);
        
        let ai_move = ai.get_best_move(&game);
        // On an empty board, any move should be valid (1-9)
        // The AI should choose some valid position
        assert!(ai_move.is_some());
        let position = ai_move.unwrap();
        assert!(position >= 1 && position <= 9);
    }

    #[test]
    fn test_ai_no_moves_available() {
        let mut game = Game::new();
        let ai = AI::new(Player::O);
        
        // Fill the board completely
        for position in 1..=9 {
            let player = if position % 2 == 1 { Player::X } else { Player::O };
            game.make_move(position);
        }
        
        let ai_move = ai.get_best_move(&game);
        assert_eq!(ai_move, None); // No moves available
    }

    #[test]
    fn test_minimax_prefers_winning_sooner() {
        let mut game = Game::new();
        let ai = AI::new(Player::O);
        
        // Create a scenario where AI has multiple ways to win
        // AI should prefer the immediate win over a longer path to victory
        game.make_move(1); // X
        game.make_move(4); // O
        game.make_move(2); // X
        game.make_move(5); // O
        game.make_move(8); // X
        // AI can win immediately at position 6
        
        let ai_move = ai.get_best_move(&game);
        assert_eq!(ai_move, Some(6)); // Should take the immediate win
    }
}