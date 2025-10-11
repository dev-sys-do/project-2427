use crate::downloader::Downloader;
use crate::parser::Parser as HtmlParser;
use crate::storage::Storage;
use anyhow::Result;
use tokio::sync::mpsc;
use url::Url;

/// Message types for worker communication
#[derive(Debug, Clone)]
pub struct WorkItem {
    pub url: Url,
    pub depth: usize,
}

#[derive(Debug)]
pub struct WorkResult {
    pub url: Url,
    pub depth: usize,
    pub links: Vec<Url>,
    pub success: bool,
    pub error: Option<String>,
}

/// Worker that downloads and processes web pages
pub struct Worker {
    id: usize,
    downloader: Downloader,
    parser: HtmlParser,
    storage: Storage,
}

impl Worker {
    /// Create a new worker
    pub fn new(id: usize, storage: Storage) -> Result<Self> {
        let downloader = Downloader::new()?;
        let parser = HtmlParser::new()?;

        Ok(Worker {
            id,
            downloader,
            parser,
            storage,
        })
    }

    /// Start the worker loop
    pub async fn run(
        mut self,
        mut receiver: mpsc::Receiver<WorkItem>,
        sender: mpsc::Sender<WorkResult>,
    ) {
        println!("Worker {} started", self.id);

        while let Some(work_item) = receiver.recv().await {
            let result = self.process_work_item(work_item).await;

            if let Err(e) = sender.send(result).await {
                eprintln!("Worker {} failed to send result: {}", self.id, e);
                break;
            }
        }

        println!("Worker {} shutting down", self.id);
    }

    /// Process a single work item
    async fn process_work_item(&mut self, work_item: WorkItem) -> WorkResult {
        let url = work_item.url.clone();
        let depth = work_item.depth;

        match self.download_and_process(&work_item).await {
            Ok(links) => WorkResult {
                url,
                depth,
                links,
                success: true,
                error: None,
            },
            Err(e) => {
                let error_msg = format!("{}", e);
                eprintln!(
                    "Worker {} failed to process {}: {}",
                    self.id, url, error_msg
                );
                WorkResult {
                    url,
                    depth,
                    links: Vec::new(),
                    success: false,
                    error: Some(error_msg),
                }
            }
        }
    }

    /// Download and process a single page
    async fn download_and_process(&mut self, work_item: &WorkItem) -> Result<Vec<Url>> {
        // Download the page
        let page = self.downloader.download(work_item.url.clone()).await?;

        // Save the page
        self.storage
            .save_page(&page.url, &page.content, work_item.depth)
            .await?;

        // Extract links
        let links = self.parser.extract_links(&page.content, &page.url)?;

        println!(
            "Worker {} processed {} (depth {}) - found {} links",
            self.id,
            page.url,
            work_item.depth,
            links.len()
        );

        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_work_item_creation() {
        let url = Url::parse("https://example.com").unwrap();
        let work_item = WorkItem {
            url: url.clone(),
            depth: 1,
        };

        assert_eq!(work_item.url, url);
        assert_eq!(work_item.depth, 1);
    }

    #[test]
    fn test_work_result_success() {
        let url = Url::parse("https://example.com").unwrap();
        let links = vec![Url::parse("https://example.com/page1").unwrap()];

        let result = WorkResult {
            url: url.clone(),
            depth: 0,
            links: links.clone(),
            success: true,
            error: None,
        };

        assert_eq!(result.url, url);
        assert_eq!(result.depth, 0);
        assert_eq!(result.links, links);
        assert!(result.success);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_work_result_failure() {
        let url = Url::parse("https://example.com").unwrap();
        let error_msg = "Network error".to_string();

        let result = WorkResult {
            url: url.clone(),
            depth: 1,
            links: Vec::new(),
            success: false,
            error: Some(error_msg.clone()),
        };

        assert_eq!(result.url, url);
        assert_eq!(result.depth, 1);
        assert!(result.links.is_empty());
        assert!(!result.success);
        assert_eq!(result.error, Some(error_msg));
    }

    #[tokio::test]
    async fn test_worker_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage = Storage::new(temp_dir.path().to_path_buf());

        let worker = Worker::new(0, storage);
        assert!(worker.is_ok());
        assert_eq!(worker.unwrap().id, 0);
    }

    #[test]
    fn test_work_item_clone() {
        let url = Url::parse("https://example.com").unwrap();
        let work_item = WorkItem {
            url: url.clone(),
            depth: 2,
        };
        let cloned = work_item.clone();

        assert_eq!(work_item.url, cloned.url);
        assert_eq!(work_item.depth, cloned.depth);
    }

    #[test]
    fn test_work_result_debug() {
        let url = Url::parse("https://example.com").unwrap();
        let result = WorkResult {
            url,
            depth: 1,
            links: Vec::new(),
            success: true,
            error: None,
        };

        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("WorkResult"));
        assert!(debug_str.contains("example.com"));
    }
}
