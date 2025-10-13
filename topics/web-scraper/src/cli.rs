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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parse_url_valid() {
        let result = parse_url("https://example.com");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "https://example.com/");
    }

    #[test]
    fn test_parse_url_invalid() {
        let result = parse_url("not-a-url");
        assert!(result.is_err());
    }

    #[test]
    fn test_args_with_defaults() {
        let args = Args::try_parse_from(&["webcrawl", "https://example.com"]).unwrap();

        assert_eq!(args.url.as_str(), "https://example.com/");
        assert_eq!(args.output, PathBuf::from("./crawled"));
        assert_eq!(args.depth, 2);
        assert_eq!(args.workers, 4);
    }

    #[test]
    fn test_args_with_custom_values() {
        let args = Args::try_parse_from(&[
            "webcrawl",
            "--output",
            "./custom_output",
            "--depth",
            "5",
            "--workers",
            "8",
            "https://test.com",
        ])
        .unwrap();

        assert_eq!(args.url.as_str(), "https://test.com/");
        assert_eq!(args.output, PathBuf::from("./custom_output"));
        assert_eq!(args.depth, 5);
        assert_eq!(args.workers, 8);
    }

    #[test]
    fn test_args_short_flags() {
        let args = Args::try_parse_from(&[
            "webcrawl",
            "-o",
            "./short_output",
            "-d",
            "3",
            "-w",
            "6",
            "https://short.com",
        ])
        .unwrap();

        assert_eq!(args.url.as_str(), "https://short.com/");
        assert_eq!(args.output, PathBuf::from("./short_output"));
        assert_eq!(args.depth, 3);
        assert_eq!(args.workers, 6);
    }

    #[test]
    fn test_args_invalid_url() {
        let result = Args::try_parse_from(&["webcrawl", "invalid-url"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_args_missing_url() {
        let result = Args::try_parse_from(&["webcrawl"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_args_invalid_depth() {
        let result =
            Args::try_parse_from(&["webcrawl", "--depth", "not-a-number", "https://example.com"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_args_invalid_workers() {
        let result = Args::try_parse_from(&[
            "webcrawl",
            "--workers",
            "not-a-number",
            "https://example.com",
        ]);
        assert!(result.is_err());
    }
}
