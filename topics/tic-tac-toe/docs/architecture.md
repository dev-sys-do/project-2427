# Tic-Tac-Toe AI Agent - Architecture Documentation

## 1. Project Definition

### Overview

A command-line tic-tac-toe game featuring an unbeatable AI opponent powered by the Minimax algorithm. The AI plays optimally, ensuring it never loses—the best outcome for a human player is a draw.

### Goals

1. **Optimal AI**: Minimax algorithm for mathematically perfect play
2. **Clean Architecture**: Modular design with clear separation of concerns  
3. **Quality Code**: Zero warnings, comprehensive testing, proper documentation
4. **User Experience**: Interactive terminal interface with visual board representation

### Technical Stack

- **Language**: Rust
- **Game Logic**: 3×3 grid, 1D array representation  
- **AI**: Minimax with depth-based scoring
- **Interface**: CLI with Unicode box-drawing
- **Testing**: Comprehensive unit and integration tests

## 2. Components and Modules

### Architecture Overview

Layered architecture with clear separation between data, logic, AI, and presentation:

```
┌─────────────────────────────────────────────────┐
│                   main.rs                       │
│           (Entry Point & CLI Wiring)            │
└──────────┬──────────────────────────────────────┘
           │
           ├──────────┬──────────┬──────────┐
           ▼          ▼          ▼          ▼
    ┌──────────┐ ┌────────┐ ┌────────┐ ┌──────────┐
    │  board/  │ │ game.rs│ │  ai/   │ │ render/  │
    │          │ │        │ │        │ │          │
    │ - mod.rs │ │ Rules  │ │minimax │ │terminal  │
    │ - lines  │ │ Win    │ │ Best   │ │ Display  │
    │          │ │ Check  │ │ Move   │ │ Input    │
    └──────────┘ └────────┘ └────────┘ └──────────┘
         ▲            ▲          ▲           │
         │            │          │           │
         └────────────┴──────────┴───────────┘
                  Dependencies
```

### Module Descriptions

#### board/ - Core Data Structures

**Purpose**: Game state representation

**Components**:
- `mod.rs`: Core types (`Player`, `Cell`, `Board`)
- `lines.rs`: 8 winning combinations (3 rows, 3 cols, 2 diagonals)

The board is represented as a fixed-size 1D array of 9 cells, where each cell can be empty or marked by a player (X/O).

**Rationale**:
- **1D Array**: Simpler indexing, better cache locality than 2D
- **Copy Semantics**: Efficient cloning for Minimax tree exploration
- **Type Safety**: Enums prevent invalid states

#### game.rs - Game Rules

**Purpose**: Win detection and state management

**Components**:
- `Outcome` enum: `InProgress | Win(Player) | Draw`
- `check_game_state()`: Evaluates board against 8 winning lines
- Pure functions with no side effects

**Algorithm**: O(1) check of 8 pre-defined winning patterns

#### ai/ - Minimax Algorithm

**Purpose**: Optimal move selection

**Implementation**: Classic Minimax with depth-first search

```
Minimax Recursion:
  If terminal → return score
  If maximizing (AI):
      For each move: score = max(scores)
  Else (Human):
      For each move: score = min(scores)
```

**Scoring**:
- Win: `+10 - depth` (prefer faster wins)
- Loss: `depth - 10` (prefer slower losses)
- Draw: `0`

**Complexity**: The search explores a small, finite game tree. For tic-tac-toe, the AI responds near-instantly.

**Rationale**:
- State space is small enough for complete exploration
- Guarantees optimal play (AI never loses)
- Depth adjustment ensures strategic preference for shorter paths
- Alpha-beta pruning omitted (acceptable given performance); can be added as future work

#### render/ - Terminal Interface

**Purpose**: User interaction completely decoupled from logic

**Components**:
- Board display with Unicode box-drawing (╔══╗)
- Input validation and error handling
- Screen management (clear, animations)

**Rationale**:
- Easy to add alternative interfaces (web, GUI)
- Testable in isolation
- No game logic mixed with presentation

### Module Interactions

**Game Turn Flow**:
```
1. Display board (render)
2. Check win condition (game)
3. Get human move (render → board)
4. Check win condition (game)
5. Calculate AI move (ai → board)
6. Loop to step 1
```

### Design Justifications

**Why This Architecture?**

1. **Testability**: Each module tested independently
   - 10 tests for board operations
   - 12 tests for game rules (all win conditions)
   - 10 tests for AI strategy

2. **Maintainability**: Clear boundaries enable isolated changes
   - Add alpha-beta pruning? Only touch `ai/minimax.rs`
   - Add web UI? Create new renderer, reuse core logic

3. **Reusability**: Core logic (board + game) is interface-agnostic

4. **Rust Best Practices**:
   - Small, focused modules (~100-200 lines)
   - Public API clearly defined in `lib.rs`
   - Private implementation hidden

**Why 1D Array?**
- Simpler user input mapping (1-9 positions)
- Better cache locality
- Natural indexing

**Why Separate `lines.rs`?**
- Configuration data separated from logic
- Easy to extend for different board sizes
- Keeps `board/mod.rs` focused

## 3. Usage

```bash
cargo run --release    # Build and run
cargo test             # Run all tests
cargo clippy           # Check code quality
```

The game presents a 3×3 grid with numbered positions (1-9). Players enter their move, and the AI responds immediately with its optimal counter-move.

## 4. Testing Strategy

Each logical layer has its own unit tests:
- **board** – verifies win detection, legal moves, immutability
- **game** – ensures correct transitions between InProgress/Win/Draw
- **ai** – validates that the AI never loses and blocks human threats

Integration tests combine these modules through full game scenarios.

## 5. Quality & PR Standards

### Quality Gates

- `cargo fmt --all --check` — consistent formatting  
- `cargo clippy --all-targets -- -D warnings` — zero lint warnings  
- `cargo test --all` — full suite passing

### Contribution & PR Standards

The project follows Git best practices for submission:
- **Commit Messages**: Imperative mood, clear description of changes
- **Linear History**: No merge commits (rebase workflow)
- **Logical Narrative**: Commits represent coherent development steps
- **PR Structure**: Single pull request with all deliverables under `topics/tic-tac-toe/`

### Implementation Notes

**Rust Features Leveraged**:
- Enums with data for type-safe game state
- Pattern matching for clean logic flow
- Copy trait for efficient Minimax cloning
- Iterators for lazy evaluation
- Zero-cost abstractions (no runtime overhead)

**Performance**: AI performs near-instantly for all moves

## 6. Limitations and Future Work

### Current Limitations

1. **No Alpha-Beta Pruning**: Full tree exploration (acceptable for tic-tac-toe size)
2. **No Transposition Tables**: Repeated states re-evaluated
3. **Single-threaded**: Could parallelize top-level move exploration

### Potential Enhancements

1. **Alpha-Beta Pruning**: Reduce explored nodes by ~50%
2. **Move Ordering**: Evaluate center/corners first for better pruning
3. **Transposition Tables**: Cache evaluated positions
4. **Difficulty Levels**: Add intentionally sub-optimal modes
5. **Game History**: Move undo and replay functionality
6. **Statistics**: Track performance across multiple games

### Architectural Extensibility

The modular design makes future enhancements straightforward:
- **New AI algorithms**: Add to `ai/` alongside Minimax
- **Alternative interfaces**: Add to `render/` (web, TUI, GUI)
- **Different game variants**: Extend `board/` and `game.rs`

All while preserving the core architecture and existing tests.

---

**Project Repository Structure**:
```
tic-tac-toe/
├── Cargo.toml
├── clippy.toml         # Strict linting configuration
├── src/
│   ├── lib.rs          # Public API surface
│   ├── main.rs         # Entry point
│   ├── game.rs         # Rules
│   ├── board/          # Game state
│   ├── ai/             # Minimax
│   └── render/         # Terminal UI
├── tests/              # Integration tests
└── docs/
    └── architecture.md # This document
```

This project illustrates how sound software engineering principles, modularity, testability, and separation of concerns can be applied to a classic AI problem within a concise Rust implementation.
