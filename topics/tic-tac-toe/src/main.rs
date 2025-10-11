/// Tic-Tac-Toe game with AI opponent using Minimax algorithm
///
/// This program allows a human player to play against an AI opponent
/// that uses the Minimax algorithm to play optimally.
use tic_tac_toe::board::{Board, Player};
use tic_tac_toe::game::{check_game_state, GameState};
use tic_tac_toe::ui::{
    ask_play_again, clear_screen, display_ai_thinking, display_board, display_game_status,
    display_position_guide, display_title, get_player_move,
};

fn main() {
    loop {
        clear_screen();
        display_title();

        if !run_game() {
            break;
        }
    }

    println!("  Thanks for playing!");
    println!();
}

/// Runs a single game session
/// Returns true if the player wants to play again, false otherwise
fn run_game() -> bool {
    let mut board = Board::new();
    let mut first_turn = true;

    loop {
        clear_screen();
        display_title();

        if first_turn {
            display_position_guide();
            first_turn = false;
        }

        display_board(&board);

        let state = check_game_state(&board);
        if state != GameState::InProgress {
            display_game_status(state);
            return ask_play_again();
        }

        let position = get_player_move(&board);
        board.set(position, Player::Human);

        let state = check_game_state(&board);
        if state != GameState::InProgress {
            clear_screen();
            display_title();
            display_board(&board);
            display_game_status(state);
            return ask_play_again();
        }

        // AI player's turn (placeholder - random move for now)
        clear_screen();
        display_title();
        display_board(&board);
        display_ai_thinking();
        let ai_position = make_ai_move(&board);
        board.set(ai_position, Player::Ai);
    }
}

/// Makes an AI move (placeholder implementation - will be replaced with Minimax)
/// Currently makes a random valid move
fn make_ai_move(board: &Board) -> usize {
    // Find all empty positions
    let empty_positions: Vec<usize> = board.empty_positions().collect();

    // For now, just pick the first empty position
    // This will be replaced with Minimax algorithm
    empty_positions[0]
}
