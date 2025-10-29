use clap::Parser;

mod cli;
mod downloader;
mod parser;
mod scraper;
mod storage;

use cli::Cli;
use scraper::Scraper;

/// Main entry point for the web scraper CLI
fn main() {
    let cli = Cli::parse();

    // Validate depth
    if cli.depth == 0 {
        eprintln!("Error: depth must be at least 1");
        std::process::exit(1);
    }

    // Validate URLs
    if cli.urls.is_empty() {
        eprintln!("Error: at least one URL must be provided");
        std::process::exit(1);
    }

    let scraper = Scraper::new();
    scraper.run(&cli.output, cli.depth, &cli.urls);
}
