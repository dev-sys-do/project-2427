use anyhow::Result;
use scraper::{Html, Selector};
use url::Url;

/// HTML parser for extracting links from pages
pub struct Parser {
    link_selector: Selector,
}

impl Parser {
    /// Create a new parser
    pub fn new() -> Result<Self> {
        let link_selector = Selector::parse("a[href]")
            .map_err(|e| anyhow::anyhow!("Failed to create CSS selector: {:?}", e))?;

        Ok(Parser { link_selector })
    }

    /// Extract all links from an HTML page
    pub fn extract_links(&self, html: &str, base_url: &Url) -> Result<Vec<Url>> {
        let document = Html::parse_document(html);
        let mut links = Vec::new();

        for element in document.select(&self.link_selector) {
            if let Some(href) = element.value().attr("href") {
                // Skip empty hrefs and fragments
                if href.is_empty() || href.starts_with('#') {
                    continue;
                }

                // Try to resolve the URL relative to the base
                match base_url.join(href) {
                    Ok(url) => {
                        // Only include HTTP/HTTPS URLs from the same domain
                        if url.scheme() == "http" || url.scheme() == "https" {
                            if let Some(base_host) = base_url.host_str() {
                                if let Some(url_host) = url.host_str() {
                                    if base_host == url_host {
                                        links.push(url);
                                    }
                                }
                            }
                        }
                    }
                    Err(_) => {
                        // Skip invalid URLs
                        continue;
                    }
                }
            }
        }

        // Remove duplicates
        links.sort();
        links.dedup();

        Ok(links)
    }
}
