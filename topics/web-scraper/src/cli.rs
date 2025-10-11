use clap::Parser;
use url::Url;

/// A web crawler that downloads pages and follows links
#[derive(Parser, Debug)]
#[command(name = "webcrawl")]
#[command(about = "A concurrent web scraper that downloads pages and follows links")]
pub struct Args {
    /// The starting URL to crawl
    #[arg(value_parser = parse_url)]
    pub url: Url,
}

fn parse_url(s: &str) -> Result<Url, url::ParseError> {
    Url::parse(s)
}