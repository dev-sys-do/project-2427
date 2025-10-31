# A P2P Transfer Protocol

## Description and Goal

Build a CLI tool that allows two users on the same network to transfer a single file to each other.
The tool should be able to act as both the sender and the receiver, without a central server.

It is expected for a sender to know the IP of the receiver, i.e. there is no discovery protocol.

```shell
# Receiving a file on port 9000
p2p-tool listen --port 9000 --output ./shared

# Sending a file
p2p-tool send --file report.pdf --to 192.168.1.100 --port 9000
```

## Hints and Suggestions

- Define and document a simple networking protocol with a few commands. For example
  - HELLO: For the sender to offer a file to the receiver. It takes a file size argument.
  - ACK: For the receiver to tell the sender it is ready to receive a proposed file.
  - NACK: For the receiver to reject a proposed file.
  - SEND: Send, for the sender to actually send a file. It also takes a file size argument, that must match the `HELLO` offer.
- Start a receiving thread for every sender connection.

## Grade Factor

The grade factor for this project is *1*.
