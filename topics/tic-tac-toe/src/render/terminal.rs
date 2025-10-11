use crate::board::{Board, Cell, Player};
use crate::game::GameState;
use std::io::{self, Write};

/// Clears the terminal screen using ANSI escape codes
pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

/// Displays the game title banner
pub fn display_title() {
    println!("╔═══════════════════════════════════════╗");
    println!("║                                       ║");
    println!("║            TIC-TAC-TOE                ║");
    println!("║                                       ║");
    println!("╚═══════════════════════════════════════╝");
    println!();
}

/// Displays the current board state with Unicode box-drawing characters
pub fn display_board(board: &Board) {
    println!();
    println!("     ╔═══╦═══╦═══╗");

    for row in 0..3 {
        print!("     ║");
        for col in 0..3 {
            let pos = row * 3 + col;
            let symbol = get_cell_display(board, pos);
            print!(" {} ║", symbol);
        }
        println!();

        if row < 2 {
            println!("     ╠═══╬═══╬═══╣");
        }
    }

    println!("     ╚═══╩═══╩═══╝");
    println!();
}

/// Displays the position guide to help players
pub fn display_position_guide() {
    println!("  How to play: Enter a number from 1 to 9");
    println!();
    println!("     ╔═══╦═══╦═══╗");
    println!("     ║ 1 ║ 2 ║ 3 ║");
    println!("     ╠═══╬═══╬═══╣");
    println!("     ║ 4 ║ 5 ║ 6 ║");
    println!("     ╠═══╬═══╬═══╣");
    println!("     ║ 7 ║ 8 ║ 9 ║");
    println!("     ╚═══╩═══╩═══╝");
    println!();
}

/// Returns the display character for a cell
fn get_cell_display(board: &Board, position: usize) -> String {
    match board.get(position) {
        Some(Cell::Empty) => " ".to_string(),
        Some(Cell::Occupied(Player::Human)) => "X".to_string(),
        Some(Cell::Occupied(Player::Ai)) => "O".to_string(),
        None => " ".to_string(),
    }
}

/// Prompts the player for their move and validates input
pub fn get_player_move(board: &Board) -> usize {
    loop {
        print!("  > Enter your move (1-9): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let position: usize = match input.trim().parse::<usize>() {
            Ok(num) if (1..=9).contains(&num) => num - 1,
            _ => {
                println!("  Please enter a number between 1 and 9.");
                continue;
            }
        };

        if let Some(cell) = board.get(position) {
            if cell.is_empty() {
                return position;
            } else {
                println!("  That position is already occupied. Try another one.");
                continue;
            }
        }
    }
}

/// Displays the game outcome
pub fn display_game_status(state: GameState) {
    println!();
    match state {
        GameState::InProgress => {
            println!("  Game in progress...");
        }
        GameState::Won(Player::Human) => {
            println!("  ╔═══════════════════════════════════════╗");
            println!("  ║                                       ║");
            println!("  ║           You win! Great job!         ║");
            println!("  ║                                       ║");
            println!("  ╚═══════════════════════════════════════╝");
        }
        GameState::Won(Player::Ai) => {
            println!("  ╔═══════════════════════════════════════╗");
            println!("  ║                                       ║");
            println!("  ║         You lost. Try again!          ║");
            println!("  ║                                       ║");
            println!("  ╚═══════════════════════════════════════╝");
        }
        GameState::Draw => {
            println!("  ╔═══════════════════════════════════════╗");
            println!("  ║                                       ║");
            println!("  ║          Draw! Well played.           ║");
            println!("  ║                                       ║");
            println!("  ╚═══════════════════════════════════════╝");
        }
    }
    println!();
}

/// Displays a move notification (unused but available)
#[allow(dead_code)]
pub fn display_move(player: Player, position: usize) {
    let symbol = player.symbol();
    let player_name = match player {
        Player::Human => "You",
        Player::Ai => "Opponent",
    };
    println!(
        "  {} placed {} at position {}",
        player_name,
        symbol,
        position + 1
    );
    println!();
}

/// Asks if the player wants to play again
pub fn ask_play_again() -> bool {
    loop {
        print!("  > Play again? (y/n): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("  Please enter 'y' or 'n'."),
        }
    }
}

/// Displays a separator line (unused but available)
#[allow(dead_code)]
pub fn display_separator() {
    println!("  ─────────────────────────────────────────");
}

/// Displays an animation while the AI is thinking
pub fn display_ai_thinking() {
    print!("  Opponent is playing");
    io::stdout().flush().unwrap();

    for _ in 0..3 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        print!(".");
        io::stdout().flush().unwrap();
    }
    println!();
}
