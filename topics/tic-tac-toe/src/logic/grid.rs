use crate::types::{Grid, PlayerID};

pub fn is_there_a_win(grid: Grid) -> Option<PlayerID> {
    // Check rows
    for row in 0..3 {
        if grid[row * 3].is_some()
            && grid[row * 3] == grid[row * 3 + 1]
            && grid[row * 3 + 1] == grid[row * 3 + 2]
        {
            return grid[row * 3];
        }
    }

    // Check columns
    for col in 0..3 {
        if grid[col].is_some() && grid[col] == grid[col + 3] && grid[col + 3] == grid[col + 6] {
            return grid[col];
        }
    }

    // Check diagonals
    if grid[0].is_some() && grid[0] == grid[4] && grid[4] == grid[8] {
        return grid[0];
    }

    if grid[2].is_some() && grid[2] == grid[4] && grid[4] == grid[6] {
        return grid[2];
    }

    None // No winner
}

/// Check if there are any moves left on the board
pub fn are_there_moves_left(grid: Grid) -> bool {
    grid.iter().any(|cell| cell.is_none())
}
