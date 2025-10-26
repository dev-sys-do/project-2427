## 
TODO: Project definition: What is it? What are the goals of the tool/project?

## Usage
How can one use it? Give usage examples.


## Modules
The project is structured around three main modules:
- The server (or receiver) module
- The client (or sender) module
- The generic protocol library, allowing reuse of the state machine in both the server and client components.

## Seperations of concerns
This protocol is intended as an Layer 7/Application-level protocol.

Hence, the underlying protocol stack is assumed :
- To be reliable
  - => No retransmission logic
  - Note: retries are not attempted by the current implementation
- To handle integrity and security concerns
  - => No checksums or encryption
- To handle multiplexing and/or connection reuse
  - Only a single file transfer per connection is allowed

This implementation uses TCP as its transport protocol, which provides reliablility and weak integrity guarantees, but no confidentiality or protection against tempering of the payload.
Such a guarentee could be achieved by encapsulating this protocol using TLS.

Another interesting possibility would be to use QUIC as the transport protocol, which could allow connection reuse, multiplexing (using streams), as well as lower latency (by merging the transport and crypto establishement as a single step).
Latency could be futher reduced by sending the HELLO packet as a 0-RTT packet on futher connections.


## Protocol description

### States
- LISTEN
- HELLO_SENT
- HELLO_RECEIVED
- ACK_SENT
- ACK_RECEIVED
- NACK_RECEIVED
- ESTABLISHED



## Messages
- HELLO: For the sender to offer a file to the receiver. It takes a file size argument.
- ACK: For the receiver to tell the sender it is ready to receive a proposed file.
- NACK: For the receiver to reject a proposed file.
- SEND: Send, for the sender to actually send a file. It also takes a file size argument, that must match the HELLO offer.


