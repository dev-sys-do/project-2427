# P2P Transfer Protocol (Peerile)
## What's this?
This is a command-line tool for transferring files between two machines on the same network, without a central server. The tool can function as both sender and receiver.

## Architecture
### Project Structure
The project is organized into 4 modules:
- `protocol.rs`: Communication protocol definition, where all protocol commands are declared and defined for "peer-to-peer" communication.
- `listener.rs`: File reception management, it listens for incoming connections and handle them.
- `sender.rs`: File sending management, it tries to connect to a listener in order to send a file.
- `main.rs`: Entry point & command-line interface declaration

### Communication Protocol
The protocol uses four commands:
1. **HELLO filename size**: The sender proposes a file to the receiver
  - `filename`: name of the file to transfer
  - `size`: file size in bytes
2. **ACK**: The receiver accepts the proposed file
3. **NACK**: The receiver refuses the proposed file
4. **SEND size**: The sender starts sending the file
  - `size`: must match the size announced in HELLO

All commands end with a newline character (`\n`) which serves as a delimiter for the program.

## How it works?
### Receiver Side (listener)
1. The receiver listens on a specified port (or by default 9000)
2. For each incoming connection, a new thread is created
3. The receiver waits for a HELLO command indicating a filename and its size
4. If the file doesn't already exist, it sends ACK, otherwise NACK
5. It waits for the SEND command with the corresponding size
6. It receives the file data and writes it to the output directory

### Sender Side
1. The sender connects to the receiver's IP address and port
2. It sends a HELLO command with the file name and size
3. It waits for the receiver's response (ACK or NACK)
4. If ACK is received, it sends the SEND command followed by the file data
5. If NACK is received, the transfer is cancelled

## Usage
### Receiving a File
> Command examples use `peerile`, but you can use `cargo run --` instead. (the `--` is important!)
```bash
peerile listen --port 9000 --output ./shared
```
- `--port | -p`: Port on which the program will listen for incoming file transfers
- `--output | -o`: destination for received files

### Sending a File

```bash
peerile send --file document.pdf --to 192.168.1.100 --port 9000
```

- `--file | -f`: Path to the file to send
- `--to | -t`: Receiver's IP address
- `--port | -p`: Port on which the receiver is listening

