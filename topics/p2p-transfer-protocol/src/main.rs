mod client;
mod protocol;
mod server;

use std::{
    fs::{File, OpenOptions},
    net::{TcpListener, TcpStream},
    process::exit,
    io,
};

use clap::{Parser, Subcommand};
use log::{error, info};

#[derive(Parser)]
#[command(
    name = "p2p-transfer-protocol",
    about = "Simple P2P file transfer protocol"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Listen {
        #[arg(long, default_value = "[::]:0")] // Listen on any inet & let the kernel pick a port.
        bind: String,
        #[arg(long, default_value = "out_file")]
        output: String,
    },

    Send {
        #[arg(long, default_value = "out_file")]
        file: String,
        #[arg()]
        remote_addr: String,
    },
}

fn main() {
    // Initialize logger
    env_logger::init();

    
    let cli = Cli::parse();
    // Note: if no command matches, clap automatically provides the help message & exits.
    match cli.command {
        Commands::Listen { bind, output } => {
            let _ = server_mode(&bind, &output);
        }
        Commands::Send { file, remote_addr } => {
            client_mode(&file, &remote_addr);
        }
    }
}

fn server_mode(bind_addr: &str, file_path: &str) -> io::Result<()> {
    // Start server
    // Bind to port: bind_addr
    let listener = match TcpListener::bind(&bind_addr) {
        Ok(l) => l,
        Err(e) => {
            error!("Failed to bind to {}: {}", bind_addr, e);
            exit(1);
        }
    };
    info!("Server listening on {}", bind_addr);

    // Open or create output file:
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_path)?;

    println!(
        "Server listening on {:?}. Saving to {}",
        listener, file_path
    );

    // Wait for client
    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                info!("Client connected: {}", s.peer_addr().unwrap());
                match server::run_server(&mut file, &mut s) {
                    Ok(_) => info!("File received successfully"),
                    Err(e) => error!("Failed to receive file: {}", e),
                }
            }
            Err(e) => {
                error!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn client_mode(file_path: &str, remote_addr: &str) {
    // Connect to server
    let stream = match TcpStream::connect(remote_addr) {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to connect to {}: {}", remote_addr, e);
            exit(1);
        }
    };
    info!("Connected to {}", remote_addr);

    // Open input file
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Failed to open file {}: {}", file_path, e);
            exit(1);
        }
    };

    match client::run_client(file, stream) {
        Ok(_) => info!("File sent successfully"),
        Err(e) => error!("Failed to send file: {}", e),
    }
}
