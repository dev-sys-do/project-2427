use crate::protocol::Command;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::thread;

pub fn listen(port: u16, output_dir: PathBuf) -> io::Result<()> {
    // output directory exists?
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir)?;
    }
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr)?;
    println!("Listening on port {} for incoming transfers", port);
    println!("Files will be saved to: {}", output_dir.display());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let output_dir = output_dir.clone();
                // spawn a new thread for each connection
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream, output_dir) {
                        println!("Error handling connection: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream, output_dir: PathBuf) -> io::Result<()> {
    let peer_addr = stream.peer_addr()?;
    println!("\nNew connection from {} established", peer_addr);
    // read HELLO command
    let hello = Command::read_from(&mut stream)?;
    let (filename, expected_size) = match hello {
        Command::Hello { filename, size } => {
            println!(
                "Received HELLO: file='{}', size={} bytes",
                filename, size
            );
            (filename, size)
        }
        _ => {
            println!("Expected HELLO from {}, got {:?}", peer_addr, hello);
            Command::Nack.write_to(&mut stream)?;
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected HELLO command",
            ));
        }
    };

    // validate filename (prevent path traversal)
    // (asked AI for this)
    let safe_filename = Path::new(&filename).file_name().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Invalid filename"))?;
    let output_path = output_dir.join(safe_filename);
    // check if file already exists
    if output_path.exists() {
        println!(
            "File already exists: {}. Rejecting file.",
            output_path.display()
        );
        Command::Nack.write_to(&mut stream)?;
        return Ok(());
    }
    // send ACK
    println!("Accepting file transfer from {}", peer_addr);
    Command::Ack.write_to(&mut stream)?;
    // dead SEND command
    let send = Command::read_from(&mut stream)?;
    let send_size = match send {
        Command::Send { size } => {
            size
        }
        _ => {
            println!("Expected SEND from {}, got {:?}", peer_addr, send);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Expected SEND command",
            ));
        }
    };
    // verify size matches HELLO
    if send_size != expected_size {
        println!(
            "Size mismatch for file from {} : HELLO={}, SEND={}",
            peer_addr, expected_size, send_size
        );
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Size mismatch between HELLO and SEND",
        ));
    }
    // receive file
    println!("Receiving file from {}...", peer_addr);
    let mut file = File::create(&output_path)?;
    let mut remaining_bytes = send_size;
    let mut buffer = [0u8; 8192];
    while remaining_bytes > 0 {
        let to_read = buffer.len();
        let n = stream.read(&mut buffer[..to_read])?;
        if n == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Connection closed before file transfer completed",
            ));
        }
        file.write_all(&buffer[..n])?;
        file.flush()?;
        remaining_bytes -= n as u64;
    }
    println!(
        "File received successfully: {}",
        output_path.display()
    );
    Ok(())
}
