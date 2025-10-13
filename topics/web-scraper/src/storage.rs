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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_storage_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());
        assert_eq!(storage.base_path, temp_dir.path());
    }

    #[tokio::test]
    async fn test_storage_init() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path().join("test_output"));

        let result = storage.init().await;
        assert!(result.is_ok());
        assert!(storage.base_path.exists());
    }

    #[tokio::test]
    async fn test_save_page() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());
        storage.init().await.unwrap();

        let url = Url::parse("https://example.com/page1").unwrap();
        let content = "<html><body>Test content</body></html>";

        let file_path = storage.save_page(&url, content, 0).await.unwrap();

        assert!(file_path.exists());
        let saved_content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(saved_content, content);
    }

    #[test]
    fn test_url_to_path_root() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let url = Url::parse("https://example.com/").unwrap();
        let path = storage.url_to_path(&url, 0).unwrap();

        let expected = temp_dir
            .path()
            .join("depth_0")
            .join("example.com")
            .join("index.html");
        assert_eq!(path, expected);
    }

    #[test]
    fn test_url_to_path_with_path() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let url = Url::parse("https://example.com/blog/post1").unwrap();
        let path = storage.url_to_path(&url, 1).unwrap();

        let expected = temp_dir
            .path()
            .join("depth_1")
            .join("example.com")
            .join("blog")
            .join("post1.html");
        assert_eq!(path, expected);
    }

    #[test]
    fn test_url_to_path_depth_organization() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let url = Url::parse("https://example.com/test").unwrap();

        let path_depth_0 = storage.url_to_path(&url, 0).unwrap();
        let path_depth_2 = storage.url_to_path(&url, 2).unwrap();

        assert!(path_depth_0.to_string_lossy().contains("depth_0"));
        assert!(path_depth_2.to_string_lossy().contains("depth_2"));
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("normal"), "normal");
        assert_eq!(sanitize_filename("with/slash"), "with_slash");
        assert_eq!(sanitize_filename("with:colon"), "with_colon");
        assert_eq!(sanitize_filename("with*star"), "with_star");
        assert_eq!(sanitize_filename("with?question"), "with_question");
        assert_eq!(sanitize_filename("with\"quote"), "with_quote");
        assert_eq!(sanitize_filename("with<greater>"), "with_greater_");
        assert_eq!(sanitize_filename("with|pipe"), "with_pipe");
        assert_eq!(sanitize_filename(".hidden."), "hidden");
    }

    #[tokio::test]
    async fn test_save_page_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());
        storage.init().await.unwrap();

        let url = Url::parse("https://example.com/deep/nested/path").unwrap();
        let content = "test content";

        let file_path = storage.save_page(&url, content, 1).await.unwrap();

        assert!(file_path.exists());
        assert!(file_path.parent().unwrap().exists());

        let saved_content = tokio::fs::read_to_string(&file_path).await.unwrap();
        assert_eq!(saved_content, content);
    }
}
