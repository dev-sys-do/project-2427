use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;
use url::Url;

/// File system storage manager
pub struct Storage {
    pub base_path: PathBuf,
}

impl Storage {
    /// Create a new storage manager
    pub fn new(base_path: PathBuf) -> Self {
        Storage { base_path }
    }

    /// Initialize the storage directory
    pub async fn init(&self) -> Result<()> {
        fs::create_dir_all(&self.base_path).await?;
        println!(
            "Initialized storage directory: {}",
            self.base_path.display()
        );
        Ok(())
    }

    /// Save a page to the appropriate location
    pub async fn save_page(&self, url: &Url, content: &str, depth: usize) -> Result<PathBuf> {
        let file_path = self.url_to_path(url, depth)?;

        // Create parent directories
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent).await?;
        }

        // Write the content
        fs::write(&file_path, content).await?;

        println!("Saved: {} -> {}", url, file_path.display());
        Ok(file_path)
    }

    /// Convert a URL to a file path following the hierarchical structure
    fn url_to_path(&self, url: &Url, depth: usize) -> Result<PathBuf> {
        let mut path = self.base_path.clone();

        // Add depth folder for organization
        path.push(format!("depth_{}", depth));

        // Add host
        if let Some(host) = url.host_str() {
            path.push(sanitize_filename(host));
        }

        // Add path components
        let url_path = url.path();
        if url_path != "/" && !url_path.is_empty() {
            for segment in url_path.split('/').filter(|s| !s.is_empty()) {
                path.push(sanitize_filename(segment));
            }
        }

        // Ensure we have a filename with .html extension
        if path.is_dir() || path.to_string_lossy().ends_with('/') || url_path == "/" {
            path.push("index.html");
        } else if !path.to_string_lossy().ends_with(".html") {
            path.set_extension("html");
        }

        Ok(path)
    }
}

/// Sanitize a string to be safe for use as a filename
fn sanitize_filename(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>()
        .trim_matches('.')
        .to_string()
}
