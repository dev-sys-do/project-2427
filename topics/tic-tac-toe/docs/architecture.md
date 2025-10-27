# Tic-Tac-Toe Architecture Documentation

## Project Definition

This is a command-line Tic-Tac-Toe game implementation written in Rust. It provides an interactive game experience where players can either compete against each other or play against a computer opponent on a classic 3x3 grid.

### Goals
The main goals of this project are:
- Provide an intuitive and user-friendly command-line interface for playing Tic-Tac-Toe
- Offer three game modes: player vs player, player vs random AI, and player vs intelligent AI
- Implement two AI opponents: one that plays randomly and one using the Minimax algorithm
- Demonstrate the Minimax algorithm for perfect play in a zero-sum game
- Implement game logic with proper validation and win detection

## Components and Modules

The project is structured into two main modules:

### 1. Game Module (`src/game.rs`)
This module contains the core game logic and data structures.

**Key Components:**

- **`Cell` enum**: Represents the state of a single cell on the board
  - `Empty`: Unoccupied cell
  - `X`: Cell occupied by player X
  - `O`: Cell occupied by player O

- **`TicTacToe` struct**: The main game state container
  - Stores the 3x3 board as a 2D array of `Cell` values
  - Encapsulates all game logic

**Methods:**

- `new()`: Creates a new game with an empty board
- `display()`: Renders the current board state to the console
- `place(row, col, player)`: Attempts to place a player's symbol at the specified position
- `is_full()`: Checks if the board is completely filled
- `check_winner()`: Analyzes the board to detect if a player has won
- `get_empty_cells()`: Returns a vector of all available (empty) cell coordinates
- `unplace(row, col)`: Removes a symbol from a cell (utility function)
- `evaluate()`: Evaluates the current board state and returns a score (+10 for O win, -10 for X win, 0 for draw)
- `minimax(is_maximizing)`: Recursive implementation of the Minimax algorithm to find optimal moves
- `find_best_move()`: Uses Minimax to determine the best possible move for the computer

### 2. Main Module (`src/main.rs`)
This module handles user interaction and game flow.

**Responsibilities:**

- Presents a menu to choose between three game modes (player vs player, player vs random AI, or player vs Minimax AI)
- Manages the game loop for all three modes
- Handles user input and validation
- Converts user coordinates (1-3) to internal array indices (0-2)
- Implements two computer opponent strategies: random move selection and Minimax algorithm
- Alternates between players (or player and computer)
- Displays game status and results

**Game Modes:**

- **Two Players Mode** (`play_two_players()`): Traditional gameplay where two human players alternate turns
- **Random Computer Mode** (`play_against_computer()`): Single player mode where the human plays as X and the computer plays as O, selecting random moves from available cells (easy difficulty)
- **Minimax Computer Mode** (`play_against_minimax()`): Single player mode where the computer uses the Minimax algorithm to play optimally, making it unbeatable (impossible difficulty)

### Architecture Justification

The separation between game logic (`game.rs`) and user interface (`main.rs`) follows the **separation of concerns** principle:

1. **Maintainability**: Game logic can be modified without affecting the UI, and vice versa
2. **Testability**: The game module can be unit tested independently
3. **Extensibility**: Easy to add new interfaces (GUI, web) or AI players without modifying core logic
4. **Reusability**: The game module can be used in different contexts

This modular approach makes the codebase clean, organized, and ready for future enhancements.

## The Minimax Algorithm

### Overview

The Minimax algorithm is a decision-making algorithm used in game theory and artificial intelligence. It's particularly effective for two-player zero-sum games like Tic-Tac-Toe, where one player's gain is the other player's loss.

### How It Works

The algorithm explores all possible game states recursively and assigns scores to terminal states (win, lose, or draw). It then propagates these scores back up the game tree to determine the best move.

**Scoring System:**
- +10: Computer (O) wins
- -10: Human player (X) wins
- 0: Draw or non-terminal state

**Algorithm Steps:**

1. **Base Cases**: If the game is over (win, lose, or draw), return the evaluation score
2. **Maximizing Player (Computer - O)**: Try all possible moves and choose the one with the highest score
3. **Minimizing Player (Human - X)**: Try all possible moves and choose the one with the lowest score
4. **Recursion**: For each possible move, simulate it and recursively call Minimax for the opponent's turn
5. **Backtracking**: After exploring a move, undo it and try the next one

### Implementation Details

The implementation consists of three key functions:

1. **`evaluate()`**: Evaluates the current board state
   - Returns +10 if O wins
   - Returns -10 if X wins
   - Returns 0 for draw or ongoing game

2. **`minimax(is_maximizing)`**: The core recursive algorithm
   - Takes a boolean indicating whether it's the maximizing player's turn
   - Explores all possible moves recursively
   - Returns the best score achievable from the current state

3. **`find_best_move()`**: Entry point for the AI
   - Tests all available moves
   - Calls `minimax()` for each move
   - Returns the move with the highest score

### Why It's Unbeatable

For a game as simple as Tic-Tac-Toe (3x3 grid with ~255,168 possible games), the Minimax algorithm can explore the entire game tree in milliseconds. This means:

- The computer always knows the outcome of every possible move sequence
- It will never make a mistake
- Best case for the human player: Draw (if they also play perfectly)
- If the human makes any mistake, the computer will exploit it and win

## Usage

### Prerequisites
- Rust toolchain installed (rustc, cargo)

### Running the Game

1. Navigate to the project directory:
```bash
cd topics/tic-tac-toe
```

2. Run the game:
```bash
cargo run
```

### How to Play

1. The game starts with a menu asking you to choose a game mode:
   - **Mode 1**: Two players (human vs human)
   - **Mode 2**: Against the computer with random moves (easy - beatable)
   - **Mode 3**: Against the computer with Minimax algorithm (impossible - unbeatable)
2. Enter `1`, `2`, or `3` to select your preferred mode
3. In two-player mode, players X and O alternate turns
4. In computer modes, you play as X and the computer plays as O
5. Enter coordinates as two numbers separated by a space: `row column`
6. Valid coordinates range from 1 to 3 for both row and column
7. In Mode 2, the computer selects moves randomly from available cells
8. In Mode 3, the computer uses Minimax to always play the optimal move

### Examples

**Grid Numbering:**
```
(1,1) | (1,2) | (1,3)
---------
(2,1) | (2,2) | (2,3)
---------
(3,1) | (3,2) | (3,3)
```

**Sample Game Session (Minimax Mode):**
```
=== Tic-Tac-Toe ===
Choisissez le mode de jeu :
1. Deux joueurs
2. Contre l'ordinateur (aléatoire)
3. Contre l'ordinateur (Minimax - imbattable)
> 3

=== Mode Contre l'Ordinateur (Minimax) ===
Vous êtes X, l'ordinateur est O
L'ordinateur utilise l'algorithme Minimax - il est imbattable !
Entrez les coordonnées comme : ligne colonne (1-3)
Exemple : 1 1 pour le coin supérieur gauche

.  .  .
---------
.  .  .
---------
.  .  .

Votre tour (X), entrez votre coup (ligne colonne) :
> 2 2

L'ordinateur calcule le meilleur coup (Minimax)...
L'ordinateur joue en (1, 1)

O  .  .
---------
.  X  .
---------
.  .  .

... (game continues)
```

### Common Commands

- **Place a move**: Enter `row column` (e.g., `1 1` for top-left corner)
- **Invalid moves**: The game will display an error message and allow you to try again

### Win Conditions

The game automatically detects when:
- A player gets three symbols in a row (horizontally, vertically, or diagonally)
- The board is full with no winner (draw)

### Example Winning Scenarios

**Horizontal Win:**
```
X  X  X     <- Player X wins
---------
O  O  .
---------
.  .  .
```

**Vertical Win:**
```
X  O  .
---------
X  O  .     <- Player O wins (middle column)
---------
X  .  .
```

**Diagonal Win:**
```
X  .  O
---------
.  X  O     <- Player X wins (top-left to bottom-right)
---------
O  .  X
```

## Dependencies

The project uses the following external dependencies:

- **`rand`** (version 0.8): Used for random number generation in the random AI opponent mode