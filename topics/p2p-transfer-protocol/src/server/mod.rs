use log::{debug, error, info, warn};
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write, copy};
use std::net::TcpStream;

use crate::protocol::StateMachine;
use crate::protocol::connection_state::ConnectionState;
use crate::protocol::message::Message;

fn on_hello(file_size: u64, state_machine: &mut StateMachine) -> Message {
    info!("Client wants to send a file of size {} bytes", file_size);
    state_machine.transition(ConnectionState::HelloReceived);

    state_machine.file_size = Some(file_size);

    // TODO: Add a basic check to handle the NACK case.

    // send ack
    state_machine.transition(ConnectionState::ACKSent);

    Message::ACK
}

fn on_message(msg: Message, state_machine: &mut StateMachine) -> Option<Message> {
    return match msg {
        Message::Hello { file_size } => Some(on_hello(file_size, state_machine)),
        Message::Send => {
            info!("Client is starting to send data.");
            state_machine.transition(ConnectionState::Established);
            None
        }

        _ => {
            warn!("Unexpected message from client: {:?}", msg);
            None
        }
    };
}

fn send_message(stream: &mut TcpStream, msg: &Message) -> Result<(), io::Error> {
    let msg_str = msg.to_string();
    stream.write_all(msg_str.as_bytes())?;
    Ok(())
}

pub fn run_server(file: &mut File, stream: &mut TcpStream) -> Result<(), io::Error> {
    let peer_addr = stream.peer_addr()?;
    println!("New connection from {peer_addr}");

    // Intialize the state machine
    let mut state_machine = StateMachine::new();
    state_machine.transition(ConnectionState::Listening);

    // Start the negociation message loop.
    message_loop(&mut state_machine, stream)?;

    // Now we're in Established state, receive the file data.
    let expected_file_size = state_machine.file_size.unwrap_or(0);

    receive_file(file, stream, expected_file_size)
}

fn receive_file(
    file: &mut File,
    stream: &mut TcpStream,
    expected_file_size: u64,
) -> io::Result<()> {
    let mut sized_stream = io::Read::take(stream, expected_file_size);

    let bytes_copied = copy(&mut sized_stream, file)?;
    file.flush()?;

    if bytes_copied != expected_file_size {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "Expected {} bytes, but received {} bytes",
                expected_file_size, bytes_copied
            ),
        ));
    }

    info!(
        "File received successfully, {} bytes written.",
        bytes_copied
    );
    Ok(())
}

fn message_loop(state_machine: &mut StateMachine, stream: &mut TcpStream) -> Result<(), io::Error> {
    let reader_stream = stream.try_clone().expect("Failed to clone stream");
    let mut writer_stream = stream;
    let reader = BufReader::new(reader_stream);

    for line in reader.lines() {
        let message = match line {
            Ok(line) => {
                debug!("Received: {}", line);
                line.parse::<Message>()
            }
            Err(e) => {
                error!("Error reading line: {}", e);
                continue;
            }
        };

        // Process message
        // FIXME: Do not use unwrap, fix the result type mess.
        match on_message(message.unwrap(), state_machine) {
            Some(resp) => send_message(&mut writer_stream, &resp)?,
            None => {} // No response is needed
        }
        // If we are in Established state, stop the line-based logic and receive the data.
        if let ConnectionState::Established = state_machine.current_state() {
            break;
        }
    }
    Ok(())
}
