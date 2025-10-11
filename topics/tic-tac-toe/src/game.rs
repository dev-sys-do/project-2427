use crate::board::{lines::WINNING_LINES, Board, Cell, Player};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Won(Player),
    Draw,
}

pub fn check_game_state(board: &Board) -> GameState {
    for line in &WINNING_LINES {
        if let Some(winner) = check_line(board, line) {
            return GameState::Won(winner);
        }
    }

    if board.is_full() {
        return GameState::Draw;
    }

    GameState::InProgress
}

/// Checks if a line (row, column, or diagonal) has three matching occupied cells
/// Returns the player who won, or None if no winner on this line
fn check_line(board: &Board, line: &[usize; 3]) -> Option<Player> {
    let cells = [
        board.get(line[0])?,
        board.get(line[1])?,
        board.get(line[2])?,
    ];

    match cells {
        [Cell::Occupied(p1), Cell::Occupied(p2), Cell::Occupied(p3)] if p1 == p2 && p2 == p3 => {
            Some(p1)
        }
        _ => None,
    }
}
