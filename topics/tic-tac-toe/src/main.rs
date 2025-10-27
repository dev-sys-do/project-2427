mod game;

use game::{TicTacToe, Cell};
use rand::Rng;
use std::io::{self, Write};

fn main() {
    println!("=== Tic-Tac-Toe ===");
    println!("Choisissez le mode de jeu :");
    println!("1. Deux joueurs");
    println!("2. Contre l'ordinateur (aléatoire)");
    println!("3. Contre l'ordinateur (Minimax - imbattable)");
    print!("> ");
    io::stdout().flush().unwrap();

    let mut mode_input = String::new();
    io::stdin().read_line(&mut mode_input).unwrap();
    let mode = mode_input.trim();

    match mode {
        "1" => play_two_players(),
        "2" => play_against_computer(),
        "3" => play_against_minimax(),
        _ => {
            println!("Mode invalide ! Relancez le jeu.");
        }
    }
}

fn play_two_players() {
    let mut game = TicTacToe::new();
    let mut current_player = Cell::X;

    println!("\n=== Mode Deux Joueurs ===");
    println!("Joueur X commence !");
    println!("Entrez les coordonnées comme : ligne colonne (1-3)");
    println!("Exemple : 1 1 pour le coin supérieur gauche\n");

    loop {
        game.display();

        if let Some(winner) = game.check_winner() {
            println!("Le joueur {:?} a gagné !", winner);
            break;
        }

        if game.is_full() {
            println!("Match nul ! La grille est pleine.");
            break;
        }

        println!("Joueur {:?}, entrez votre coup (ligne colonne) :", current_player);
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let coords: Vec<&str> = input.trim().split_whitespace().collect();
        if coords.len() != 2 {
            println!("Entrée invalide ! Utilisez le format : ligne colonne (ex: 1 1)");
            continue;
        }

        let row = match coords[0].parse::<usize>() {
            Ok(r) if r >= 1 && r <= 3 => r - 1,
            Ok(_) => {
                println!("Ligne invalide ! Utilisez un nombre entre 1 et 3.");
                continue;
            }
            Err(_) => {
                println!("Ligne invalide !");
                continue;
            }
        };

        let col = match coords[1].parse::<usize>() {
            Ok(c) if c >= 1 && c <= 3 => c - 1,
            Ok(_) => {
                println!("Colonne invalide ! Utilisez un nombre entre 1 et 3.");
                continue;
            }
            Err(_) => {
                println!("Colonne invalide !");
                continue;
            }
        };

        if game.place(row, col, current_player) {
            current_player = match current_player {
                Cell::X => Cell::O,
                Cell::O => Cell::X,
                Cell::Empty => Cell::X,
            };
        } else {
            println!("Coup invalide ! La case est déjà occupée ou hors limites.");
        }
    }

    game.display();
    println!("\nMerci d'avoir joué !");
}

fn play_against_computer() {
    let mut game = TicTacToe::new();
    let mut rng = rand::thread_rng();

    println!("\n=== Mode Contre l'Ordinateur ===");
    println!("Vous êtes X, l'ordinateur est O");
    println!("Entrez les coordonnées comme : ligne colonne (1-3)");
    println!("Exemple : 1 1 pour le coin supérieur gauche\n");

    loop {
        game.display();

        // Vérifier victoire ou match nul
        if let Some(winner) = game.check_winner() {
            match winner {
                Cell::X => println!("Vous avez gagné !"),
                Cell::O => println!("L'ordinateur a gagné !"),
                _ => {}
            }
            break;
        }

        if game.is_full() {
            println!("Match nul ! La grille est pleine.");
            break;
        }

        // Tour du joueur humain (X)
        println!("Votre tour (X), entrez votre coup (ligne colonne) :");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let coords: Vec<&str> = input.trim().split_whitespace().collect();
        if coords.len() != 2 {
            println!("Entrée invalide ! Utilisez le format : ligne colonne (ex: 1 1)");
            continue;
        }

        let row = match coords[0].parse::<usize>() {
            Ok(r) if r >= 1 && r <= 3 => r - 1,
            Ok(_) => {
                println!("Ligne invalide ! Utilisez un nombre entre 1 et 3.");
                continue;
            }
            Err(_) => {
                println!("Ligne invalide !");
                continue;
            }
        };

        let col = match coords[1].parse::<usize>() {
            Ok(c) if c >= 1 && c <= 3 => c - 1,
            Ok(_) => {
                println!("Colonne invalide ! Utilisez un nombre entre 1 et 3.");
                continue;
            }
            Err(_) => {
                println!("❌ Colonne invalide !");
                continue;
            }
        };

        if !game.place(row, col, Cell::X) {
            println!("Coup invalide ! La case est déjà occupée.");
            continue;
        }

        // Vérifier victoire après coup du joueur
        if let Some(winner) = game.check_winner() {
            game.display();
            if winner == Cell::X {
                println!("Vous avez gagné !");
            }
            break;
        }

        if game.is_full() {
            game.display();
            println!("Match nul ! La grille est pleine.");
            break;
        }

        // Tour de l'ordinateur (O)
        println!("\nL'ordinateur réfléchit...");
        std::thread::sleep(std::time::Duration::from_millis(800));

        let empty_cells = game.get_empty_cells();
        if !empty_cells.is_empty() {
            let random_index = rng.gen_range(0..empty_cells.len());
            let (comp_row, comp_col) = empty_cells[random_index];
            game.place(comp_row, comp_col, Cell::O);
            println!("L'ordinateur joue en ({}, {})", comp_row + 1, comp_col + 1);
        }
    }

    game.display();
    println!("\nMerci d'avoir joué !");
}

fn play_against_minimax() {
    let mut game = TicTacToe::new();

    println!("\n=== Mode Contre l'Ordinateur (Minimax) ===");
    println!("Vous êtes X, l'ordinateur est O");
    println!("L'ordinateur utilise l'algorithme Minimax - il est imbattable !");
    println!("Entrez les coordonnées comme : ligne colonne (1-3)");
    println!("Exemple : 1 1 pour le coin supérieur gauche\n");

    loop {
        game.display();

        // Vérifier victoire ou match nul
        if let Some(winner) = game.check_winner() {
            match winner {
                Cell::X => println!("Vous avez gagné ! Incroyable !"),
                Cell::O => println!("L'ordinateur a gagné !"),
                _ => {}
            }
            break;
        }

        if game.is_full() {
            println!("Match nul ! Bien joué !");
            break;
        }

        // Tour du joueur humain (X)
        println!("Votre tour (X), entrez votre coup (ligne colonne) :");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let coords: Vec<&str> = input.trim().split_whitespace().collect();
        if coords.len() != 2 {
            println!("Entrée invalide ! Utilisez le format : ligne colonne (ex: 1 1)");
            continue;
        }

        let row = match coords[0].parse::<usize>() {
            Ok(r) if r >= 1 && r <= 3 => r - 1,
            Ok(_) => {
                println!("Ligne invalide ! Utilisez un nombre entre 1 et 3.");
                continue;
            }
            Err(_) => {
                println!("Ligne invalide !");
                continue;
            }
        };

        let col = match coords[1].parse::<usize>() {
            Ok(c) if c >= 1 && c <= 3 => c - 1,
            Ok(_) => {
                println!("Colonne invalide ! Utilisez un nombre entre 1 et 3.");
                continue;
            }
            Err(_) => {
                println!("Colonne invalide !");
                continue;
            }
        };

        if !game.place(row, col, Cell::X) {
            println!("Coup invalide ! La case est déjà occupée.");
            continue;
        }

        // Vérifier victoire après coup du joueur
        if let Some(winner) = game.check_winner() {
            game.display();
            if winner == Cell::X {
                println!("Vous avez gagné ! Incroyable !");
            }
            break;
        }

        if game.is_full() {
            game.display();
            println!("Match nul ! Bien joué !");
            break;
        }

        // Tour de l'ordinateur (O) avec Minimax
        println!("\nL'ordinateur calcule le meilleur coup (Minimax)...");
        std::thread::sleep(std::time::Duration::from_millis(500));

        if let Some((comp_row, comp_col)) = game.find_best_move() {
            game.place(comp_row, comp_col, Cell::O);
            println!("L'ordinateur joue en ({}, {})", comp_row + 1, comp_col + 1);
        }
    }

    game.display();
    println!("\nMerci d'avoir joué !");
}
