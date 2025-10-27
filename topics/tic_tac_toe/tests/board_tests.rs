#[cfg(test)]
mod board_tests {

    use tic_tac_toe::board::{Board, Cell};

    /// Test: un nouveau plateau est entièrement vide
    #[test]
    fn test_new_board_is_empty(){
        let board = Board::new();
        for row in 0..3{
            for col in 0..3{
                assert_eq!(board.get_cell(row,col), Cell::Empty);
            }
        }
    }

    /// Test: les fonctions set_cell et get_cell modifient et récupèrent correctement les cellules
    #[test]
    fn test_set_and_get_cell(){
        let mut board = Board::new();
        board.set_cell(1,1, Cell::X);
        assert_eq!(board.get_cell(1,1), Cell::X);
        assert_eq!(board.get_cell(0,0), Cell::Empty);
    }

    /// Test: la fonction is_cell_empty détecte correctement si une cellule est vide
    #[test]
    fn test_is_cell_empty(){
        let mut board = Board::new();
        assert!(board.is_cell_empty(0,0));
        board.set_cell(0,0, Cell::X);
        assert!(!board.is_cell_empty(0,0));
    }

    /// Test: la fonction is_full détecte correctement si le plateau est plein
    #[test]
    fn test_is_full(){
        let mut board = Board::new();
        assert!(!board.is_full());

        for row in 0..3{
            for col in 0..3{
                board.set_cell(row,col,Cell::X);
            }
        }
        assert!(board.is_full());
    }

    /// Test: la fonction check_winner détecte correctement une victoire par ligne
    #[test]
    fn test_check_winner_row(){
        let mut board = Board::new();
        assert!(!board.check_winner(Cell::X));

        board.set_cell(0,0, Cell::X);
        board.set_cell(0,1, Cell::X);
        board.set_cell(0,2, Cell::X);

        assert!(board.check_winner(Cell::X));
        assert!(!board.check_winner(Cell::O));
    }

    /// Test: la fonction check_winner détecte correctement une victoire par colonne
    #[test]
    fn test_check_winner_col(){

        let mut board = Board::new();
        assert!(!board.check_winner(Cell::X));

        board.set_cell(0,0, Cell::X);
        board.set_cell(1,0, Cell::X);
        board.set_cell(2,0, Cell::X);

        assert!(board.check_winner(Cell::X));
        assert!(!board.check_winner(Cell::O));

    }

    /// Test: la fonction check_winner détecte correctement une victoire par diagonale
    #[test]
    fn test_check_winner_diag(){
        
        let mut board = Board::new();
        assert!(!board.check_winner(Cell::X));

        board.set_cell(0,0, Cell::X);
        board.set_cell(1,1, Cell::X);
        board.set_cell(2,2, Cell::X);

        assert!(board.check_winner(Cell::X));

        let mut board = Board::new();

        board.set_cell(0,2, Cell::X);
        board.set_cell(1,1, Cell::X);
        board.set_cell(2,0, Cell::X);

        assert!(board.check_winner(Cell::X));
    }

    /// Test: la fonction available_moves retourne correctement les coups disponibles
    #[test]
    fn test_available_moove(){
        let mut board = Board::new();
        assert_eq!(board.available_moves().len(),9);

        board.set_cell(0,0, Cell::X);
        board.set_cell(1,1, Cell::X);

        let moves = board.available_moves();
        assert_eq!(moves.len(),7);
    }
    
    /// Test: la fonction to_char convertit correctement les cellules en caractères
    #[test]
    fn test_cell_to_char() {
        assert_eq!(Cell::Empty.to_char(), ' ');
        assert_eq!(Cell::X.to_char(), 'X');
        assert_eq!(Cell::O.to_char(), 'O');
    }
}