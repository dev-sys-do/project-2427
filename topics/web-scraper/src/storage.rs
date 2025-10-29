use std::fs;
use std::path::{Path, PathBuf};
use url::Url;

/// Storage: saves HTML files in hierarchical structure
pub struct Storage {
    base_path: PathBuf,
}

impl Storage {
    /// Creates a new Storage instance with the specified base directory
    pub fn new(base_path: &str) -> Result<Self, String> {
        let path = PathBuf::from(base_path);
        fs::create_dir_all(&path)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;
        Ok(Storage { base_path: path })
    }

    /// Saves HTML content to a file, creating subdirectories as needed
    ///
    /// # Arguments
    /// * `url` - The URL of the page being saved
    /// * `html` - The HTML content to save
    /// * `parent_path` - Optional parent directory path for hierarchical storage
    pub fn save_page(&self, url: &str, html: &str, parent_path: Option<&Path>) -> Result<PathBuf, String> {
        let parsed_url = Url::parse(url)
            .map_err(|e| format!("Invalid URL {}: {}", url, e))?;

        // Create a safe filename from the URL
        let filename = Self::url_to_filename(&parsed_url);

        // Determine the save path
        let save_dir = if let Some(parent) = parent_path {
            self.base_path.join(parent)
        } else {
            self.base_path.clone()
        };

        // Create directory if it doesn't exist
        fs::create_dir_all(&save_dir)
            .map_err(|e| format!("Failed to create directory {:?}: {}", save_dir, e))?;

        let file_path = save_dir.join(&filename);

        // Write HTML content to file
        fs::write(&file_path, html)
            .map_err(|e| format!("Failed to write file {:?}: {}", file_path, e))?;

        // Return relative path from base (including filename)
        file_path.strip_prefix(&self.base_path)
            .map(|p| p.to_path_buf())
            .map_err(|e| format!("Failed to compute relative path: {}", e))
    }

    /// Converts a URL to a safe filename
    fn url_to_filename(url: &Url) -> String {
        let host = url.host_str().unwrap_or("unknown");
        let path = url.path();

        // Create a meaningful filename
        let name = if path == "/" || path.is_empty() {
            format!("{}.html", host)
        } else {
            // Clean up the path to create a valid filename
            let clean_path = path
                .trim_start_matches('/')
                .trim_end_matches('/')
                .replace('/', "_")
                .replace(['?', '&', '=', '#'], "_");

            if clean_path.is_empty() {
                format!("{}.html", host)
            } else if clean_path.ends_with(".html") || clean_path.ends_with(".htm") {
                clean_path
            } else {
                format!("{}_{}.html", host, clean_path)
            }
        };

        // Limit filename length
        if name.len() > 200 {
            format!("{}.html", &name[..200])
        } else {
            name
        }
    }
}
