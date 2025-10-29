use reqwest::blocking::Client;
use std::time::Duration;

/// Downloader: fetches HTML content from URLs
pub struct Downloader {
    client: Client,
}

impl Downloader {
    /// Creates a new Downloader with a configured HTTP client
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("Mozilla/5.0 (compatible; RustWebScraper/0.1)")
            .build()
            .expect("Failed to create HTTP client");

        Downloader { client }
    }

    /// Downloads the HTML content from the given URL
    pub fn download(&self, url: &str) -> Result<String, String> {
        self.client
            .get(url)
            .send()
            .map_err(|e| format!("Failed to fetch {}: {}", url, e))?
            .text()
            .map_err(|e| format!("Failed to read response body: {}", e))
    }
}

impl Default for Downloader {
    fn default() -> Self {
        Self::new()
    }
}
