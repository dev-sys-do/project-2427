# Tic Tac Toe with Unbeatable AI

A command-line Tic Tac Toe game implemented in Rust, featuring an unbeatable AI using the Minimax algorithm.

## Project Structure

```
tic_tac_toe/
├── src/
│   ├── board.rs       # Game board representation and logic
│   ├── player/
│   │   ├── mod.rs     # Player trait definition
│   │   ├── human.rs   # Human player implementation
│   │   └── ai.rs      # AI player with Minimax algorithm
│   ├── game.rs        # Game state and flow management
│   ├── main.rs        # Entry point and user interface
│   └── lib.rs         # Library exports
├── tests/            # Unit tests
├── docs/
│   └── architecture_EN.md # Detailed architecture documentation (EN)       
└── README_EN.md      
```

## How to Run the Project

1. **Build**
   ```bash
   cargo build
   ```

2. **Run**
   ```bash
   cargo run
   ```

3. **Test**
   ```bash
   cargo test
   ```

## How to Play

1. Start the game with `cargo run`
2. Enter your name when prompted
3. The game randomly decides who goes first
4. On your turn, enter coordinates as "row column" (e.g., "1 1" for the center)
5. Indices range from 0 to 2, as shown on the board

## Main Features

- **Unbeatable AI** using the Minimax algorithm
- Command-line interface with Unicode display
- Modular and object-oriented architecture
- Comprehensive unit tests

## Documentation

For detailed documentation of the project architecture, see:
- English version: `docs/architecture_EN.md`
