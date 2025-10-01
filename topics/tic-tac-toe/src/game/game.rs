use crate::game::board::Board;
use crate::game::robot::Robot;
use std::io;

pub struct Game {
    board: Board,
    current_player: char,
    robot: Robot,
    is_robot_game: bool,
}

impl Game {
    pub fn new() -> Self {
        println!("Starting a new game of Tic Tac Toe!");
        Game {
            board: Board::new(),
            current_player: 'X',
            robot: Robot::new('O'),
            is_robot_game: true, // Set to true to play against robot
        }
    }

    // Main game loop
    pub fn play(&mut self) {
        loop {
            self.display_board();
            
            let move_successful = if self.is_robot_game && self.current_player == 'O' {
                // Robot's turn
                println!("Robot's turn (O)");
                self.get_robot_move()
            } else {
                // Human player's turn
                println!("Player {}'s turn", self.current_player);
                self.get_player_move()
            };
            
            if move_successful {
                // Check for winner after a successful move
                if let Some(winner) = self.board.check_winner() {
                    self.display_board();
                    if self.is_robot_game && winner == 'O' {
                        println!("Game Over! Robot wins!");
                    } else {
                        println!("Game Over! Player {} wins!", winner);
                    }
                    break;
                }
                
                // Check for draw after a successful move
                if self.board.is_full() {
                    self.display_board();
                    println!("Game Over! It's a draw!");
                    break;
                }
                self.switch_player();
            }
        }
    }

    pub fn display_board(&self) {
        println!();
        self.board.display();
        println!();
    }

    fn get_player_move(&mut self) -> bool {
        println!("Enter position (1-9): ");
        let position = self.get_position_input();

        if self.board.place_symbol_by_position(position, self.current_player) {
            println!("Move placed successfully!");
            true
        } else {
            println!("Invalid move! Position is either occupied or invalid. Try again.");
            false
        }
    }

    fn get_robot_move(&mut self) -> bool {
        if let Some(position) = self.robot.make_move(&mut self.board) {
            println!("Robot chose position {}", position);
            true
        } else {
            false
        }
    }

    fn get_position_input(&self) -> usize {
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            match input.trim().parse::<usize>() {
                Ok(num) if num >= 1 && num <= 9 => return num,
                _ => println!("Please enter a number between 1 and 9:"),
            }
        }
    }

    fn switch_player(&mut self) {
        self.current_player = if self.current_player == 'X' { 'O' } else { 'X' };
    }
}