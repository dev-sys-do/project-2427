use crate::game::{Board, Player, GameState};
use super::strategy::Strategy;

pub struct MiniMax {
    debug: bool,
    nodes_evaluated: usize,
}

impl MiniMax {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
            nodes_evaluated: 0,
        }
    }
    
    pub fn get_best_move(&mut self, board: &Board) -> usize {
        self.nodes_evaluated = 0;
        
        if board.is_empty() {
            if self.debug {
                println!("🎯 Empty board: choosing center position");
            }
            return 4;
        }
        
        let available_moves = board.available_moves();
        if available_moves.is_empty() {
            panic!("No available moves on board");
        }
        
        if available_moves.len() == 1 {
            if self.debug {
                println!("🎯 Only one move available: {}", available_moves[0] + 1);
            }
            return available_moves[0];
        }
        
        let mut best_move = available_moves[0];
        let mut best_score = i32::MIN;
        
        let ordered_moves = Strategy::order_moves(available_moves);
        
        if self.debug {
            println!("🤔 Evaluating {} possible moves...", ordered_moves.len());
        }
        
        for &move_pos in &ordered_moves {
            let mut board_copy = board.clone_for_simulation();
            board_copy.make_move(move_pos, Player::AI).unwrap();
            
            let score = self.minimax(
                &board_copy,
                0,
                false,
                i32::MIN,
                i32::MAX
            );
            
            if self.debug {
                println!("  Position {}: score {}", move_pos + 1, score);
            }
            
            if score > best_score {
                best_score = score;
                best_move = move_pos;
            }
        }
        
        if self.debug {
            println!("🎯 Best move: {} (score: {}, nodes evaluated: {})", 
                     best_move + 1, best_score, self.nodes_evaluated);
        }
        
        best_move
    }
    
    fn minimax(
        &mut self,
        board: &Board,
        depth: usize,
        is_maximizing: bool,
        mut alpha: i32,
        mut beta: i32,
    ) -> i32 {
        self.nodes_evaluated += 1;
        
        let state = board.game_state();
        
        match state {
            GameState::Win(_) | GameState::Draw => {
                let base_score = Strategy::evaluate_terminal_state(&state);
                return if base_score > 0 {
                    base_score + (10 - depth as i32)
                } else if base_score < 0 {
                    base_score - (10 - depth as i32)
                } else {
                    base_score
                };
            }
            GameState::InProgress => {}
        }
        
        if depth > 9 {
            return Strategy::heuristic_evaluation(board);
        }
        
        let available_moves = board.available_moves();
        let ordered_moves = Strategy::order_moves(available_moves);
        
        if is_maximizing {
            let mut max_score = i32::MIN;
            
            for &move_pos in &ordered_moves {
                let mut board_copy = board.clone_for_simulation();
                board_copy.make_move(move_pos, Player::AI).unwrap();
                
                let score = self.minimax(&board_copy, depth + 1, false, alpha, beta);
                max_score = max_score.max(score);
                alpha = alpha.max(score);
                
                if beta <= alpha {
                    break;
                }
            }
            
            max_score
        } else {
            let mut min_score = i32::MAX;
            
            for &move_pos in &ordered_moves {
                let mut board_copy = board.clone_for_simulation();
                board_copy.make_move(move_pos, Player::Human).unwrap();
                
                let score = self.minimax(&board_copy, depth + 1, true, alpha, beta);
                min_score = min_score.min(score);
                beta = beta.min(score);
                
                if beta <= alpha {
                    break;
                }
            }
            
            min_score
        }
    }
}
