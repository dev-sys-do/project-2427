## p2p-transfert-protocol
A simple text-based protocol for file transfert.
This tool includes both a client and a single threaded server implementation.

By default, the server listens on `[::]:0`. This can 

## Usage

Via cargo run:

```bash
cargo run listen [--bind] [--output-file]
cargo run send <--server HOST> <--file FILE>
```

Or by building a binary: `cargo build --release` (`./target/release/p2p-transfert-protocol`)

### Logging

This project uses `env_logger` for logging.
For debug output (eg. the state machine's transition), set `RUST_LOG=debug`:

```bash
RUST_LOG=debug cargo run listen [--bind] [--output-file]
RUST_LOG=debug  cargo run send <--server HOST> <--file FILE>
```


## Modules
The project is structured around three main modules:
- The "server" module
- The "client" module
- The generic protocol library, containing shared serialization/deserialization logic, as well as the FSM (Finite State Machine) driving the protocol's state.

## Limitations

### Separation of concerns and limitations of this protocol

This protocol is intended as an Layer 7/Application-level protocol.

Hence, the underlying protocol stack is assumed :
- To be reliable
  - => No retransmission logic
  - Note: retries are not attempted by the current implementation
- To handle integrity and security concerns
  - => No checksums or encryption
- To handle multiplexing and/or connection reuse
  - Only a single file transfer per connection is allowed
  - As this protocol uses the same channel for data and control, it technically suffers from head-of-line blocking. 
    - In this context this however is an non-issue, given that only one file per connection can be transmitted and that resuming is not supported.


### Limitation of this implementation
- This implementation uses TCP as its transport protocol, which provides reliablility and weak integrity guarantees, but no confidentiality nor protection against tempering of the payload. 
Such a guarantee could be achieved by encapsulating this protocol in TLS.
- The serialization/deserialization logic could be simplified using serde
- The server is singlethreaded.

## Potential work
An interesting possibility would be to use QUIC as the transport protocol, which could allow connection reuse, multiplexing (using streams), as well as lower latency (by merging the transport and crypto establishement as a single step).
Latency could be futher reduced by sending the HELLO packet as a 0-RTT packet on futher connections. 