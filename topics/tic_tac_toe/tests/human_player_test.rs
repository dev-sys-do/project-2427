#[cfg(test)]
mod human_player_tests {
    use tic_tac_toe::board::{Board, Cell};
    use tic_tac_toe::player::Player;
    use tic_tac_toe::player::human::HumanPlayer;

    /// Joueur factice pour les tests
    struct MockHumanPlayer {
        cell_type: Cell,
        name: String,
        next_move: (usize, usize),
    }

    impl Player for MockHumanPlayer {
        fn get_cell_type(&self) -> Cell {
            self.cell_type
        }
        
        fn get_name(&self) -> &str {
            &self.name
        }
        
        fn make_move(&self, _board: &Board) -> (usize, usize) {
            self.next_move
        }
    }

    /// Test: la création d'un joueur humain initialise ses propriétés
    #[test]
    fn test_human_player_creation() {
        let human = HumanPlayer::new(Cell::X, "Alice".to_string());
        assert_eq!(human.get_cell_type(), Cell::X);
        assert_eq!(human.get_name(), "Alice");
    }

    /// Test: le joueur factice retourne le coup prédéfini
    #[test]
    fn test_mock_human_player_move() {
        let board = Board::new();
        let mock_human = MockHumanPlayer {
            cell_type: Cell::O,
            name: "MockHuman".to_string(),
            next_move: (1, 2),
        };
        
        let (row, col) = mock_human.make_move(&board);
        assert_eq!((row, col), (1, 2));
    }
    
    /// Test: la fonction get_cell_type retourne le bon type de cellule
    #[test]
    fn test_get_cell_type() {
        let human_x = HumanPlayer::new(Cell::X, "Player_X".to_string());
        let human_o = HumanPlayer::new(Cell::O, "Player_O".to_string());
        
        assert_eq!(human_x.get_cell_type(), Cell::X);
        assert_eq!(human_o.get_cell_type(), Cell::O);
    }
    
    /// Test: la fonction get_name retourne le bon nom du joueur
    #[test]
    fn test_get_name() {
        let human = HumanPlayer::new(Cell::X, "Human_Player".to_string());
        
        assert_eq!(human.get_name(), "Human_Player");
    }
}
