# Tic-Tac-Toe AI Agent - Architecture Documentation

## Project Definition

### What is it?

This project is a command-line Tic-Tac-Toe game where a human player competes against an AI opponent. The AI uses the Minimax algorithm to play optimally, ensuring it cannot be beaten - it will either win or draw every game.

### Goals

The primary goals of this project are:

1. **Unbeatable AI**: Implement an AI that plays optimally using the Minimax algorithm with depth-first search
2. **Interactive Gameplay**: Provide a user-friendly command-line interface for humans to play against the AI
3. **Clean Architecture**: Organize the code into well-defined, modular components
4. **Code Quality**: Follow Rust best practices with proper error handling, formatting, and comprehensive testing

## Components and Modules

The project is structured into five main modules, each with a specific responsibility:

### 1. `types.rs` - Core Type Definitions

**Purpose**: Defines the fundamental types used throughout the application.

**Key Components**:
- `Player` enum: Represents either Human (X) or AI (O) player
  - `opponent()`: Returns the opposing player
  - `symbol()`: Returns the character representation ('X' or 'O')
- `Cell` enum: Represents a board cell state (Empty or Occupied by a player)
  - `is_empty()`: Checks if the cell is available
  - `symbol()`: Returns the display character

**Rationale**: Separating type definitions provides a single source of truth for core concepts and enables type safety throughout the codebase.

### 2. `board.rs` - Game Board Representation

**Purpose**: Manages the 3x3 game board state and provides board manipulation operations.

**Key Components**:
- `Board` struct: Internally uses a 1D array of 9 cells for efficient storage
- Key methods:
  - `new()`: Creates an empty board
  - `make_move(position, player)`: Places a player's mark at a position
  - `available_moves()`: Returns all empty positions
  - `is_full()`: Checks if the board is completely filled
  - `display()`: Renders the board to the console
  - `get(position)`: Retrieves the cell state at a position
  - `cells()`: Provides access to the internal cell array

**Rationale**: Encapsulating board logic in a dedicated module ensures board operations are consistent and testable. Using a 1D array (index 0-8) simplifies indexing calculations compared to a 2D array.

### 3. `game.rs` - Game Logic and State Management

**Purpose**: Implements game rules, win detection, and state transitions.

**Key Components**:
- `GameState` enum: Tracks the current game status
  - `InProgress`: Game is ongoing
  - `Won(Player)`: A player has won
  - `Draw`: Game ended in a draw
- `Game` struct: Orchestrates the overall game flow
- Key methods:
  - `new()`: Initializes a new game with Human starting
  - `from_board(board, player)`: Creates a game from an existing board state (used by AI simulations)
  - `make_move(position)`: Executes a move and updates game state
  - `check_winner(player)`: Checks all win conditions (rows, columns, diagonals)
  - `evaluate()`: Returns a score for the current board state (+10 for AI win, -10 for Human win, 0 otherwise)
  - `update_state()`: Updates the game state after each move

**Rationale**: Centralizing game logic separates rules enforcement from board representation and AI logic. The `evaluate()` method provides a bridge between game state and the Minimax algorithm.

### 4. `ai.rs` - Minimax AI Implementation

**Purpose**: Implements the AI player using the Minimax algorithm.

**Key Components**:
- `AI` struct: Represents the AI player
- Key methods:
  - `find_best_move(game)`: Finds the optimal move for the current game state
  - `minimax(game, depth, is_maximizing)`: Recursive Minimax algorithm implementation
  - `simulate_move(game, position, player)`: Creates a hypothetical future game state
  - `create_game_from_board(board, player)`: Helper for game state creation

**Algorithm Details**:
- **Minimax with Depth Optimization**: The algorithm explores all possible future game states recursively
  - Maximizing player (AI): Chooses moves that maximize the score
  - Minimizing player (Human): Assumes the opponent plays optimally to minimize AI's score
  - Depth consideration: Prefers faster wins (score - depth) and slower losses (score + depth)
- **Terminal States**: 
  - AI wins: +10
  - Human wins: -10
  - Draw: 0

**Rationale**: The Minimax algorithm guarantees optimal play by exhaustively searching the game tree. Depth optimization ensures the AI prefers quicker victories. Separating AI logic into its own module allows for potential future AI strategy variations.

### 5. `main.rs` - User Interface and Game Loop

**Purpose**: Provides the command-line interface and coordinates the game flow.

**Key Components**:
- `main()`: Main game loop that alternates between human and AI turns
- `get_human_move(game)`: Handles user input with validation
- `display_position_guide()`: Shows position numbering (1-9)

**User Experience Features**:
- Clear visual position guide
- Input validation (1-9 range, position availability)
- Informative error messages
- Game result announcements with emojis
- AI thinking indicator

**Rationale**: Separating the UI from business logic makes the core game engine reusable and testable. The CLI provides an intuitive interface with helpful guidance for users.

## Module Interaction Flow

```
┌─────────────────────────────────────────────────────────┐
│                        main.rs                          │
│                   (User Interface)                      │
└────────────┬────────────────────────────┬───────────────┘
             │                            │
             ▼                            ▼
      ┌─────────────┐             ┌─────────────┐
      │   game.rs   │             │    ai.rs    │
      │ (Game Logic)│◄────────────┤ (AI Player) │
      └──────┬──────┘             └─────────────┘
             │
             ▼
      ┌─────────────┐
      │  board.rs   │
      │   (Board)   │
      └──────┬──────┘
             │
             ▼
      ┌─────────────┐
      │  types.rs   │
      │   (Types)   │
      └─────────────┘
```

**Data Flow**:
1. `main.rs` creates a `Game` instance and an `AI` instance
2. For human turns: `main.rs` gets input → validates → calls `game.make_move()`
3. For AI turns: `main.rs` calls `ai.find_best_move()` → AI explores game tree using `game.evaluate()` → returns best position → `main.rs` calls `game.make_move()`
4. `Game` updates the `Board` and checks win conditions
5. `main.rs` displays updated board and game state

## Architecture Rationale

### Modularity
Each module has a single, well-defined responsibility following the Single Responsibility Principle. This makes the code easier to understand, test, and maintain.

### Separation of Concerns
- **Presentation** (main.rs): User interaction
- **Business Logic** (game.rs): Game rules and state
- **Data Structures** (board.rs, types.rs): Core data representations
- **AI Strategy** (ai.rs): Decision-making algorithm

### Testability
The modular design enables comprehensive unit testing. Each module can be tested independently:
- Game logic tests verify win detection and state transitions
- AI tests verify blocking and winning move selection
- Board tests verify move validation and state management

### Type Safety
Rust's strong type system ensures correctness:
- Enums prevent invalid player or cell states
- The borrow checker prevents data races
- Pattern matching ensures all cases are handled

## Usage

### Building the Project

```bash
# Clone the repository
git clone <repository-url>
cd topics/tic-tac-toe

# Build the project
cargo build --release

# Run the game
cargo run --release
```

### Playing the Game

When you start the game, you'll see a position guide:

```
   1 | 2 | 3
  -----------
   4 | 5 | 6
  -----------
   7 | 8 | 9
```

Enter numbers 1-9 to place your mark (X) on the board. The AI (O) will respond after each move.

### Example Game Session

```
=================================
   Welcome to Tic-Tac-Toe!
=================================

You are X, AI is O
Enter positions 1-9 as shown:

   1 | 2 | 3
  -----------
   4 | 5 | 6
  -----------
   7 | 8 | 9


   |   |  
 -----------
   |   |  
 -----------
   |   |  

Your turn (X)
Enter position (1-9): 5

   |   |  
 -----------
   | X |  
 -----------
   |   |  

AI is thinking... 🤔
AI played position 1

   O |   |  
 -----------
   | X |  
 -----------
   |   |  

Your turn (X)
Enter position (1-9): 3
...
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_ai_blocks_winning_move
```

### Code Quality Checks

```bash
# Check formatting
cargo fmt --check

# Format code
cargo fmt

# Run linter
cargo clippy

# Build without warnings
cargo build --release
```

## Performance Considerations

### Minimax Optimization
- The game tree is relatively small for Tic-Tac-Toe (maximum 9! = 362,880 possible games)
- Depth-based scoring encourages faster wins, reducing average computation time
- Early terminal state detection prunes unnecessary branches

### Memory Efficiency
- Board uses a stack-allocated array instead of heap allocation
- Game state cloning for AI simulations is lightweight (72 bytes)
- No dynamic memory allocation during gameplay

## Future Enhancements

Potential improvements for future versions:

1. **Alpha-Beta Pruning**: Further optimize Minimax by skipping branches that cannot affect the final decision
2. **Difficulty Levels**: Add options for easier AI by limiting search depth or introducing randomness
3. **Undo/Redo**: Allow players to rewind moves
4. **Game History**: Save and replay past games
5. **GUI Version**: Create a graphical interface using a framework like `egui` or web-based UI
6. **Network Play**: Enable human vs human over network
7. **Different Board Sizes**: Generalize to NxN boards

## Conclusion

This Tic-Tac-Toe implementation demonstrates clean software architecture principles in Rust. The modular design separates concerns effectively, making the codebase maintainable and extensible. The Minimax algorithm ensures optimal AI play, providing a challenging opponent that cannot be beaten. The project showcases Rust's strengths in type safety, performance, and code quality enforcement through its tooling ecosystem.
