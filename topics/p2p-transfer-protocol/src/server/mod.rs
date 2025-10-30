use std::fs::File;
use std::io;
use std::net::TcpStream;

use crate::server::connection::handle_connection;

mod connection;
mod events;

pub fn run_server(file: &mut File, stream: &mut TcpStream) -> Result<(), io::Error> {
    let peer_addr = stream.peer_addr()?;
    println!("New connection from {peer_addr}");

    handle_connection(file, stream)
}
