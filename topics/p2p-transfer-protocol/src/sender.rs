use crate::protocol::Command;
use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::Path;

pub fn send(file_path: &Path, to_addr: &str, port: u16) -> io::Result<()> {
    // verify file exists and get metadata
    let file = File::open(file_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    let filename = file_path
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid file path"))?
        .to_string_lossy()
        .to_string();
    println!("Connecting to {to_addr}:{port}...");
    let addr = format!("{to_addr}:{port}");
    let mut stream = TcpStream::connect(&addr)?;
    println!("Connected!");
    // send HELLO
    println!("Proposing file...");
    let hello = Command::Hello {
        filename: filename.clone(),
        size: file_size,
    };
    hello.write_to(&mut stream)?;
    // wait for ACK or NACK
    let response = Command::read_from(&mut stream)?;
    match response {
        Command::Ack => {
            println!("Peer accepted the file!");
        }
        Command::Nack => {
            println!("Peer rejected the file transfer...");
            return Ok(());
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Expected ACK or NACK, got {response:?}"),
            ));
        }
    }
    // send SEND command
    let send = Command::Send { size: file_size };
    println!("Sending file...");
    send.write_to(&mut stream)?;
    // send file data
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        stream.write_all(&buffer[..n])?;
        stream.flush()?; // useless? or can cause problem? idk
    }
    println!("File sent successfully!");
    Ok(())
}

// I don't know what to test here...