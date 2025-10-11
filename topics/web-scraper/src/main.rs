use clap::Parser;

mod cli;
use cli::Args;

fn main() {
    let args = Args::parse();
    
    println!("Starting web crawler...");
    println!("Target URL: {}", args.url);
    println!("Output directory: {}", args.output.display());
    println!("Max depth: {}", args.depth);
    println!("Workers: {}", args.workers);
    println!("Web crawler configured successfully!");
}
