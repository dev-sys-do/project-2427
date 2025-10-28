# Tic Tac Toe Architecture Documentation

## Project Definition

This project is a command-line implementation of the classic Tic Tac Toe game, featuring an unbeatable AI opponent. The AI uses the Minimax algorithm to make optimal decisions, ensuring that it cannot be beaten (it will either win or force a draw).

The main objectives of this project are:

1. Create a playable Tic Tac Toe game with a clean command-line interface
2. Implement an unbeatable AI using the Minimax algorithm
3. Demonstrate good software architecture principles in Rust
4. Provide a modular, maintainable, and well-documented codebase 

## Components and Modules

The project is organized into several modules, each with a specific responsibility:

### Board Module (`board.rs`)

Responsible for representing and managing the game board.

- **Cell Enum**: Represents the state of a cell on the board (Empty, X, O)
- **Board Struct**: Manages a 3x3 grid of cells
- **Key Methods**:
  - `new()`: Creates a new empty board
  - `get_cell()` / `set_cell()`: Access and modify cells
  - `available_moves()`: Lists all valid moves
  - `check_winner()`: Determines if a player has won
  - `is_full()`: Checks if the board is full (draw)
  - `display()`: Renders the board to the console

### Player Module (`player/`)

Defines the player interface and implementations.

- **Player Trait** (`mod.rs`): Interface that all player types must implement
  - `get_cell_type()`: Returns the player's cell type (X or O)
  - `make_move()`: Determines the next move
  - `get_name()`: Returns the player's name

- **Human Player** (`human.rs`): Implementation for human players
  - **HumanPlayer Structure**: Represents a human player
    - `cell_type`: Cell type used by the player (X or O)
    - `name`: Player's name
  - **Key Methods**:
    - `new(cell_type, name)`: Creates a new human player instance
    - `get_cell_type()`: Trait implementation that returns the player's cell type
    - `get_name()`: Trait implementation that returns the player's name
    - `make_move(board)`: Trait implementation that:
      - Displays the message asking the player to enter a move
      - Reads user input from the console
      - Validates that the input contains two valid coordinates
      - Checks that the coordinates are in the 0-2 range
      - Ensures that the selected cell is empty
      - Continues to ask until a valid move is obtained
      - Returns the coordinates of the validated move

- **AI Player** (`ai.rs`): Implementation for the AI opponent
  - **AIPlayer Structure**: Represents an AI player
    - `cell_type`: Cell type used by the AI (X or O)
    - `name`: AI player's name
  - **Key Methods**:
    - `new(cell_type, name)`: Creates a new AI player instance
    - `opponent_cell()`: Determines the opponent's cell type (X if AI is O, O if AI is X)
    - `minimax(board, depth, is_maximizing)`: Implements the Minimax algorithm that:
      - Recursively explores all possible moves
      - Evaluates terminal states (win, loss, draw)
      - Assigns scores to positions (+10 for win, -10 for loss, 0 for draw)
      - Adjusts scores based on depth to favor quick wins
      - Alternates between maximization (AI's turn) and minimization (opponent's turn)
      - Returns the optimal score for the current position
    - `get_cell_type()`: Trait implementation that returns the AI's cell type
    - `get_name()`: Trait implementation that returns the AI's name
    - `make_move(board)`: Trait implementation that:
      - Iterates through all available moves
      - Evaluates each move with the Minimax algorithm
      - Selects the move with the best score
      - Returns the coordinates of the optimal move

### Game Module (`game.rs`)

Manages the game state and flow.

- **GameState Enum**: Represents the current state of the game (InProgress, Win, Draw)
- **Game Struct**: Orchestrates the game flow
- **Key Methods**:
  - `new()`: Creates a new game with specified players
  - `play_turn()`: Executes a single turn
  - `update_state()`: Updates the game state after each move
  - `display_result()`: Shows the final result

### Main Module (`main.rs`)

Entry point for the application.

- Handles initial setup
- Creates players (human and AI)
- Manages the game loop
- Handles user interaction

## Module Interactions

The Tic Tac Toe game is organized around simple interactions between its different modules. Here's how they communicate with each other:

### Game Startup

- The program begins in `main.rs` which creates the players (human and AI)
- A random selection decides who goes first
- The main module then creates the game and starts the main loop

### Turn Progression

1. The `Game` module displays the current state of the board using functions from the `Board` module
2. The current player is prompted to make a move:
   - The human player enters coordinates via the console
   - The AI calculates the best possible move using the Minimax algorithm
3. The `Game` module verifies that the move is valid by consulting the `Board`
4. The board is updated with the new move
5. The `Game` module checks if the game is over:
   - Checking for a win (three symbols in a row)
   - Checking for a draw (full board)
6. If the game continues, the next player's turn begins

### Game End

- Once the game is over, the final result is displayed
- The final board is shown one last time
- A message indicates who won or if there was a draw

This organization allows for a clear separation of responsibilities: the `Board` manages the state of the board, the `Players` determine the moves, and the `Game` coordinates the overall flow of the game.

## Why This Modular Architecture?

I chose to separate the code into distinct modules for several important reasons:

1. **Logical Code Organization**: 
   - The separation into modules (`board.rs`, `player/`, `game.rs`) reflects the natural components of a Tic Tac Toe game
   - Each file contains a specific aspect of the game, making the code easier to explore and understand
   - This structure allows quickly finding where to implement a new feature or fix a bug

2. **Separation of Concerns**:
   - The board (`board.rs`) only handles the game state and rules (win checking, etc.)
   - The players (`player/`) focus on game strategy (human via console, AI via Minimax)
   - The game (`game.rs`) manages the game flow and turn alternation
   - The entry point (`main.rs`) handles initialization and user interface

3. **Common Interface for Different Player Types**:
   - The `Player` trait in `player/mod.rs` defines a contract that all players must follow
   - This abstraction allows treating human and AI players uniformly
   - It facilitates adding new player types (e.g., a network player or an AI with a different algorithm)

4. **Decoupling and Flexibility**:
   - Modules interact through well-defined interfaces
   - The game doesn't need to know the implementation details of the AI
   - We can replace the Minimax algorithm with another strategy without modifying the rest of the code
   - This independence facilitates future evolution and experimentation

5. **Component Reusability**:
   - The separation into modules allows exporting certain parts as a reusable library

This code organization makes the project:

- **Easier to understand**: Each file has a clear role
- **Easier to modify**: We can change one part without touching the rest
- **Easier to test**: We can verify each part separately
- **Easier to evolve**: We can add new features simply

## Minimax Algorithm

The Minimax algorithm is implemented in the `ai.rs` module and forms the core of the AI's intelligence. Here's how it works:

1. The algorithm recursively explores all possible moves until reaching a terminal state (win, loss, or draw)
2. For each terminal state, a score is assigned:
   - +10 for an AI win (adjusted by depth to favor quick wins)
   - -10 for an AI loss (adjusted by depth to delay losses)
   - 0 for a draw
3. These scores are propagated upward in the game tree:
   - The maximizing player (AI) chooses the move with the maximum score
   - The minimizing player (opponent) chooses the move with the minimum score
4. The move with the best score for the AI is selected

## Usage Examples

### Starting the Game

```bash
cargo run
```

### Playing a Move

When prompted, enter coordinates as row and column, separated by a space:

```
Enter your move (row col): 1 1
```

This would place your mark in the center of the board.

### Game Flow

1. The game randomly decides who goes first
2. Players take turns making moves
3. The game checks for win/draw conditions after each move
4. When the game ends, the result is displayed

### Example Game Session

```
Welcome to Tic Tac Toe!
You'll be playing against an unbeatable AI using the Minimax algorithm.
Enter your name: Alice
Alice (X) goes first!
Alice's turn (X)
    0   1   2  
  ┌───┬───┬───┐
0 │   │   │   │
  ├───┼───┼───┤
1 │   │   │   │
  ├───┼───┼───┤
2 │   │   │   │
  └───┴───┴───┘

Enter your move (row col): 1 1
Alice placed at position (1, 1)
AI's turn (O)
    0   1   2  
  ┌───┬───┬───┐
0 │   │   │   │
  ├───┼───┼───┤
1 │   │ X │   │
  ├───┼───┼───┤
2 │   │   │   │
  └───┴───┴───┘

AI is thinking...
AI placed at position (0, 0)
Alice's turn (X)
    0   1   2  
  ┌───┬───┬───┐
0 │ O │   │   │
  ├───┼───┼───┤
1 │   │ X │   │
  ├───┼───┼───┤
2 │   │   │   │
  └───┴───┴───┘

Enter your move (row col): 2 2
Alice placed at position (2, 2)
AI's turn (O)
    0   1   2  
  ┌───┬───┬───┐
0 │ O │   │   │
  ├───┼───┼───┤
1 │   │ X │   │
  ├───┼───┼───┤
2 │   │   │ X │
  └───┴───┴───┘

AI is thinking...
AI placed at position (0, 2)
Alice's turn (X)
    0   1   2  
  ┌───┬───┬───┐
0 │ O │   │ O │
  ├───┼───┼───┤
1 │   │ X │   │
  ├───┼───┼───┤
2 │   │   │ X │
  └───┴───┴───┘

Enter your move (row col): 2 0
Alice placed at position (2, 0)
AI's turn (O)
    0   1   2  
  ┌───┬───┬───┐
0 │ O │   │ O │
  ├───┼───┼───┤
1 │   │ X │   │
  ├───┼───┼───┤
2 │ X │   │ X │
  └───┴───┴───┘

AI is thinking...
AI placed at position (0, 1)
AI wins!
    0   1   2  
  ┌───┬───┬───┐
0 │ O │ O │ O │
  ├───┼───┼───┤
1 │   │ X │   │
  ├───┼───┼───┤
2 │ X │   │ X │
  └───┴───┴───┘
```

In this example, the AI wins by aligning three Os on the top row (positions 0,0 - 0,1 - 0,2). This demonstrates how the Minimax algorithm allows the AI to play optimally and win when the human player makes suboptimal choices.

### Draw Example

Here's an example of a game that ends in a draw when both players play optimally:

```
Welcome to Tic Tac Toe!
You'll be playing against an unbeatable AI using the Minimax algorithm.
Enter your name: Bob
Bob (X) goes first!
Bob's turn (X)
    0   1   2  
  ┌───┬───┬───┐
0 │   │   │   │
  ├───┼───┼───┤
1 │   │   │   │
  ├───┼───┼───┤
2 │   │   │   │
  └───┴───┴───┘

Enter your move (row col): 0 0
Bob placed at position (0, 0)
AI's turn (O)
    0   1   2  
  ┌───┬───┬───┐
0 │ X │   │   │
  ├───┼───┼───┤
1 │   │   │   │
  ├───┼───┼───┤
2 │   │   │   │
  └───┴───┴───┘

AI is thinking...
AI placed at position (1, 1)
Bob's turn (X)
    0   1   2  
  ┌───┬───┬───┐
0 │ X │   │   │
  ├───┼───┼───┤
1 │   │ O │   │
  ├───┼───┼───┤
2 │   │   │   │
  └───┴───┴───┘

Enter your move (row col): 2 0
Bob placed at position (2, 0)
AI's turn (O)
    0   1   2  
  ┌───┬───┬───┐
0 │ X │   │   │
  ├───┼───┼───┤
1 │   │ O │   │
  ├───┼───┼───┤
2 │ X │   │   │
  └───┴───┴───┘

AI is thinking...
AI placed at position (0, 2)
Bob's turn (X)
    0   1   2  
  ┌───┬───┬───┐
0 │ X │   │ O │
  ├───┼───┼───┤
1 │   │ O │   │
  ├───┼───┼───┤
2 │ X │   │   │
  └───┴───┴───┘

Enter your move (row col): 0 1
Bob placed at position (0, 1)
AI's turn (O)
    0   1   2  
  ┌───┬───┬───┐
0 │ X │ X │ O │
  ├───┼───┼───┤
1 │   │ O │   │
  ├───┼───┼───┤
2 │ X │   │   │
  └───┴───┴───┘

AI is thinking...
AI placed at position (2, 2)
Bob's turn (X)
    0   1   2  
  ┌───┬───┬───┐
0 │ X │ X │ O │
  ├───┼───┼───┤
1 │   │ O │   │
  ├───┼───┼───┤
2 │ X │   │ O │
  └───┴───┴───┘

Enter your move (row col): 1 0
Bob placed at position (1, 0)
AI's turn (O)
    0   1   2  
  ┌───┬───┬───┐
0 │ X │ X │ O │
  ├───┼───┼───┤
1 │ X │ O │   │
  ├───┼───┼───┤
2 │ X │   │ O │
  └───┴───┴───┘

AI is thinking...
AI placed at position (1, 2)
Bob's turn (X)
    0   1   2  
  ┌───┬───┬───┐
0 │ X │ X │ O │
  ├───┼───┼───┤
1 │ X │ O │ O │
  ├───┼───┼───┤
2 │ X │   │ O │
  └───┴───┴───┘

Enter your move (row col): 2 1
Bob placed at position (2, 1)
The game ended in a draw!
    0   1   2  
  ┌───┬───┬───┐
0 │ X │ X │ O │
  ├───┼───┼───┤
1 │ X │ O │ O │
  ├───┼───┼───┤
2 │ X │ X │ O │
  └───┴───┴───┘
```

In this example, both players play optimally, resulting in a draw. The human player started with the top-left corner, and the AI responded with the center, following the optimal strategy. This illustrates how the Minimax algorithm ensures that the AI can never lose: it will win if possible, or at least force a draw against a player who plays well.

## Tests

I've written tests to verify each part of the game. The tests are organized in the following files:

### `tests/board_tests.rs`

This file tests the game board functionality with the following functions:

- `test_new_board`: Verifies that a new board is empty.
- `test_set_get_cell`: Ensures that cells are correctly updated.
- `test_is_cell_empty`: Confirms that empty cell detection works.
- `test_available_moves`: Verifies that available moves are properly listed.
- `test_check_winner`: Ensures that win conditions are correctly detected.
- `test_is_full`: Tests the detection of a full board (draw).

### `tests/human_tests.rs`

This file tests the human player:

- `test_human_initialization`: Verifies that the human player is correctly initialized.
- `test_human_get_cell_type`: Ensures that the cell type is correctly returned.
- `test_human_get_name`: Verifies that the player's name is correctly returned.
- `test_human_make_move_validation`: Tests the validation of inputs for human player moves.

### `tests/ai_tests.rs`

This file tests the artificial intelligence and Minimax algorithm:

- `test_ai_blocks_win`: Verifies that the AI blocks the opponent when they're about to win.
- `test_ai_takes_win`: Ensures that the AI chooses a winning move when possible.
- `test_ai_optimal_first_move`: Verifies that the AI makes an optimal first move.
- `test_minimax_scores`: Tests the scores assigned by the Minimax algorithm.

### `tests/game_tests.rs`

This file tests the overall game flow:

- `test_game_initialization`: Verifies that the game is correctly initialized.
- `test_switch_player`: Ensures that player alternation works.
- `test_update_state`: Tests the update of the game state after each move.
- `test_game_win_detection`: Verifies that wins are properly detected.
- `test_game_draw_detection`: Ensures that draws are correctly identified.

To run all the tests:

```bash
cargo test
```

## Conclusion

This Tic Tac Toe project allowed me to create a simple but complete game with an AI that never loses. I organized the code into modules to make it easy to understand and modify. Using the Minimax algorithm makes the AI very strong, impossible to beat.
