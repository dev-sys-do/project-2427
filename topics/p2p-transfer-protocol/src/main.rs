mod client;
mod protocol;
//mod server;

use clap::{Parser, Subcommand};

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

fn client_mode(file: &str, remote_addr: &str) {
    // Start client
    // Connect to server: remote_addr
    // Open input file: file
    // Send input file
    // Show result
    println!("Client sending {} to {}", file, remote_addr);
    // ...rest of client logic...
}
