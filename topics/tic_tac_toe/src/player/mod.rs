use crate::board::{Board, Cell};

/// Module pour l'implémentation du joueur humain
pub mod human;
/// Module pour l'implémentation du joueur IA 
pub mod ai;

/// Trait définissant l'interface commune à tous les types de joueurs (IA et Humain)
pub trait Player {

    /// Fonction qui: Retourne le type de cellule du joueur
    /// Returns: Le type de cellule utilisé par le joueur (X ou O) (cell_type: Cell)
    fn get_cell_type(&self) -> Cell;
    
    /// Fonction qui: Détermine le prochain coup à jouer sur le plateau
    /// Arguments: `board` - Référence au plateau de jeu actuel 
    /// Returns: Un tuple représentant la position du coup à jouer (row: usize, col: usize)
    fn make_move(&self, board: &Board) -> (usize, usize);
    
    /// Fonction qui: Retourne le nom du joueur
    /// Returns: Le nom du joueur sous forme de référence à une chaîne de caractères (name: &str)
    fn get_name(&self) -> &str;
}