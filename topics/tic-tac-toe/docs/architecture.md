# Tic-Tac-Toe AI Agent - Architecture

## Project Definition

A command-line tic-tac-toe game where a human player competes against an AI opponent using the minimax algorithm. The AI plays optimally and cannot be beaten.

### Goals

- Implement optimal AI using minimax algorithm
- Provide clean command-line interface
- Maintain modular, testable code structure

## Components and Modules

### Board Module (`src/board.rs`)

Manages game state using a 3x3 grid. Handles move validation, winner detection, and board display.

### AI Module (`src/ai.rs`)

Implements minimax algorithm for optimal move selection.

### Game Module (`src/game.rs`)

Coordinates gameplay flow, manages turns, and handles player interactions.

### Main Module (`src/main.rs`)

Provides command-line interface and user input handling.

## Usage

Build and run:

```bash
cargo run
```

Enter moves as coordinates (0-2): `1 2` for row 1, column 2. Type `quit` to exit.
