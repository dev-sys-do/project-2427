/// Module pour la gestion du plateau de jeu
pub mod board;
/// Module pour la gestion des joueurs (humain et IA)
pub mod player;
/// Module pour la gestion du déroulement du jeu
pub mod game;

// Réexportation des types principaux pour faciliter leur utilisation
/// Type de cellule (Empty, X, O)
pub use board::Cell;
/// Interface commune à tous les types de joueurs
pub use player::Player;
/// Joueur humain qui interagit via la console
pub use player::human::HumanPlayer;
/// Joueur IA utilisant l'algorithme Minimax
pub use player::ai::AIPlayer;
/// Structure de jeu et énumération des états possibles
pub use game::{Game, GameState};
