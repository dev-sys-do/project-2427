use clap::Parser;
use anyhow::Result;

mod cli;
mod downloader;
mod parser;
mod storage;
mod worker;
mod crawler;

use cli::Args;
use storage::Storage;
use crawler::SimpleCrawler;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Starting web crawler...");
    println!("Target URL: {}", args.url);
    println!("Output directory: {}", args.output.display());
    println!("Max depth: {}", args.depth);
    println!("Workers: {}", args.workers);
    println!();
    
    // Create and start the concurrent crawler
    let storage = Storage::new(args.output);
    let mut crawler = SimpleCrawler::new(
        storage,
        args.depth,
        args.workers,
        args.url,
    );
    
    crawler.crawl().await?;
    
    println!("Web crawler completed successfully!");
    Ok(())
}
