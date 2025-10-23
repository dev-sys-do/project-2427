# Tic-Tac-Toe AI Agent - Architecture Document

## Project Definition

This project implements a command-line Tic-Tac-Toe game where a human player competes against an AI opponent. The AI uses the Minimax algorithm with depth-first search to play optimally, ensuring it never loses and will either win or draw every game.

### Goals
- Create an interactive command-line Tic-Tac-Toe game
- Implement an unbeatable AI using the Minimax algorithm
- Provide a clean, modular architecture with clear separation of concerns
- Ensure the code is well-documented, tested, and follows Rust best practices
- Build a system that can be easily extended or modified

## Components and Modules

The project is architected using a layered, modular approach with clear separation of responsibilities:

### Core Modules

#### 1. **Board Module (`board.rs`)**
**Responsibility**: Physical game state representation and basic operations
- Represents the 3x3 game board as a 1D array for efficiency
- Manages cell states (Empty, Occupied by Player X/O)
- Handles move placement and position validation
- Provides board querying methods (empty positions, full board check)
- Implements clean display formatting

#### 2. **Game Module (`game.rs`)**
**Responsibility**: Game logic, rules, and state management
- Manages overall game state (InProgress, Won, Draw)
- Handles player turns and move validation
- Implements win condition detection using pattern matching
- Provides game flow control (reset, state transitions)
- Acts as the central coordinator between other modules

#### 3. **AI Module (`ai.rs`)**
**Responsibility**: Intelligent opponent using Minimax algorithm
- Implements the Minimax algorithm with depth-first search
- Evaluates all possible game states recursively
- Scores positions: +10 for AI win, -10 for AI loss, 0 for draw
- Optimizes for winning quickly and losing slowly
- Provides unbeatable gameplay that never loses

#### 4. **UI Module (`ui.rs`)**
**Responsibility**: User interaction and interface management
- Handles command-line input/output operations
- Manages game flow (start, play rounds, restart)
- Provides user-friendly error messages and feedback
- Displays board state and position guides
- Coordinates between human input and AI responses

#### 5. **Main Application (`main.rs`)**
**Responsibility**: Entry point and module coordination
- Orchestrates all modules
- Initializes the game system
- Provides the main execution flow

### Architecture Justification

This modular design follows several key principles:

**1. Single Responsibility Principle**: Each module has one clear purpose
- Board: Data representation
- Game: Business logic
- AI: Intelligence algorithms  
- UI: User interaction

**2. Separation of Concerns**: Clear boundaries between layers
- Data layer (Board) is independent of game rules
- Game logic is separate from AI implementation
- UI is decoupled from core game mechanics

**3. Dependency Direction**: Clean dependency flow
```
main.rs → ui.rs → game.rs → board.rs
       → ai.rs ↗
```

**4. Testability**: Each module can be unit tested independently
- 11 tests for Board operations
- 14 tests for Game logic  
- 6 tests for AI behavior
- 4 integration tests for UI

**5. Extensibility**: Easy to modify or extend
- AI algorithm can be swapped without affecting other modules
- UI can be replaced (web, GUI) without changing core logic
- Game rules can be modified independently

## Usage

### Building and Running

```bash
# Navigate to project directory
cd topics/tic-tac-toe

# Build the project
cargo build

# Run the game
cargo run

# Run tests
cargo test
```

### Gameplay Experience

#### 1. **Game Start**
The game displays a welcome message and position guide:
```
🎮 Welcome to Tic-Tac-Toe! 🎮
You are X, AI is O.
Enter positions 1-9 corresponding to board positions:

Board positions:
 1 | 2 | 3 
-----------
 4 | 5 | 6 
-----------
 7 | 8 | 9 
```

#### 2. **Gameplay Flow**
- Human player (X) always goes first
- Enter positions 1-9 to place your mark
- AI automatically responds with optimal moves
- Game displays board state after each move
- Clear feedback for invalid moves or occupied positions

#### 3. **Example Game Session**
```
🆕 Starting new game!

   |   |   
-----------
   |   |   
-----------
   |   |   

Your move (1-9): 5

   |   |   
-----------
   | X |   
-----------
   |   |   

🤖 AI is thinking...
🤖 AI plays position 1

 O |   |   
-----------
   | X |   
-----------
   |   |   

Your move (1-9): 9
```

#### 4. **Game Results**
The game automatically detects and displays results:
- **Human Win**: "🎉 Congratulations! Player X wins! 🎉"
- **AI Win**: "🎉 Congratulations! Player O wins! 🎉"  
- **Draw**: "🤝 It's a draw! Well played both players! 🤝"

#### 5. **Replay Option**
After each game, players can choose to play again:
```
🔄 Would you like to play again? (y/n): y
```

### AI Behavior

The AI implements optimal Minimax strategy:
- **Defensive**: Automatically blocks human winning moves
- **Offensive**: Takes immediate winning opportunities
- **Strategic**: Chooses moves that maximize long-term advantage
- **Unbeatable**: Mathematical guarantee of never losing

### Development and Testing

#### Running Tests
```bash
# Run all tests (35 total)
cargo test

# Run specific module tests
cargo test board::tests
cargo test game::tests  
cargo test ai::tests
cargo test ui::tests

# Run with verbose output
cargo test -- --nocapture
```

#### Code Quality
```bash
# Check for compilation issues
cargo check

# Format code
cargo fmt

# Run clippy for additional lints
cargo clippy
```

### Technical Specifications

- **Language**: Rust (Edition 2024)
- **Dependencies**: Standard library only
- **Board Representation**: 1D array of 9 positions
- **AI Algorithm**: Minimax with depth-first search
- **Input Validation**: Comprehensive error handling
- **Test Coverage**: 35 unit and integration tests
- **Performance**: Instant AI response for all game states

This architecture provides a robust, maintainable, and extensible foundation for the Tic-Tac-Toe game while demonstrating clean code principles and effective modular design.