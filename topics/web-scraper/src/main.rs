use clap::Parser;
use anyhow::Result;

mod cli;
mod downloader;

use cli::Args;
use downloader::Downloader;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Starting web crawler...");
    println!("Target URL: {}", args.url);
    println!("Output directory: {}", args.output.display());
    println!("Max depth: {}", args.depth);
    println!("Workers: {}", args.workers);
    println!();
    
    // Test the downloader
    println!("Testing downloader...");
    let downloader = Downloader::new()?;
    
    match downloader.download(args.url).await {
        Ok(page) => {
            println!("Successfully downloaded page!");
            println!("Final URL: {}", page.url);
            println!("Content preview (first 200 chars):");
            let preview = if page.content.len() > 200 {
                &page.content[..200]
            } else {
                &page.content
            };
            println!("{}", preview);
            if page.content.len() > 200 {
                println!("... (truncated)");
            }
        }
        Err(e) => {
            println!("Failed to download: {}", e);
        }
    }
    
    println!("\nDownloader test completed!");
    Ok(())
}
