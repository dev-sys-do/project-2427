use crate::board::{Board, Player};
use crate::game::{check_game_state, GameState};

/// Finds the best move for the AI using the Minimax algorithm
///
/// The Minimax algorithm explores all possible game states to find the optimal move.
/// It assumes the opponent plays perfectly and chooses moves that minimize the
/// maximum possible loss.
///
/// # Arguments
///
/// * `board` - The current game board
/// * `ai_player` - The player for which to find the best move
///
/// # Returns
///
/// * `Some(position)` - The best position to play (0-8)
/// * `None` - If the board is full
pub fn find_best_move(board: &Board, ai_player: Player) -> Option<usize> {
    let mut best_score = i32::MIN;
    let mut best_move = None;

    for position in board.empty_positions() {
        let mut board_copy = *board;
        board_copy.set(position, ai_player);

        let score = minimax(&board_copy, 0, false, ai_player);

        if score > best_score {
            best_score = score;
            best_move = Some(position);
        }
    }

    best_move
}

/// Minimax algorithm with depth tracking
///
/// Recursively evaluates all possible game states to find the optimal move.
///
/// # Scoring
///
/// * +10 to -10: AI win (faster wins get higher scores)
/// * 0: Draw
/// * -10 to +10: Human win (slower losses get higher scores)
///
/// # Arguments
///
/// * `board` - Current board state
/// * `depth` - Current search depth
/// * `is_maximizing` - True if maximizing (AI's turn), false if minimizing (human's turn)
/// * `ai_player` - The AI player
///
/// # Returns
///
/// The score of the best move from this position
fn minimax(board: &Board, depth: i32, is_maximizing: bool, ai_player: Player) -> i32 {
    let state = check_game_state(board);

    match state {
        GameState::Won(player) => {
            if player == ai_player {
                return 10 - depth; // Prefer faster wins
            } else {
                return depth - 10; // Prefer slower losses
            }
        }
        GameState::Draw => return 0,
        GameState::InProgress => {}
    }

    if is_maximizing {
        // AI's turn - maximize score
        let mut best_score = i32::MIN;

        for position in board.empty_positions() {
            let mut board_copy = *board;
            board_copy.set(position, ai_player);

            let score = minimax(&board_copy, depth + 1, false, ai_player);
            best_score = best_score.max(score);
        }

        best_score
    } else {
        // Human's turn - minimize score
        let mut best_score = i32::MAX;
        let human_player = ai_player.opponent();

        for position in board.empty_positions() {
            let mut board_copy = *board;
            board_copy.set(position, human_player);

            let score = minimax(&board_copy, depth + 1, true, ai_player);
            best_score = best_score.min(score);
        }

        best_score
    }
}
