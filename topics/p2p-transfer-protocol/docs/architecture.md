## 
TODO: Project definition: What is it? What are the goals of the tool/project?

## Usage
How can one use it? Give usage examples.


## Modules
The project is structured around three main modules:
- The "server" module
- The "client" module
- The generic protocol library, containing shared serialization/deserialization logic, as well as the FSM (Finite State Machine) driving the protocol's state.

Note: for simplicity's sake, we use "client" to refer to the active opener, that is, the peer that sends the `HELLO` message.

## Seperations of concerns and limitations of this protocol
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

This implementation uses TCP as its transport protocol, which provides reliablility and weak integrity guarantees, but no confidentiality nor protection against tempering of the payload.
Such a guarantee could be achieved by encapsulating this protocol in TLS.

Another interesting possibility would be to use QUIC as the transport protocol, which could allow connection reuse, multiplexing (using streams), as well as lower latency (by merging the transport and crypto establishement as a single step).
Latency could be futher reduced by sending the HELLO packet as a 0-RTT packet on futher connections.


## Protocol description

### States
#### Initial states
- LISTENING (passive opener)
- HELLO_SENT

#### Transition states
- HELLO_RECEIVED
- ACK_SENT
- ACK_RECEIVED
- ESTABLISHED

#### Final states
- NACK_RECEIVED
- (Implicit teardown)

## Messages
- HELLO
- ACK
- NACK
- SEND

## Messages
- HELLO: For the sender to offer a file to the receiver. It takes a file size argument.
- ACK: For the receiver to tell the sender it is ready to receive a proposed file.
- NACK: For the receiver to reject a proposed file.
- SEND: Send, for the sender to actually send a file. It also takes a file size argument, that must match the HELLO offer.


## Limitations & futher work
- The serialization/deserialization logic could be simplified using serde.
- 