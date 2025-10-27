use crate::board::{Board, Cell};
use crate::player::Player;

/// Énumération représentant l'état du jeu
pub enum GameState{
    /// Jeu en cours
    InProgress,
    /// Victoire d'un joueur, contient le type de cellule du gagnant
    Win(Cell),
    /// Match nul
    Draw,
}

/// Structure  pour la gestion du jeu
pub struct Game{
    /// Plateau de jeu
    board: Board,
    /// Index du joueur actuel
    current_player_index: usize,
    /// Liste des joueurs
    players: Vec<Box<dyn Player>>,
    /// État actuel du jeu
    state: GameState,
}

impl Game{

    /// Fonction qui: Crée une nouvelle partie avec les joueurs spécifiés
    /// Arguments: `players` - Vecteur contenant les joueurs (doit contenir exactement 2 joueurs)
    /// Returns: Une nouvelle instance de jeu avec un plateau vide (Game)
    /// Panics: Si le nombre de joueurs n'est pas égal à 2
    pub fn new(players: Vec<Box<dyn Player>>) -> Self {
        if players.len() != 2 {
            panic!("Game must have exactly 2 players");
        }
        
        Self {
            board: Board::new(),
            current_player_index: 0,
            players,
            state: GameState::InProgress,
        }
    }

    /// Fonction qui: Retourne une référence au joueur dont c'est le tour
    /// Returns: Une référence au joueur actuel (&dyn Player)
    pub fn current_player(&self) -> &dyn Player {
        self.players[self.current_player_index].as_ref()
    }

    /// Fonction qui: Passe au joueur suivant en alternant entre les deux joueurs
    pub fn switch_player(&mut self){
        self.current_player_index = (self.current_player_index +1) % self.players.len();
    }


    /// Fonction qui: Met à jour l'état du jeu (vérifie s'il y a un gagnant ou un match nul)
    pub fn update_state(&mut self){
        // Vérifie si un des joueurs a gagné
        for player in &self.players{
            // Si un joueur a aligné trois symboles, met à jour l'état du jeu en victoire
            if self.board.check_winner(player.get_cell_type()){
                self.state = GameState::Win(player.get_cell_type());
                return;
            }
        }

        // Si le plateau est plein et qu'aucun joueur n'a gagné, c'est un match nul
        if self.board.is_full(){
            self.state = GameState::Draw;
        }
    }

    /// Fonction qui: Retourne l'état actuel du jeu
    /// Returns: Une référence à l'état actuel du jeu (&GameState)
    pub fn state(&self) -> &GameState{
        &self.state
    }
    
    /// Fonction qui: Retourne une référence mutable au plateau de jeu (pour les tests)
    /// Returns: Une référence mutable au plateau de jeu (&mut Board)
    #[allow(dead_code)]
    pub fn get_board_mut(&mut self) -> &mut Board {
        &mut self.board
    }

    /// Fonction qui: Exécute un tour de jeu complet (affichage, coup, mise à jour)
    pub fn play_turn(&mut self) {
        let player_name = self.current_player().get_name().to_string();
        let cell_type = self.current_player().get_cell_type();
        let cell_char = cell_type.to_char();
        
        println!("{}'s turn ({})", player_name, cell_char);
        
        self.board.display();
        
        let (row, col) = self.current_player().make_move(&self.board);
        
        self.board.set_cell(row, col, cell_type);
        
        println!("{} placed at position ({}, {})", player_name, row, col);
        
        self.update_state();
        // Passe au joueur suivant seulement si le jeu est toujours en cours
        if matches!(self.state, GameState::InProgress) {
            self.switch_player();
        }
    }

    /// Fonction qui: Affiche le résultat final de la partie (victoire ou match nul)
    pub fn display_result(&self) {
        self.board.display();
        
        // Affiche un message différent selon l'état final du jeu
        match &self.state {
            // En cas de victoire, trouve le joueur gagnant et affiche son nom
            GameState::Win(cell) => {
                let winner = self.players.iter()
                    .find(|p| p.get_cell_type() == *cell)
                    .unwrap();
                println!("{} wins!", winner.get_name());
            }
            // En cas de match nul, affiche un message approprié
            GameState::Draw => {
                println!("The game ended in a draw.");
            }
            // Si le jeu est encore en cours (cas rare ici), affiche un message d'information
            GameState::InProgress => {
                println!("The game is still in progress.");
            }
        }
    }

}