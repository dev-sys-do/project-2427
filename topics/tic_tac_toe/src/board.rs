/// Énumération représentant l'état d'une cellule sur le plateau de jeu
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    /// Cellule vide
    Empty,
    /// Cellule occupée par le joueur X
    X,
    /// Cellule occupée par le joueur O
    O,
}

impl Cell {
    /// Fonction qui: Convertit une cellule en caractère pour l'affichage dans la console
    /// Returns: Un caractère représentant la cellule (' ' pour Empty, 'X' pour X, 'O' pour O) (char)
    pub fn to_char(self) -> char {

        match self {
            Cell::Empty => ' ',
            Cell::X => 'X',
            Cell::O => 'O',
        }
    }
}

/// Structure représentant le plateau de jeu du Tic Tac Toe (grille 3x3)
#[derive(Debug, Clone)]
pub struct Board {
    /// Grille 3x3 
    cells: [[Cell; 3]; 3],
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    /// Fonction qui: Crée un nouveau plateau de jeu vide
    /// Returns: Un nouveau plateau avec toutes les cellules vides (Board)
    pub fn new() -> Self {
        Self {
            cells: [[Cell::Empty; 3]; 3]
        }
    }

    /// Fonction qui: Récupère l'état d'une cellule à une position donnée
    /// Arguments: `row` - L'indice de ligne (0-2), `col` - L'indice de colonne (0-2)
    /// Returns: L'état de la cellule à la position spécifiée (cell: Cell)
    pub fn get_cell(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }

    /// Fonction qui: Définit l'état d'une cellule à une position donnée
    /// Arguments: `row` - L'indice de ligne (0-2), `col` - L'indice de colonne (0-2), `cell` - Le nouvel état de la cellule
    pub fn set_cell(&mut self, row: usize, col: usize, cell: Cell){
        self.cells[row][col] = cell;
    }

    /// Fonction qui: Vérifie si une cellule est vide à une position donnée
    /// Arguments: `row` - L'indice de ligne (0-2), `col` - L'indice de colonne (0-2)
    /// Returns: `true` si la cellule est vide, `false` sinon 
    pub fn is_cell_empty(&self, row: usize, col: usize) -> bool {
        self.cells[row][col] == Cell::Empty
    }

    /// Fonction qui: Récupère la liste des coups disponibles (cellules vides)
    /// Returns: Un vecteur de tuples représentant les positions des cellules vides 
    pub fn available_moves(&self) -> Vec<(usize,usize)> {
        let mut moves = Vec::new();
        // Parcourt toutes les cellules du plateau
        for row in 0..3{
            for col in 0..3{
                // Ajoute la position à la liste si la cellule est vide
                if self.is_cell_empty(row,col){
                    moves.push((row,col));
                }
            }
        }
        moves
    }


    /// Fonction qui: Vérifie si le plateau est plein (aucune cellule vide)
    /// Returns: `true` si toutes les cellules sont occupées, `false` sinon 
    pub fn is_full(&self) -> bool {
        // Parcourt toutes les cellules du plateau
        for row in 0..3{
            for col in 0..3{
                // Si une cellule est vide, le plateau n'est pas plein
                if self.is_cell_empty(row,col){
                    return false;
                }
            }
        }

        true
    }

    /// Fonction qui: Vérifie si un joueur a gagné en alignant trois symboles
    /// Arguments: `player` - Le type de cellule du joueur (X ou O) à vérifier
    /// Returns: `true` si le joueur a gagné, `false` sinon 
    pub fn check_winner(&self, player: Cell) -> bool {

        // Vérifie les lignes pour un alignement horizontal
        for row in 0..3{
            if self.cells[row][0] == player && self.cells[row][1] == player && self.cells[row][2] == player {
                return true;
            }
        }

        // Vérifie les colonnes pour un alignement vertical
        for col in 0..3{
            if self.cells[0][col] == player && self.cells[1][col] == player && self.cells[2][col] == player {
                return true;
            }
        }

        // Vérifie la diagonale principale (haut gauche à bas droite)
        if self.cells[0][0] == player && self.cells[1][1] == player && self.cells[2][2] == player{
            return true;
        }

        // Vérifie la diagonale secondaire (bas gauche à haut droite)
        if self.cells[2][0] == player && self.cells[1][1] == player && self.cells[0][2] == player {
            return true;
        }

        false
    }

    /// Fonction qui: Affiche le plateau de jeu dans la console avec des bordures Unicode
    pub fn display(&self) {
        println!("    0   1   2  ");
        println!("  ┌───┬───┬───┐");
        for row in 0..3 {
            print!("{} │", row);
            for col in 0..3 {
                let cell = self.get_cell(row, col);
                match cell {
                    Cell::X => print!(" X │"),
                    Cell::O => print!(" O │"),
                    Cell::Empty => print!("   │"),
                }
            }
            println!();
            if row < 2 {
                println!("  ├───┼───┼───┤");
            } else {
                println!("  └───┴───┴───┘");
            }
        }
        println!();
    }
}






