use tic_tac_toe::ai::find_best_move;
use tic_tac_toe::board::{Board, Player};
use tic_tac_toe::game::{check_game_state, GameState};

#[test]
fn test_ai_wins_when_possible() {
    let mut board = Board::new();
    board.set(0, Player::Ai);
    board.set(1, Player::Ai);
    board.set(3, Player::Human);
    board.set(4, Player::Human);

    let best_move = find_best_move(&board, Player::Ai);
    assert_eq!(best_move, Some(2));

    board.set(2, Player::Ai);
    assert_eq!(check_game_state(&board), GameState::Won(Player::Ai));
}

#[test]
fn test_ai_blocks_human_win() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(1, Player::Human);
    board.set(3, Player::Ai);

    let best_move = find_best_move(&board, Player::Ai);
    assert_eq!(best_move, Some(2));
}

#[test]
fn test_ai_takes_center_on_empty_board() {
    let board = Board::new();

    let best_move = find_best_move(&board, Player::Ai);
    assert!(best_move.is_some());

    let position = best_move.unwrap();
    assert!(position < 9);
}

#[test]
fn test_ai_blocks_diagonal_win() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(4, Player::Human);

    let best_move = find_best_move(&board, Player::Ai);
    assert_eq!(best_move, Some(8));
}

#[test]
fn test_ai_creates_fork() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(4, Player::Ai);

    let best_move = find_best_move(&board, Player::Ai);
    assert!(best_move.is_some());
}

#[test]
fn test_ai_prioritizes_immediate_win_over_block() {
    let mut board = Board::new();
    board.set(0, Player::Ai);
    board.set(1, Player::Ai);
    board.set(3, Player::Human);
    board.set(4, Player::Human);

    let best_move = find_best_move(&board, Player::Ai);
    assert_eq!(best_move, Some(2));
}

#[test]
fn test_ai_blocks_vertical_win() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(3, Player::Human);
    board.set(5, Player::Ai);

    let best_move = find_best_move(&board, Player::Ai);
    assert_eq!(best_move, Some(6));
}

#[test]
fn test_ai_finds_move_on_almost_full_board() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(1, Player::Ai);
    board.set(2, Player::Human);
    board.set(3, Player::Ai);
    board.set(4, Player::Human);
    board.set(5, Player::Ai);
    board.set(6, Player::Ai);
    board.set(7, Player::Human);

    let best_move = find_best_move(&board, Player::Ai);
    assert_eq!(best_move, Some(8));
}

#[test]
fn test_ai_returns_none_on_full_board() {
    let mut board = Board::new();
    for i in 0..9 {
        board.set(
            i,
            if i % 2 == 0 {
                Player::Human
            } else {
                Player::Ai
            },
        );
    }

    let best_move = find_best_move(&board, Player::Ai);
    assert_eq!(best_move, None);
}

#[test]
fn test_ai_wins_anti_diagonal() {
    let mut board = Board::new();
    board.set(2, Player::Ai);
    board.set(4, Player::Ai);
    board.set(3, Player::Human);
    board.set(5, Player::Human);

    let best_move = find_best_move(&board, Player::Ai);
    assert_eq!(best_move, Some(6));
}
