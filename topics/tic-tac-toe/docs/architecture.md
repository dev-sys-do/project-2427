# Architecture

## File and folder structure
- `src/logic/`: Logic code of tic-tac-toe. It contains the player iteration logic in `game.rs`, and tic-tac-toe grid logic in `grid.rs`
- `src/player`: Different implementations of players that can play the tic-tac-toe game. Currently, it contains a terminal player, which will be controlled by a player by their terminal, and an AI player, using the MinMax algorithm.
- `src/types.rs`: Contains pure data types used by the library 

# Objects
The main objects that will interact together in this library are the `Game` object, and `PlayerBehavior` objects. a Game will call methods on PlayerBehaviors, which simulate method. PlayerBehaviors may act on these methods to run diverse actions (e.g. printing information to the terminal), or communicate back to the `Game` (e.g. `play()` return value).

An interesting property of Rust I've tried to use here is passing-by-movement. You will notice that the `Game` constructor takes ownership of players, and that `play()` takes ownership of `Game`. This allows me to enforce that players or games won't be re-used, and allow me to skip creating state checks to ensure these objects would behave correctly when misused this way.

## Error handling
This project uses `thiserror` to help with error handling. This crate has been chosen because it allows us to create semantic error types, while not leaking itself into the interface provided by this library.

Error handling is very limited due to the few cases in which returning an error would be acceptable. In a more complex library, we could use an associated object in `PlayerBehavior`, and make `Game::play` return either an associated object error from Player1 or Player2 using generics.
