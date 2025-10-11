use clap::Parser;
use anyhow::Result;

mod cli;
mod downloader;
mod parser;
mod storage;

use cli::Args;
use downloader::Downloader;
use parser::Parser as HtmlParser;
use storage::Storage;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    println!("Starting web crawler...");
    println!("Target URL: {}", args.url);
    println!("Output directory: {}", args.output.display());
    println!("Max depth: {}", args.depth);
    println!("Workers: {}", args.workers);
    println!();
    
    // Test the downloader, parser, and storage
    println!("Testing downloader, parser, and storage...");
    let downloader = Downloader::new()?;
    let parser = HtmlParser::new()?;
    let storage = Storage::new(args.output.clone());
    
    // Initialize storage
    storage.init().await?;
    
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
            
            // Test the storage
            println!("\nSaving page to storage...");
            match storage.save_page(&page.url, &page.content, 0).await {
                Ok(file_path) => {
                    println!("Page saved successfully!");
                    println!("File path: {}", file_path.display());
                }
                Err(e) => {
                    println!("Failed to save page: {}", e);
                }
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
                    
                    // Test saving a few linked pages (simulate depth 1)
                    if !links.is_empty() {
                        println!("\nTesting hierarchical storage with linked pages...");
                        for (i, link) in links.iter().take(2).enumerate() {
                            match downloader.download(link.clone()).await {
                                Ok(linked_page) => {
                                    match storage.save_page(&linked_page.url, &linked_page.content, 1).await {
                                        Ok(file_path) => {
                                            println!("Saved linked page {}: {}", i + 1, file_path.display());
                                        }
                                        Err(e) => {
                                            println!("Failed to save linked page {}: {}", i + 1, e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    println!("Failed to download linked page {}: {}", i + 1, e);
                                }
                            }
                        }
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
    
    println!("\nDownloader, parser, and storage test completed!");
    Ok(())
}
