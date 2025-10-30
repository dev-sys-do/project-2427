use log::info;
use std::fs::File;
use std::io::{self, Write, copy};
use std::net::TcpStream;

use crate::protocol::StateMachine;
use crate::protocol::connection_state::ConnectionState;
use crate::protocol::message::Message;

use crate::server::events::handle_message_loop;

pub(crate) fn handle_connection(file: &mut File, stream: &mut TcpStream) -> Result<(), io::Error> {
    // Intialize the state machine
    let mut state_machine = StateMachine::new();
    state_machine.transition(ConnectionState::Listening);

    // Start the negociation message loop.
    handle_message_loop(&mut state_machine, stream)?;

    // Now we're in Established state, receive the file data.
    let expected_file_size = state_machine.file_size.unwrap_or(0);

    receive_file(file, stream, expected_file_size)
}

pub fn send_message(stream: &mut TcpStream, msg: &Message) -> Result<(), io::Error> {
    let msg_str = msg.to_string();
    stream.write_all(msg_str.as_bytes())?;
    Ok(())
}

pub fn receive_file(
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
