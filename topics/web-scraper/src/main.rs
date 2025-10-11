use clap::Parser;

mod cli;
use cli::Args;

fn main() {
    let args = Args::parse();
    
    println!("Starting web crawler...");
    println!("Target URL: {}", args.url);
    println!("Web crawler initialized successfully!");
}
