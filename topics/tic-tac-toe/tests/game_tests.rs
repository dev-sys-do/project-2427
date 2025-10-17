use tic_tac_toe::board::{Board, Player};
use tic_tac_toe::game::{check_game_state, GameState};

#[test]
fn test_initial_state() {
    let board = Board::new();
    assert_eq!(check_game_state(&board), GameState::InProgress);
}

#[test]
fn test_horizontal_win_top() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(1, Player::Human);
    board.set(2, Player::Human);

    assert_eq!(check_game_state(&board), GameState::Won(Player::Human));
}

#[test]
fn test_horizontal_win_middle() {
    let mut board = Board::new();
    board.set(3, Player::Ai);
    board.set(4, Player::Ai);
    board.set(5, Player::Ai);

    assert_eq!(check_game_state(&board), GameState::Won(Player::Ai));
}

#[test]
fn test_horizontal_win_bottom() {
    let mut board = Board::new();
    board.set(6, Player::Human);
    board.set(7, Player::Human);
    board.set(8, Player::Human);

    assert_eq!(check_game_state(&board), GameState::Won(Player::Human));
}

#[test]
fn test_vertical_win_left() {
    let mut board = Board::new();
    board.set(0, Player::Ai);
    board.set(3, Player::Ai);
    board.set(6, Player::Ai);

    assert_eq!(check_game_state(&board), GameState::Won(Player::Ai));
}

#[test]
fn test_vertical_win_middle() {
    let mut board = Board::new();
    board.set(1, Player::Human);
    board.set(4, Player::Human);
    board.set(7, Player::Human);

    assert_eq!(check_game_state(&board), GameState::Won(Player::Human));
}

#[test]
fn test_vertical_win_right() {
    let mut board = Board::new();
    board.set(2, Player::Ai);
    board.set(5, Player::Ai);
    board.set(8, Player::Ai);

    assert_eq!(check_game_state(&board), GameState::Won(Player::Ai));
}

#[test]
fn test_diagonal_win_main() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(4, Player::Human);
    board.set(8, Player::Human);

    assert_eq!(check_game_state(&board), GameState::Won(Player::Human));
}

#[test]
fn test_diagonal_win_anti() {
    let mut board = Board::new();
    board.set(2, Player::Ai);
    board.set(4, Player::Ai);
    board.set(6, Player::Ai);

    assert_eq!(check_game_state(&board), GameState::Won(Player::Ai));
}

#[test]
fn test_draw() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(1, Player::Ai);
    board.set(2, Player::Human);
    board.set(3, Player::Human);
    board.set(4, Player::Ai);
    board.set(5, Player::Ai);
    board.set(6, Player::Ai);
    board.set(7, Player::Human);
    board.set(8, Player::Human);

    assert_eq!(check_game_state(&board), GameState::Draw);
}

#[test]
fn test_in_progress() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(1, Player::Ai);

    assert_eq!(check_game_state(&board), GameState::InProgress);
}

#[test]
fn test_in_progress_almost_full() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(1, Player::Ai);
    board.set(2, Player::Human);
    board.set(3, Player::Ai);
    board.set(4, Player::Human);
    board.set(5, Player::Ai);
    board.set(6, Player::Ai);
    board.set(7, Player::Human);

    assert_eq!(check_game_state(&board), GameState::InProgress);
}
