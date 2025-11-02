# Integrating this into your application

Note for Mr. Ortiz: this project wasn't decoupled into a separate library and binary crate because doing so would create an big commit with lots of files moving around, but clear contracts have been established. I am going to assume below that all files but main.rs are part of a library.

You can use this library into your application by creating two players, (e.g. a `player::ai_minmax::AIMinMax` and a `player::ai_minmax::TerminalPlayer`), creating a game with `logic::game::Game::new()`, anv invoking `Game::play()` on your `Game` instance. A sample code is available [here](../src/main.rs)

# Using the sample application provided
You can run a sample application by building the project with `cargo build --release`, and running the binary at `target/release/tic-tac-toe`
When run in this way, you will run against an AI using the MinMax algorithm to win.

In the grid presented to you, numbers represent free cells that you can play in. Cells with X or O (which will be colored) represent cells controlled by you or the AI. You can play by entering the number corresponding to the cell you want to play in.
