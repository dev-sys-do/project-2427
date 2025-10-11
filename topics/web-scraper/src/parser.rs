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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = Parser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_extract_links_basic() {
        let parser = Parser::new().unwrap();
        let html = r##"
            <html>
                <body>
                    <a href="/page1">Link 1</a>
                    <a href="/page2">Link 2</a>
                </body>
            </html>
        "##;
        let base_url = Url::parse("https://example.com").unwrap();

        let links = parser.extract_links(html, &base_url).unwrap();

        assert_eq!(links.len(), 2);
        assert!(links.contains(&Url::parse("https://example.com/page1").unwrap()));
        assert!(links.contains(&Url::parse("https://example.com/page2").unwrap()));
    }

    #[test]
    fn test_extract_links_filters_external() {
        let parser = Parser::new().unwrap();
        let html = r##"
            <html>
                <body>
                    <a href="/internal">Internal Link</a>
                    <a href="https://external.com/page">External Link</a>
                    <a href="https://example.com/same-domain">Same Domain</a>
                </body>
            </html>
        "##;
        let base_url = Url::parse("https://example.com").unwrap();

        let links = parser.extract_links(html, &base_url).unwrap();

        assert_eq!(links.len(), 2);
        assert!(links.contains(&Url::parse("https://example.com/internal").unwrap()));
        assert!(links.contains(&Url::parse("https://example.com/same-domain").unwrap()));
        assert!(
            !links
                .iter()
                .any(|url| url.host_str() == Some("external.com"))
        );
    }

    #[test]
    fn test_extract_links_skips_fragments() {
        let parser = Parser::new().unwrap();
        let html = r##"
            <html>
                <body>
                    <a href="/page1">Valid Link</a>
                    <a href="#fragment">Fragment</a>
                    <a href="">Empty</a>
                </body>
            </html>
        "##;
        let base_url = Url::parse("https://example.com").unwrap();

        let links = parser.extract_links(html, &base_url).unwrap();

        assert_eq!(links.len(), 1);
        assert_eq!(links[0], Url::parse("https://example.com/page1").unwrap());
    }

    #[test]
    fn test_extract_links_removes_duplicates() {
        let parser = Parser::new().unwrap();
        let html = r##"
            <html>
                <body>
                    <a href="/page1">Link 1</a>
                    <a href="/page1">Link 1 Again</a>
                    <a href="/page2">Link 2</a>
                </body>
            </html>
        "##;
        let base_url = Url::parse("https://example.com").unwrap();

        let links = parser.extract_links(html, &base_url).unwrap();

        assert_eq!(links.len(), 2);
    }

    #[test]
    fn test_extract_links_empty_html() {
        let parser = Parser::new().unwrap();
        let html = "<html><body></body></html>";
        let base_url = Url::parse("https://example.com").unwrap();

        let links = parser.extract_links(html, &base_url).unwrap();

        assert_eq!(links.len(), 0);
    }
}
