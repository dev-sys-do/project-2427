use scraper::{Html, Selector};
use url::Url;

/// Parser: extracts links from HTML content
pub struct Parser;

impl Parser {
    /// Extracts all absolute URLs from HTML content
    ///
    /// # Arguments
    /// * `html` - The HTML content to parse
    /// * `base_url` - The base URL to resolve relative links against
    ///
    /// # Returns
    /// A vector of absolute URLs found in the HTML
    pub fn extract_links(html: &str, base_url: &str) -> Vec<String> {
        let document = Html::parse_document(html);
        let selector = Selector::parse("a[href]").unwrap();

        let base = match Url::parse(base_url) {
            Ok(url) => url,
            Err(_) => return vec![],
        };

        let mut links = Vec::new();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                // Convert relative URLs to absolute
                if let Ok(absolute_url) = base.join(href) {
                    let url_str = absolute_url.to_string();
                    // Only include http/https URLs
                    if url_str.starts_with("http://") || url_str.starts_with("https://") {
                        links.push(url_str);
                    }
                }
            }
        }

        links
    }
}
