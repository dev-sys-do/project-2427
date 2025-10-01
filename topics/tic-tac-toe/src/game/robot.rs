use crate::game::board::Board;

pub struct Robot {
    symbol: char,
    opponent_symbol: char,
}

impl Robot {
    pub fn new(symbol: char) -> Self {
        let opponent_symbol = if symbol == 'X' { 'O' } else { 'X' };
        Robot { symbol, opponent_symbol }
    }

    pub fn make_move(&self, board: &mut Board) -> Option<usize> {
        let best_position = self.find_best_move(board);
        
        if let Some(position) = best_position {
            if board.place_symbol_by_position(position, self.symbol) {
                Some(position)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn find_best_move(&self, board: &Board) -> Option<usize> {
        let mut best_score = i32::MIN;
        let mut best_position = None;

        for position in 1..=9 {
            let (row, col) = self.position_to_coords(position);
            if board.is_position_empty(row, col) {
                // Create a copy of the board to test the move
                let mut test_board = board.clone();
                test_board.place_symbol_by_position(position, self.symbol);
                
                // Calculate the score using minimax
                let score = self.minimax(&test_board, 0, false);
                
                if score > best_score {
                    best_score = score;
                    best_position = Some(position);
                }
            }
        }

        best_position
    }

    fn minimax(&self, board: &Board, depth: i32, is_maximizing: bool) -> i32 {
        // Check terminal states
        if let Some(winner) = board.check_winner() {
            if winner == self.symbol {
                return 10 - depth; // Robot wins (prefer shorter paths to victory)
            } else {
                return depth - 10; // Opponent wins (prefer longer paths to defeat)
            }
        }

        if board.is_full() {
            return 0; // Draw
        }

        if is_maximizing {
            // Robot's turn - maximize score
            let mut max_score = i32::MIN;
            
            for position in 1..=9 {
                let (row, col) = self.position_to_coords(position);
                if board.is_position_empty(row, col) {
                    let mut test_board = board.clone();
                    test_board.place_symbol_by_position(position, self.symbol);
                    
                    let score = self.minimax(&test_board, depth + 1, false);
                    max_score = max_score.max(score);
                }
            }
            
            max_score
        } else {
            // Opponent's turn - minimize score
            let mut min_score = i32::MAX;
            
            for position in 1..=9 {
                let (row, col) = self.position_to_coords(position);
                if board.is_position_empty(row, col) {
                    let mut test_board = board.clone();
                    test_board.place_symbol_by_position(position, self.opponent_symbol);
                    
                    let score = self.minimax(&test_board, depth + 1, true);
                    min_score = min_score.min(score);
                }
            }
            
            min_score
        }
    }

    fn position_to_coords(&self, position: usize) -> (usize, usize) {
        let index = position - 1; // Convert to 0-based index
        let row = index / 3;
        let col = index % 3;
        (row, col)
    }
}