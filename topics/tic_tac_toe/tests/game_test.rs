#[cfg(test)]
mod game_tests {
    use tic_tac_toe::board::{Board, Cell};
    use tic_tac_toe::game::{Game, GameState};
    use tic_tac_toe::player::Player;

    /// Joueur factice pour les tests 
    struct MockPlayer {
        cell_type: Cell,
        name: String,
        next_move: (usize, usize),
    }

    impl Player for MockPlayer {
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

    /// Test: la création d'un jeu initialise l'état
    #[test]
    fn test_game_creation() {
        let player1 = Box::new(MockPlayer {
            cell_type: Cell::X,
            name: "Player1".to_string(),
            next_move: (0, 0),
        });
        
        let player2 = Box::new(MockPlayer {
            cell_type: Cell::O,
            name: "Player2".to_string(),
            next_move: (1, 1),
        });
        
        let players: Vec<Box<dyn Player>> = vec![player1, player2];
        let game = Game::new(players);
        
        assert!(matches!(game.state(), GameState::InProgress));
    }

    /// Test: la fonction switch_player alterne entre les joueurs
    #[test]
    fn test_game_switch_player() {
        let player1 = Box::new(MockPlayer {
            cell_type: Cell::X,
            name: "Player1".to_string(),
            next_move: (0, 0),
        });
        
        let player2 = Box::new(MockPlayer {
            cell_type: Cell::O,
            name: "Player2".to_string(),
            next_move: (1, 1),
        });
        
        let players: Vec<Box<dyn Player>> = vec![player1, player2];
        let mut game = Game::new(players);
        
        let first_player_cell = game.current_player().get_cell_type();
        game.switch_player();
        let second_player_cell = game.current_player().get_cell_type();
        
        assert_ne!(first_player_cell, second_player_cell);
    }

    /// Test: la fonction update_state détecte une victoire
    #[test]
    fn test_game_win_detection() {
        let player1 = Box::new(MockPlayer {
            cell_type: Cell::X,
            name: "Player1".to_string(),
            next_move: (0, 0),
        });
        
        let player2 = Box::new(MockPlayer {
            cell_type: Cell::O,
            name: "Player2".to_string(),
            next_move: (1, 0),
        });
        
        let players: Vec<Box<dyn Player>> = vec![player1, player2];
        let mut game = Game::new(players);
        
        game.get_board_mut().set_cell(0, 0, Cell::X);
        game.get_board_mut().set_cell(0, 1, Cell::X);
        game.get_board_mut().set_cell(0, 2, Cell::X);
        
        game.update_state();
        
        assert!(matches!(game.state(), GameState::Win(Cell::X)));
    }

    /// Test: la fonction update_state détecte un match nul
    #[test]
    fn test_game_draw_detection() {
        let player1 = Box::new(MockPlayer {
            cell_type: Cell::X,
            name: "Player1".to_string(),
            next_move: (0, 0),
        });
        
        let player2 = Box::new(MockPlayer {
            cell_type: Cell::O,
            name: "Player2".to_string(),
            next_move: (1, 0),
        });
        
        let players: Vec<Box<dyn Player>> = vec![player1, player2];
        let mut game = Game::new(players);
        
        game.get_board_mut().set_cell(0, 0, Cell::X);
        game.get_board_mut().set_cell(0, 1, Cell::O);
        game.get_board_mut().set_cell(0, 2, Cell::X);
        game.get_board_mut().set_cell(1, 0, Cell::X);
        game.get_board_mut().set_cell(1, 1, Cell::O);
        game.get_board_mut().set_cell(1, 2, Cell::X);
        game.get_board_mut().set_cell(2, 0, Cell::O);
        game.get_board_mut().set_cell(2, 1, Cell::X);
        game.get_board_mut().set_cell(2, 2, Cell::O);
        
        game.update_state();
        
        assert!(matches!(game.state(), GameState::Draw));
    }
    
    /// Test: la fonction play_turn exécute un tour de jeu
    #[test]
    fn test_game_play_turn() {
        let player1 = Box::new(MockPlayer {
            cell_type: Cell::X,
            name: "Player1".to_string(),
            next_move: (0, 0),
        });
        
        let player2 = Box::new(MockPlayer {
            cell_type: Cell::O,
            name: "Player2".to_string(),
            next_move: (1, 1),
        });
        
        let players: Vec<Box<dyn Player>> = vec![player1, player2];
        let mut game = Game::new(players);
        
        assert!(matches!(game.state(), GameState::InProgress));
        
        game.play_turn();
        
        assert_eq!(game.get_board_mut().get_cell(0, 0), Cell::X);
        
        assert_eq!(game.current_player().get_cell_type(), Cell::O);
    }
    
    /// Test: scénario complet d'une partie jusqu'à la victoire d'un joueur
    #[test]
    fn test_full_game_scenario() {
        let player1 = Box::new(MockPlayer {
            cell_type: Cell::X,
            name: "Player1".to_string(),
            next_move: (0, 0), 
        });
        
        let player2 = Box::new(MockPlayer {
            cell_type: Cell::O,
            name: "Player2".to_string(),
            next_move: (1, 0), 
        });
        
        let players: Vec<Box<dyn Player>> = vec![player1, player2];
        let mut game = Game::new(players);
        
        fn play_turn_with_move(game: &mut Game, row: usize, col: usize) {
            let cell_type = game.current_player().get_cell_type();
            game.get_board_mut().set_cell(row, col, cell_type);
            
            game.update_state();
            if matches!(game.state(), GameState::InProgress) {
                game.switch_player();
            }
        }
        
        play_turn_with_move(&mut game, 0, 0); 
        assert!(matches!(game.state(), GameState::InProgress));
        
        play_turn_with_move(&mut game, 1, 0); 
        assert!(matches!(game.state(), GameState::InProgress));
        
        play_turn_with_move(&mut game, 0, 1); 
        assert!(matches!(game.state(), GameState::InProgress));
        
        play_turn_with_move(&mut game, 1, 1); 
        assert!(matches!(game.state(), GameState::InProgress));
        
        play_turn_with_move(&mut game, 0, 2); 
        
        assert!(matches!(game.state(), GameState::Win(Cell::X)));
    }
}