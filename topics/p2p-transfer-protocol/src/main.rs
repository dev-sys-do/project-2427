mod protocol;
mod listener;
mod sender;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Listen {
        #[arg(short, long, default_value = "9000")]
        port: u16,

        #[arg(short, long, default_value = "./shared")]
        output: PathBuf,
    },

    Send {
        #[arg(short, long)]
        file: PathBuf,
        
        #[arg(short, long)]
        to: String, // listener address

        #[arg(short, long, default_value = "9000")]
        port: u16,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Listen { port, output } => {
            listener::listen(port, output)
        }
        Commands::Send { file, to, port } => {
            sender::send(&file, &to, port)
        }
    };
    if let Err(e) = result {
        println!("oops, {}", e);
        process::exit(1);
    }
}
