use std::io::{self, Write};
use crate::board::{Board, Cell};
use super::Player;

/// Structure représentant un joueur humain
pub struct HumanPlayer {
    /// Type de cellule du joueur
    cell_type: Cell,
    /// Nom du joueur
    name: String,
}


impl HumanPlayer {
    /// Fonction qui: Crée un nouveau joueur humain avec le type de cellule et le nom spécifiés
    /// Arguments: `cell_type` - Le type de cellule du joueur, `name` - Le nom du joueur
    /// Returns: Une nouvelle instance de joueur humain (HumanPlayer)
    pub fn new(cell_type: Cell, name: String) -> Self {
        Self { cell_type, name }
    }
}


impl Player for HumanPlayer {
    /// Fonction qui: Retourne le type de cellule du joueur humain
    /// Returns: Le type de cellule du joueur (X ou O) (cell_type: Cell)
    fn get_cell_type(&self) -> Cell {
        self.cell_type
    }

    /// Fonction qui: Retourne le nom du joueur humain
    /// Returns: Le nom du joueur (name: &str)
    fn get_name(&self) -> &str {
        &self.name
    }

    /// Fonction qui: Demande au joueur humain d'entrer un coup valide via la console
    /// Arguments: `board` - Référence au plateau de jeu actuel
    /// Returns: Un tuple représentant la position du coup (row: usize, col: usize)
    fn make_move(&self, board: &Board) -> (usize, usize) {


        loop{
            print!("Enter your move (row col): ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            let coords: Vec<&str> = input.split_whitespace().collect();

            // Vérifie si l'utilisateur a bien entré exactement deux coordonnées
            if coords.len() != 2 {
                println!("Please enter two numbers separated by a space.");
                continue;
            }

            // Convertit la première coordonnée en indice de ligne et vérifie qu'elle est valide (entre 0 et 2)
            let row = match coords[0].parse::<usize>() {
                Ok(num) if num < 3 => num,
                _ => {
                    println!("Row must be a number between 0 and 2.");
                    continue;
                }
            };

            // Convertit la deuxième coordonnée en indice de colonne et vérifie qu'elle est valide (entre 0 et 2)
            let col = match coords[1].parse::<usize>() {
                Ok(num) if num < 3 => num,
                _ => {
                    println!("Column must be a number between 0 and 2.");
                    continue;
                }
            };

            // Vérifie si la cellule sélectionnée est vide et disponible
            if !board.is_cell_empty(row, col) {
                println!("That cell is already taken. Try again.");
                continue;
            }

            return (row, col);

        }
    }
}
