use tic_tac_toe::board::{Board, Cell, Player};

#[test]
fn test_new_board_is_empty() {
    let board = Board::new();
    for i in 0..9 {
        assert_eq!(board.get(i), Some(Cell::Empty));
    }
}

#[test]
fn test_set_and_get() {
    let mut board = Board::new();
    assert!(board.set(0, Player::Human));
    assert_eq!(board.get(0), Some(Cell::Occupied(Player::Human)));
}

#[test]
fn test_cannot_overwrite_cell() {
    let mut board = Board::new();
    assert!(board.set(0, Player::Human));
    assert!(!board.set(0, Player::Ai));
    assert_eq!(board.get(0), Some(Cell::Occupied(Player::Human)));
}

#[test]
fn test_empty_positions() {
    let mut board = Board::new();
    board.set(0, Player::Human);
    board.set(4, Player::Ai);

    let empty: Vec<usize> = board.empty_positions().collect();
    assert_eq!(empty, vec![1, 2, 3, 5, 6, 7, 8]);
}

#[test]
fn test_is_full() {
    let mut board = Board::new();
    assert!(!board.is_full());

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
    assert!(board.is_full());
}

#[test]
fn test_player_opponent() {
    assert_eq!(Player::Human.opponent(), Player::Ai);
    assert_eq!(Player::Ai.opponent(), Player::Human);
}

#[test]
fn test_player_symbol() {
    assert_eq!(Player::Human.symbol(), 'X');
    assert_eq!(Player::Ai.symbol(), 'O');
}

#[test]
fn test_cell_is_empty() {
    assert!(Cell::Empty.is_empty());
    assert!(!Cell::Occupied(Player::Human).is_empty());
    assert!(!Cell::Occupied(Player::Ai).is_empty());
}

#[test]
fn test_board_get_out_of_bounds() {
    let board = Board::new();
    assert_eq!(board.get(9), None);
    assert_eq!(board.get(100), None);
}

#[test]
fn test_board_set_out_of_bounds() {
    let mut board = Board::new();
    assert!(!board.set(9, Player::Human));
    assert!(!board.set(100, Player::Human));
}
