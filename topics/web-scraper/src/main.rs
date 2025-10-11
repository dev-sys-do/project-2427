use clap::Parser;
use anyhow::Result;

mod cli;
mod downloader;
mod parser;

use cli::Args;
use downloader::Downloader;
use parser::Parser as HtmlParser;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Starting web crawler...");
    println!("Target URL: {}", args.url);
    println!("Output directory: {}", args.output.display());
    println!("Max depth: {}", args.depth);
    println!("Workers: {}", args.workers);
    println!();
    
    // Test the downloader and parser
    println!("Testing downloader and parser...");
    let downloader = Downloader::new()?;
    let parser = HtmlParser::new()?;
    
    match downloader.download(args.url.clone()).await {
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
            
            // Test the parser
            println!("\nExtracting links...");
            match parser.extract_links(&page.content, &page.url) {
                Ok(links) => {
                    println!("Found {} links:", links.len());
                    for (i, link) in links.iter().take(10).enumerate() {
                        println!("  {}. {}", i + 1, link);
                    }
                    if links.len() > 10 {
                        println!("  ... and {} more links", links.len() - 10);
                    }
                }
                Err(e) => {
                    println!("Failed to extract links: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to download: {}", e);
        }
    }
    
    println!("\nDownloader and parser test completed!");
    Ok(())
}
