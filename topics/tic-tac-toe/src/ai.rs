//! AI module - Minimax algorithm implementation

use crate::board::{Board, Cell};

/// AI agent that uses minimax algorithm to determine optimal moves
pub struct AiAgent;

impl AiAgent {
    /// Creates a new AI agent
    pub fn new() -> Self {
        Self
    }

    /// Returns the best move for the AI player using minimax algorithm with alpha-beta pruning
    /// Returns None if no moves are available (game is over)
    pub fn get_best_move(&self, board: &Board) -> Option<(usize, usize)> {
        let empty_positions = board.empty_positions();

        if empty_positions.is_empty() {
            return None;
        }

        let mut best_score = i32::MIN;
        let mut best_moves = Vec::new();

        for (row, col) in empty_positions {
            let mut board_copy = board.clone();
            board_copy.set(row, col, Cell::O);

            let score = Self::minimax_alpha_beta(&board_copy, 0, false, i32::MIN, i32::MAX);

            if score > best_score {
                best_score = score;
                best_moves.clear();
                best_moves.push((row, col));
            } else if score == best_score {
                best_moves.push((row, col));
            }
        }

        // If multiple moves have the same score, prioritize strategically
        Self::select_strategic_move(&best_moves)
    }

    /// Select the most strategic move from equally scored positions
    /// Priority: center > corners > edges
    fn select_strategic_move(moves: &[(usize, usize)]) -> Option<(usize, usize)> {
        if moves.is_empty() {
            return None;
        }

        // Check for center position (1,1)
        if moves.contains(&(1, 1)) {
            return Some((1, 1));
        }

        // Check for corner positions
        let corners = [(0, 0), (0, 2), (2, 0), (2, 2)];
        for corner in corners {
            if moves.contains(&corner) {
                return Some(corner);
            }
        }

        // Return any remaining move (edges)
        Some(moves[0])
    }

    /// Minimax algorithm with alpha-beta pruning for improved performance
    fn minimax_alpha_beta(
        board: &Board,
        depth: usize,
        is_maximizing: bool,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        // Check for terminal states
        if let Some(winner) = board.check_winner() {
            return match winner {
                Cell::O => 100 - depth as i32, // AI wins (prefer shorter paths to victory)
                Cell::X => depth as i32 - 100, // Human wins (prefer longer paths to defeat)
                Cell::Empty => 0,              // Should never happen in practice
            };
        }

        // If board is full, it's a draw
        if board.is_full() {
            return 0;
        }

        if is_maximizing {
            // AI's turn - maximize score
            let mut max_score = i32::MIN;

            for (row, col) in board.empty_positions() {
                let mut board_copy = board.clone();
                board_copy.set(row, col, Cell::O);

                let score = Self::minimax_alpha_beta(&board_copy, depth + 1, false, alpha, beta);
                max_score = max_score.max(score);
                alpha = alpha.max(score);

                // Alpha-beta pruning
                if beta <= alpha {
                    break;
                }
            }

            max_score
        } else {
            // Human's turn - minimize score
            let mut min_score = i32::MAX;

            for (row, col) in board.empty_positions() {
                let mut board_copy = board.clone();
                board_copy.set(row, col, Cell::X);

                let score = Self::minimax_alpha_beta(&board_copy, depth + 1, true, alpha, beta);
                min_score = min_score.min(score);
                beta = beta.min(score);

                // Alpha-beta pruning
                if beta <= alpha {
                    break;
                }
            }

            min_score
        }
    }
}

impl Default for AiAgent {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_agent_creation() {
        let ai = AiAgent::new();
        let board = Board::new();
        assert!(ai.get_best_move(&board).is_some());
    }

    #[test]
    fn test_ai_blocks_winning_move() {
        let mut board = Board::new();
        board.set(0, 0, Cell::X);
        board.set(0, 1, Cell::X);

        let ai = AiAgent::new();
        let best_move = ai.get_best_move(&board);
        assert_eq!(best_move, Some((0, 2)));
    }

    #[test]
    fn test_ai_takes_winning_move() {
        let mut board = Board::new();
        board.set(1, 1, Cell::O);
        board.set(0, 0, Cell::O);
        board.set(2, 1, Cell::X);
        board.set(1, 0, Cell::X);

        let ai = AiAgent::new();
        let best_move = ai.get_best_move(&board);
        assert_eq!(best_move, Some((2, 2)));
    }

    #[test]
    fn test_ai_prefers_center_on_empty_board() {
        let board = Board::new();
        let ai = AiAgent::new();
        let best_move = ai.get_best_move(&board);
        assert_eq!(best_move, Some((1, 1)));
    }

    #[test]
    fn test_ai_no_moves_available() {
        let mut board = Board::new();
        // Fill the entire board
        board.set(0, 0, Cell::X);
        board.set(0, 1, Cell::O);
        board.set(0, 2, Cell::X);
        board.set(1, 0, Cell::O);
        board.set(1, 1, Cell::X);
        board.set(1, 2, Cell::O);
        board.set(2, 0, Cell::X);
        board.set(2, 1, Cell::O);
        board.set(2, 2, Cell::X);

        let ai = AiAgent::new();
        assert_eq!(ai.get_best_move(&board), None);
    }

    #[test]
    fn test_strategic_move_selection() {
        // Test center preference
        let moves = vec![(0, 1), (1, 1), (2, 1)];
        assert_eq!(AiAgent::select_strategic_move(&moves), Some((1, 1)));

        // Test corner preference when no center
        let moves = vec![(0, 1), (0, 0), (2, 1)];
        assert_eq!(AiAgent::select_strategic_move(&moves), Some((0, 0)));

        // Test edge selection when no center or corners
        let moves = vec![(0, 1), (1, 0), (2, 1)];
        assert_eq!(AiAgent::select_strategic_move(&moves), Some((0, 1)));
    }

    #[test]
    fn test_ai_fork_blocking() {
        let mut board = Board::new();
        // Set up a fork scenario where human has two ways to win
        // X in corners creates a fork
        board.set(0, 0, Cell::X); // Top-left corner
        board.set(2, 2, Cell::X); // Bottom-right corner
        board.set(1, 1, Cell::O); // AI has center

        let ai = AiAgent::new();
        let best_move = ai.get_best_move(&board);

        // AI should block one of the winning paths
        // Valid blocking moves: (0,2), (2,0), (0,1), (1,0), (1,2), (2,1)
        let blocking_moves = vec![(0, 2), (2, 0), (0, 1), (1, 0), (1, 2), (2, 1)];
        assert!(blocking_moves.contains(&best_move.unwrap()));
    }

    #[test]
    fn test_ai_priorities_winning_over_blocking() {
        let mut board = Board::new();
        // Set up scenario where AI can win OR block human win
        board.set(0, 0, Cell::O); // AI
        board.set(0, 1, Cell::O); // AI (can win at 0,2)
        board.set(1, 0, Cell::X); // Human
        board.set(1, 1, Cell::X); // Human (can win at 1,2)

        let ai = AiAgent::new();
        let best_move = ai.get_best_move(&board);

        // AI should prioritize winning over blocking
        assert_eq!(best_move, Some((0, 2)));
    }

    #[test]
    fn test_ai_corner_response() {
        let mut board = Board::new();
        // If human takes a corner, AI should take center
        board.set(0, 0, Cell::X);

        let ai = AiAgent::new();
        let best_move = ai.get_best_move(&board);
        assert_eq!(best_move, Some((1, 1)));

        // If center is taken, AI should take opposite corner
        let mut board = Board::new();
        board.set(0, 0, Cell::X); // Human takes corner
        board.set(1, 1, Cell::X); // Human takes center

        let ai = AiAgent::new();
        let best_move = ai.get_best_move(&board);
        // Should take opposite corner (2,2) or another strategic position
        let strategic_moves = vec![(2, 2), (0, 2), (2, 0)];
        assert!(strategic_moves.contains(&best_move.unwrap()));
    }
}
