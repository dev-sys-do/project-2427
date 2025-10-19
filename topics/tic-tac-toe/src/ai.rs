use crate::board::Board;
use crate::game::Game;
use crate::types::Player;

/// AI player using the Minimax algorithm
pub struct AI {
    player: Player,
}

impl AI {
    /// Creates a new AI instance
    pub fn new() -> Self {
        AI { player: Player::AI }
    }

    /// Finds the best move for the AI using the Minimax algorithm
    /// Returns the position (0-8) of the best move
    pub fn find_best_move(&self, game: &Game) -> Option<usize> {
        let available_moves = game.available_moves();

        if available_moves.is_empty() {
            return None;
        }

        let mut best_score = i32::MIN;
        let mut best_move = available_moves[0];

        // Try each available move and evaluate it
        for &position in &available_moves {
            let mut game_clone = self.simulate_move(game, position, self.player);
            let score = self.minimax(&mut game_clone, 0, false);

            if score > best_score {
                best_score = score;
                best_move = position;
            }
        }

        Some(best_move)
    }

    /// Minimax algorithm with depth tracking
    ///
    /// # Arguments
    /// * `game` - The current game state
    /// * `depth` - Current depth in the game tree
    /// * `is_maximizing` - True if maximizing player (AI), false if minimizing (Human)
    ///
    /// # Returns
    /// The score of the board state
    fn minimax(&self, game: &mut Game, depth: i32, is_maximizing: bool) -> i32 {
        // Terminal state: check if game is over
        let score = game.evaluate();

        // If AI won, return score minus depth (prefer faster wins)
        if score == 10 {
            return score - depth;
        }

        // If Human won, return score plus depth (prefer slower losses)
        if score == -10 {
            return score + depth;
        }

        // Check for draw
        let available_moves = game.available_moves();
        if available_moves.is_empty() {
            return 0;
        }

        if is_maximizing {
            // Maximizing player (AI)
            let mut best_score = i32::MIN;

            for &position in &available_moves {
                let mut game_clone = self.simulate_move(game, position, Player::AI);
                let score = self.minimax(&mut game_clone, depth + 1, false);
                best_score = best_score.max(score);
            }

            best_score
        } else {
            // Minimizing player (Human)
            let mut best_score = i32::MAX;

            for &position in &available_moves {
                let mut game_clone = self.simulate_move(game, position, Player::Human);
                let score = self.minimax(&mut game_clone, depth + 1, true);
                best_score = best_score.min(score);
            }

            best_score
        }
    }

    /// Simulates a move and returns a new game state
    fn simulate_move(&self, game: &Game, position: usize, player: Player) -> Game {
        // Create a copy of the current game using the board state
        let mut new_board = Board::new();

        // Copy the current board state
        for i in 0..9 {
            if let Some(crate::types::Cell::Occupied(p)) = game.board().get(i) {
                new_board.make_move(i, p);
            }
        }

        // Make the new move on the copied board
        new_board.make_move(position, player);

        // Create a new game with this board state
        // We need to use Game::from_board or similar
        // For now, let's create a helper in Game
        self.create_game_from_board(new_board, player.opponent())
    }

    /// Creates a game state from a board
    fn create_game_from_board(&self, board: Board, next_player: Player) -> Game {
        Game::from_board(board, next_player)
    }
}

impl Default for AI {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_blocks_winning_move() {
        let mut game = Game::new();
        let ai = AI::new();

        // Human has two in a row
        game.make_move(0); // Human X at position 0
        game.make_move(3); // AI O at position 3
        game.make_move(1); // Human X at position 1

        // AI should block position 2 to prevent human win
        let best_move = ai.find_best_move(&game);
        assert_eq!(best_move, Some(2));
    }

    #[test]
    fn test_ai_takes_winning_move() {
        let mut game = Game::new();
        let ai = AI::new();

        // Setup: AI has two in a row
        game.make_move(0); // Human X
        game.make_move(3); // AI O
        game.make_move(1); // Human X
        game.make_move(4); // AI O
        game.make_move(8); // Human X

        // AI should take position 5 to win
        let best_move = ai.find_best_move(&game);
        assert_eq!(best_move, Some(5));
    }

    #[test]
    fn test_ai_finds_move_on_empty_board() {
        let game = Game::new();
        let ai = AI::new();

        // AI should find a valid move
        let best_move = ai.find_best_move(&game);
        assert!(best_move.is_some());
        assert!(best_move.unwrap() < 9);
    }
}
