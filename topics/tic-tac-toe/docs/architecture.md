# A Simple Tic-Tac-Toe Agent
Implemented by Pierre-Louis Leclerc

## Project Description

This project is a command-line Tic-Tac-Toe game implemented in Rust that features an intelligent AI opponent using the minimax algorithm. The game allows a human player to compete against a perfect-playing robot in a traditional 3x3 grid tic-tac-toe match.

### Goals
- **Perfect AI Implementation**: Create an unbeatable AI opponent using the minimax algorithm
- **Clean Architecture**: Demonstrate modular design with separation of concerns
- **Interactive Gameplay**: Provide an engaging command-line interface for human players
- **Educational Value**: Showcase game theory concepts and AI decision-making algorithms

## Components and Modules

The project is structured using a modular architecture with clear separation of responsibilities:

### Core Modules

#### 1. **Board Module** (`src/game/board.rs`)
- **Purpose**: Manages the game state and board operations
- **Key Features**:
  - 3x3 grid representation using `[[Option<char>; 3]; 3]`
  - Position-based input system (1-9 numbering)
  - Win condition detection (rows, columns, diagonals)
  - Draw detection (full board check)
  - Move validation and symbol placement
- **Justification**: Encapsulates all board-related logic, making it easy to test and maintain game state

#### 2. **Robot Module** (`src/game/robot.rs`)
- **Purpose**: Implements the AI opponent using minimax algorithm
- **Key Features**:
  - Perfect play strategy through minimax decision tree
  - Recursive game state evaluation
  - Optimal move selection with depth consideration
  - Strategic scoring system (wins, losses, draws)
- **Justification**: Separates AI logic from game flow, allowing for easy algorithm swapping or improvements

#### 3. **Game Module** (`src/game/game.rs`)
- **Purpose**: Orchestrates the overall game flow and user interaction
- **Key Features**:
  - Turn-based game loop management
  - Human vs Robot player coordination
  - Input handling and validation
  - Game state transitions and end conditions
- **Justification**: Acts as the controller, coordinating between board state and AI decisions

#### 4. **Main Module** (`src/main.rs`)
- **Purpose**: Entry point and game initialization
- **Key Features**:
  - Game instance creation
  - Application startup
- **Justification**: Keeps the entry point minimal and focused

### Module Interactions

```
main.rs
  ↓ creates
Game
  ↓ manages
Board ←→ Robot
  ↑        ↓
  └─ reads state
     makes moves
```

- **Game** coordinates between human input and robot decisions
- **Board** maintains authoritative game state for both players
- **Robot** analyzes board state to make optimal moves

## Usage

### Building and Running

```bash
# Navigate to the project directory
cd topics/tic-tac-toe

# Build the project
cargo build

# Run the game
cargo run
```