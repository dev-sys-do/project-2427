use crate::board::{Board, Cell};
use super::Player;

/// Joueur IA utilisant l'algorithme Minimax
pub struct AIPlayer {
    /// Type de cellule du joueur
    cell_type: Cell,
    /// Nom du joueur
    name: String,
}

impl AIPlayer {
    /// Fonction qui: Crée un nouveau joueur IA avec le type de cellule et le nom spécifiés
    /// Arguments: `cell_type` - Le type de cellule du joueur, `name` - Le nom du joueur
    /// Returns: Une nouvelle instance de joueur IA (AIPlayer)
    pub fn new(cell_type: Cell, name: String) -> Self {
        Self { cell_type, name }
    }

    /// Fonction qui: Retourne le type de cellule de l'adversaire
    /// Returns: Le type de cellule de l'adversaire (X si l'IA est O, O si l'IA est X) (Cell)
    pub fn opponent_cell(&self) -> Cell {
        // Détermine le type de cellule de l'adversaire en fonction du type de cellule de l'IA
        match self.cell_type {
            Cell::X => Cell::O,
            Cell::O => Cell::X,
            Cell::Empty => panic!("AI player cannot have Empty cell type"),
        }
    }

    /// Fonction qui: Implémente l'algorithme Minimax pour déterminer le meilleur coup
    /// Arguments: `board` - État du plateau, `depth` - Profondeur actuelle, `is_maximizing` - Tour du joueur maximisant
    /// Returns: Score de la position (-10 à +10) (score: i32)
    pub fn minimax(&self, board: &Board, depth: i32, is_maximizing: bool) -> i32 {
        let ai_cell = self.cell_type;
        let human_cell = self.opponent_cell();
        
        // Vérifie les états terminaux du jeu et retourne le score approprié
        if board.check_winner(ai_cell) {
            // L'IA a gagné, retourne un score positif (ajusté par la profondeur)
            return 10 - depth;
        }
        if board.check_winner(human_cell) {
            // L'humain a gagné, retourne un score négatif (ajusté par la profondeur)
            return depth - 10;
        }
        if board.is_full() {
            // Match nul, retourne un score neutre
            return 0;
        }
        
        // Calcule récursivement le meilleur score pour chaque coup possible
        if is_maximizing {
            // Tour du joueur maximisant (IA) - cherche le score maximum
            let mut best_score = i32::MIN;
            for (row, col) in board.available_moves() {
                // Simule le coup de l'IA
                let mut new_board = board.clone();
                new_board.set_cell(row, col, ai_cell);
                // Calcule récursivement le score pour ce coup
                let score = self.minimax(&new_board, depth + 1, false);
                // Met à jour le meilleur score
                best_score = best_score.max(score);
            }
            best_score
        } else {
            // Tour du joueur minimisant (humain) - cherche le score minimum
            let mut best_score = i32::MAX;
            for (row, col) in board.available_moves() {
                // Simule le coup de l'humain
                let mut new_board = board.clone();
                new_board.set_cell(row, col, human_cell);
                // Calcule récursivement le score pour ce coup
                let score = self.minimax(&new_board, depth + 1, true);
                // Met à jour le meilleur score
                best_score = best_score.min(score);
            }
            best_score
        }
    }
}

impl Player for AIPlayer {
    /// Fonction qui: Retourne le type de cellule du joueur IA
    /// Returns: Le type de cellule du joueur (X ou O) (cell_type: Cell)
    fn get_cell_type(&self) -> Cell {
        self.cell_type
    }

    /// Fonction qui: Retourne le nom du joueur IA
    /// Returns: Le nom du joueur (name: &str)
    fn get_name(&self) -> &str {
        &self.name
    }

    /// Fonction qui: Détermine le meilleur coup à jouer en utilisant l'algorithme Minimax
    /// Arguments: `board` - État actuel du plateau
    /// Returns: Un tuple représentant la position du meilleur coup (row: usize, col: usize)
    fn make_move(&self, board: &Board) -> (usize, usize) {
        println!("{} is thinking...", self.name);
        
        let mut best_score = i32::MIN;
        let mut best_move = (0, 0);
        
        // Parcourt tous les coups disponibles pour trouver celui avec le meilleur score
        for (row, col) in board.available_moves() {
            // Simule le coup de l'IA sur une copie du plateau
            let mut new_board = board.clone();
            new_board.set_cell(row, col, self.cell_type);
            // Calcule le score pour ce coup en utilisant l'algorithme Minimax
            let score = self.minimax(&new_board, 0, false);
            
            // Met à jour le meilleur coup si le score est supérieur
            if score > best_score {
                best_score = score;
                best_move = (row, col);
            }
        }
        
        best_move
    }
}

