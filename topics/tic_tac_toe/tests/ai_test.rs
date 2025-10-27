#[cfg(test)]
mod ai_player_tests {

    use tic_tac_toe::board::{Board, Cell};
    use tic_tac_toe::player::Player;
    use tic_tac_toe::player::ai::AIPlayer;

    /// Test: création d'un joueur IA et vérification de ses propriétés
    #[test]
    fn test_ai_player_creation(){
        let player = AIPlayer::new(Cell::X, "AI".to_string());
        assert_eq!(player.get_cell_type(),Cell::X);
        assert_eq!(player.get_name(),"AI");
    }

    /// Test: l'IA bloque un coup gagnant de l'adversaire
    #[test]
    fn test_ai_block_winning_move(){
        let mut board = Board::new();
        let player = AIPlayer::new(Cell::X, "AI".to_string());

        board.set_cell(0,0,Cell::O);
        board.set_cell(0,1,Cell::O);

        let (row,col) = player.make_move(&board);
        assert_eq!((row,col),(0,2));
    }

    /// Test: l'IA choisit un coup gagnant quand c'est possible
    #[test]
    fn test_ai_makes_winning_move() {
        let mut board = Board::new();
        let ai = AIPlayer::new(Cell::X, "AI".to_string());
        
        board.set_cell(0, 0, Cell::X);
        board.set_cell(0, 1, Cell::X);
        board.set_cell(1, 0, Cell::O);
        board.set_cell(1, 1, Cell::O);
        
        let (row, col) = ai.make_move(&board);
        assert_eq!((row, col), (0, 2));
    }
    
    /// Test: l'algorithme Minimax évalue correctement les positions gagnantes et perdantes
    #[test]
    fn test_minimax_algorithm() {
        let ai = AIPlayer::new(Cell::X, "AI".to_string());
        
        let mut board = Board::new();
        board.set_cell(0, 0, Cell::X);
        board.set_cell(0, 1, Cell::X);
        
        let score = ai.minimax(&board, 0, true);
        assert!(score > 0, "Minimax should return a positive score for a winning position");
        
        let mut board = Board::new();
        board.set_cell(0, 0, Cell::O);
        board.set_cell(0, 1, Cell::O);
        
        let score = ai.minimax(&board, 0, true);
        assert!(score < 0, "Minimax should return a negative score for a losing position");
        
        let board = Board::new();
        let score = ai.minimax(&board, 0, true);
        assert!(score > -5 && score < 5, "Minimax should return a moderate score for an empty board");
    }
    
    /// Test: l'algorithme Minimax retourne un score approprié pour un plateau vide
    #[test]
    fn test_minimax_empty_board() {
        let board = Board::new();
        let ai = AIPlayer::new(Cell::X, "AI".to_string());
        
        let score = ai.minimax(&board, 0, true);
        assert!(score > -5 && score < 5, "Minimax should return a moderate score for an empty board");
    }
    
    /// Test: la fonction opponent_cell retourne le type de cellule opposé
    #[test]
    fn test_opponent_cell() {
        let ai_x = AIPlayer::new(Cell::X, "AI_X".to_string());
        let ai_o = AIPlayer::new(Cell::O, "AI_O".to_string());
        
        assert_eq!(ai_x.opponent_cell(), Cell::O);
        assert_eq!(ai_o.opponent_cell(), Cell::X);
    }
    
    /// Test: la fonction get_cell_type retourne le bon type de cellule
    #[test]
    fn test_get_cell_type() {
        let ai_x = AIPlayer::new(Cell::X, "AI_X".to_string());
        let ai_o = AIPlayer::new(Cell::O, "AI_O".to_string());
        
        assert_eq!(ai_x.get_cell_type(), Cell::X);
        assert_eq!(ai_o.get_cell_type(), Cell::O);
    }
    
    /// Test: la fonction get_name retourne le bon nom du joueur
    #[test]
    fn test_get_name() {
        let ai = AIPlayer::new(Cell::X, "AI_Player".to_string());
        
        assert_eq!(ai.get_name(), "AI_Player");
    }
}
