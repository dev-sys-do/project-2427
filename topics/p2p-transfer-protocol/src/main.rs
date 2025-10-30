mod client;
mod protocol;

use std::{fs::File, net::TcpStream, process::exit};

use clap::{Parser, Subcommand};
use log::{error, info};

#[derive(Parser)]
#[command(name = "p2p-transfer-protocol", about = "Simple P2P file transfer protocol")]
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
    let cli = Cli::parse();
    // Note: if no command matches, clap automatically provides the help message & exits.   
    match cli.command {
        Commands::Listen { bind, output } => {
            server_mode(&bind, &output);
        }
        Commands::Send { file, remote_addr } => {
            client_mode(&file, &remote_addr);
        }
    }
}

fn server_mode(bind_addr: &str, output_file: &str) {
    // Start server
    // Bind to port: bind_addr
    // Open output file: output_file
    // Wait for client
    // Receive & save file
    println!("Server listening on {}. Saving to {}", bind_addr, output_file);
}

fn client_mode(file_path: &str, remote_addr: &str) {
    // Connect to server
    let mut stream = match TcpStream::connect(remote_addr) {
        Ok(mut s) => s,
        Err(e) => {
            error!("Failed to connect to {}: {}", remote_addr, e);
            exit(1);
        }
    };
    info!("Connected to {}", remote_addr);

    // Open input file
    let mut file = match File::open(file_path){
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
