use std::io::{self, Read, Write, BufRead, BufReader};
use std::net::TcpStream;
use std::fs::File;
use log::{debug, info, warn};

use crate::protocol::StateMachine;
use crate::protocol::message::Message;
use crate::protocol::connection_state::ConnectionState;


pub fn run_client(mut file: File, mut stream: TcpStream) -> io::Result<()> {
    let file_size = file.metadata()?.len();
    info!("Sending a {} bytes file", file_size);

    // Initialize protocol state machine
    let mut sm = StateMachine::new();

    // Send HELLO
    let hello_msg = Message::Hello { file_size }.to_string();
    stream.write_all(hello_msg.as_bytes())?;
    sm.transition(&ConnectionState::HelloSent);
    debug!("Sent HELLO");

    // Wait for ACK/NACK
    let mut reader = BufReader::new(&mut stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;
    let response = response.trim();
    match response.parse::<Message>() {
        Ok(Message::ACK) => {
            debug!("Received ACK");
            sm.transition(&ConnectionState::ACKReceived);
        }
        Ok(Message::NACK) => {
            debug!("Received NACK");
            sm.transition(&ConnectionState::NACKReceived);
            return Ok(());
        }
        _ => {
            warn!("Unexpected response from server: {}", response);
            return Ok(());
        }
    }

    // Start sending data
    let send_msg = Message::Send.to_string();
    stream.write_all(send_msg.as_bytes())?;
    sm.transition(&ConnectionState::Established);
    
    debug!("Sending data.");

    // Send file data in 4kB chunks
    let mut buffer = [0u8; 4096];

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 { break; } // EOF
        stream.write_all(&buffer[..n])?;
    }
    // Flush to ensure all data is sent
    stream.flush()?;
    info!("File sent successfully");
    sm.transition(&ConnectionState::Closed);
    // Close socket
    stream.shutdown(std::net::Shutdown::Both)?;
    debug!("Transfer complete");
    Ok(())
}