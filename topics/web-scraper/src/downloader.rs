use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use url::Url;

/// HTTP client for downloading web pages
pub struct Downloader {
    client: Client,
}

/// Downloaded page data
pub struct Page {
    pub url: Url,
    pub content: String,
}

impl Downloader {
    /// Create a new downloader with reasonable defaults
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("webcrawl/0.1.0")
            .build()?;
        
        Ok(Downloader { client })
    }
    
    /// Download a page from the given URL
    pub async fn download(&self, url: Url) -> Result<Page> {
        println!("Downloading: {}", url);
        
        let response = self.client.get(url.clone()).send().await?;
        
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("HTTP error {}: {}", response.status(), url));
        }
        
        let content = response.text().await?;
        
        println!("Downloaded {} bytes from {}", content.len(), url);
        
        Ok(Page {
            url,
            content,
        })
    }
}