use clap::Parser;
use std::path::PathBuf;
use url::Url;

/// A web crawler that downloads pages and follows links
#[derive(Parser, Debug)]
#[command(name = "webcrawl")]
#[command(about = "A concurrent web scraper that downloads pages and follows links")]
pub struct Args {
    /// The starting URL to crawl
    #[arg(value_parser = parse_url)]
    pub url: Url,
    
    /// Output directory for downloaded pages
    #[arg(short, long, default_value = "./crawled")]
    pub output: PathBuf,
    
    /// Maximum crawling depth
    #[arg(short, long, default_value = "2")]
    pub depth: usize,
    
    /// Number of concurrent workers
    #[arg(short, long, default_value = "4")]
    pub workers: usize,
}

fn parse_url(s: &str) -> Result<Url, url::ParseError> {
    Url::parse(s)
}