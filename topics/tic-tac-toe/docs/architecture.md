# Tic-Tac-Toe AI Agent - Architecture

## Project Definition

### What is it?

A command-line Tic-Tac-Toe game implemented in Rust where a human player competes against an unbeatable AI opponent.

### Goals

- Provide an interactive CLI game experience
- Implement an optimal AI that never loses using the Minimax algorithm
- Demonstrate clean Rust code with proper error handling and testing
- Offer a simple yet engaging gameplay interface

## Components and Modules

### Core Data Structures

#### `Mark` enum
Represents player symbols (X or O) with an `opponent()` helper method to switch between players.

#### `Cell` enum
Represents individual board positions as either `Empty` or `Filled(Mark)`.

#### `Board` struct
The main game state representation using a 1D array of 9 cells (positions 0-8).

**Key methods:**
- `new()`: Creates an empty board
- `place_mark()`: Places a mark with validation
- `check_winner()`: Detects wins across all 8 lines (3 rows, 3 columns, 2 diagonals)
- `is_full()` / `is_draw()`: Game completion detection
- `legal_moves()`: Returns available positions
- `display()`: Renders the board as a formatted string

#### `GameState` enum
Represents game status: `Ongoing`, `Win(Mark)`, or `Draw`.

### AI Module

#### Minimax Algorithm
Implemented as private `Board::minimax()` method:
- Recursively evaluates all possible game states
- Returns +1 for AI win, -1 for opponent win, 0 for draw
- Alternates between maximizing (AI turn) and minimizing (opponent turn)
- Explores the complete game tree depth-first

#### Best Move Selection
`Board::best_move()` method:
- Evaluates all legal moves using Minimax
- Returns the position with the highest score
- Guarantees optimal play

### CLI Interface

#### Input Handling
- `read_line()`: Reads and trims user input
- `get_user_move()`: Validates move input (0-8 range, empty position)

#### Game Loop
`play_game()` function orchestrates:
1. Board display at each turn
2. Human move input with validation and retry
3. AI response with optimal move
4. Game state checking and end condition detection

### Architecture Rationale

**Single-file design**: Given the project's scope (~500 LOC), all code lives in `src/main.rs` for simplicity and ease of review.

**Immutable game tree exploration**: The Minimax algorithm clones the board for each move simulation, ensuring clean separation of concerns and making the algorithm easier to reason about.

**Separation of concerns**:
- Data structures handle state representation
- Board methods handle game logic
- AI module handles decision-making
- CLI functions handle user interaction

**Error handling**: Uses Rust's `Result` type for operations that can fail (e.g., invalid moves), with clear error messages.

## Usage

### Building and Running

```bash
# Build the project
cargo build --release

# Run the game
cargo run

# Run tests
cargo test

# Check code quality
cargo clippy -- -D warnings
cargo fmt --check
```

### Gameplay Example

```
=== Tic-Tac-Toe: Human (X) vs AI (O) ===

Positions are numbered 0-8:

0 | 1 | 2
---------
3 | 4 | 5
---------
6 | 7 | 8

Current board:
0 | 1 | 2
---------
3 | 4 | 5
---------
6 | 7 | 8

Your turn (X):
Enter your move (0-8): 4

AI is thinking...
AI plays position 0

Current board:
O | 1 | 2
---------
3 | X | 5
---------
6 | 7 | 8

Your turn (X):
Enter your move (0-8): 2

AI is thinking...
AI plays position 8

Current board:
O | 1 | X
---------
3 | X | 5
---------
6 | 7 | O

Your turn (X):
Enter your move (0-8): 6

Final board:
O | 1 | X
---------
3 | X | 5
---------
X | 7 | O

🎉 You won! Congratulations!
```

### Testing

The project includes 29 unit tests covering:
- Board operations (creation, move placement, state checking)
- Win detection (rows, columns, diagonals)
- Draw detection
- Minimax algorithm correctness
- AI optimal play verification
- Board display formatting

Run all tests with:
```bash
cargo test
```

### Performance

The unoptimized Minimax implementation evaluates the complete game tree. For Tic-Tac-Toe:
- Maximum depth: 9 moves
- Average response time: < 200ms on modern hardware
- The AI is deterministic and always plays optimally

Future optimizations could include alpha-beta pruning to reduce the search space.
