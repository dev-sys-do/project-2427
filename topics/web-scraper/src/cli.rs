use clap::Parser;

/// Web Scraper CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Output directory for scraped pages
    #[arg(short, long)]
    pub output: String,

    /// Maximum crawl depth
    #[arg(short, long, default_value_t = 1)]
    pub depth: usize,

    /// Starting URLs
    #[arg(required = true)]
    pub urls: Vec<String>,
}

