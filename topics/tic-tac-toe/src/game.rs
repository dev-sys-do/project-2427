const SIZE: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

pub struct TicTacToe {
    board: [[Cell; SIZE]; SIZE],
}

impl TicTacToe {
    pub fn new() -> Self {
        TicTacToe {
            board: [[Cell::Empty; SIZE]; SIZE],
        }
    }

    pub fn display(&self) {
        println!();
        for (i, row) in self.board.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                let symbol = match cell {
                    Cell::Empty => ".",
                    Cell::X => "X",
                    Cell::O => "O",
                };
                print!("{}", symbol);
                if j < SIZE - 1 {
                    print!(" | ");
                }
            }
            println!();
            if i < SIZE - 1 {
                println!("---------");
            }
        }
        println!();
    }

    pub fn place(&mut self, row: usize, col: usize, player: Cell) -> bool {
        if row < SIZE && col < SIZE && self.board[row][col] == Cell::Empty {
            self.board[row][col] = player;
            true
        } else {
            false
        }
    }

    pub fn is_full(&self) -> bool {
        self.board.iter().all(|row| row.iter().all(|&cell| cell != Cell::Empty))
    }

    pub fn check_winner(&self) -> Option<Cell> {
        // Vérifier les lignes et colonnes
        for i in 0..SIZE {
            if self.board[i][0] != Cell::Empty
                && self.board[i][0] == self.board[i][1]
                && self.board[i][1] == self.board[i][2]
            {
                return Some(self.board[i][0]);
            }
            if self.board[0][i] != Cell::Empty
                && self.board[0][i] == self.board[1][i]
                && self.board[1][i] == self.board[2][i]
            {
                return Some(self.board[0][i]);
            }
        }

        // Vérifier les diagonales
        if self.board[0][0] != Cell::Empty
            && self.board[0][0] == self.board[1][1]
            && self.board[1][1] == self.board[2][2]
        {
            return Some(self.board[0][0]);
        }
        if self.board[0][2] != Cell::Empty
            && self.board[0][2] == self.board[1][1]
            && self.board[1][1] == self.board[2][0]
        {
            return Some(self.board[0][2]);
        }

        None
    }

    pub fn get_empty_cells(&self) -> Vec<(usize, usize)> {
        let mut empty_cells = Vec::new();
        for i in 0..SIZE {
            for j in 0..SIZE {
                if self.board[i][j] == Cell::Empty {
                    empty_cells.push((i, j));
                }
            }
        }
        empty_cells
    }

    pub fn unplace(&mut self, row: usize, col: usize) {
        if row < SIZE && col < SIZE {
            self.board[row][col] = Cell::Empty;
        }
    }

    pub fn evaluate(&self) -> i32 {
        if let Some(winner) = self.check_winner() {
            match winner {
                Cell::O => 10,  // L'ordinateur (O) gagne
                Cell::X => -10, // Le joueur humain (X) gagne
                Cell::Empty => 0,
            }
        } else {
            0 // Match nul ou jeu non terminé
        }
    }

    pub fn minimax(&mut self, is_maximizing: bool) -> i32 {
        // Vérifier si le jeu est terminé
        let score = self.evaluate();
        if score == 10 || score == -10 {
            return score;
        }

        if self.is_full() {
            return 0; // Match nul
        }

        if is_maximizing {
            // Tour de l'ordinateur (O) - maximiser le score
            let mut best_score = i32::MIN;
            for (row, col) in self.get_empty_cells() {
                self.board[row][col] = Cell::O;
                let score = self.minimax(false);
                self.board[row][col] = Cell::Empty;
                best_score = best_score.max(score);
            }
            best_score
        } else {
            // Tour du joueur (X) - minimiser le score
            let mut best_score = i32::MAX;
            for (row, col) in self.get_empty_cells() {
                self.board[row][col] = Cell::X;
                let score = self.minimax(true);
                self.board[row][col] = Cell::Empty;
                best_score = best_score.min(score);
            }
            best_score
        }
    }

    pub fn find_best_move(&mut self) -> Option<(usize, usize)> {
        let mut best_score = i32::MIN;
        let mut best_move = None;

        for (row, col) in self.get_empty_cells() {
            self.board[row][col] = Cell::O;
            let score = self.minimax(false);
            self.board[row][col] = Cell::Empty;

            if score > best_score {
                best_score = score;
                best_move = Some((row, col));
            }
        }

        best_move
    }
}
