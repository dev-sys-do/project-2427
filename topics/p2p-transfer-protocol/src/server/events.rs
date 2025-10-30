use log::{debug, error, info, warn};
use std::io::{self, BufRead, BufReader};
use std::net::TcpStream;

use crate::protocol::StateMachine;
use crate::protocol::connection_state::ConnectionState;
use crate::protocol::message::Message;
use crate::server::connection::send_message;

fn on_hello(file_size: u64, state_machine: &mut StateMachine) -> Message {
    info!("Client wants to send a file of size {} bytes", file_size);
    state_machine.transition(ConnectionState::HelloReceived);

    state_machine.file_size = Some(file_size);

    // TODO: Add a basic chec   k to handle the NACK case.

    // send ack
    state_machine.transition(ConnectionState::ACKSent);

    Message::Ack
}

fn on_message(msg: Message, state_machine: &mut StateMachine) -> Option<Message> {
    match msg {
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
    }
}
pub(crate) fn handle_message_loop(
    state_machine: &mut StateMachine,
    stream: &mut TcpStream,
) -> Result<(), io::Error> {
    let reader_stream = stream.try_clone().expect("Failed to clone stream");
    let writer_stream = stream;
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
        if let Some(resp) = on_message(message.unwrap(), state_machine) {
            send_message(writer_stream, &resp)?
        }

        // If we are in Established state, stop the line-based logic and receive the data.
        if let ConnectionState::Established = state_machine.current_state() {
            break;
        }
    }
    Ok(())
}
