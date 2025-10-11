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
                eprintln!("Worker {} failed to process {}: {}", self.id, url, error_msg);
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
        self.storage.save_page(&page.url, &page.content, work_item.depth).await?;
        
        // Extract links
        let links = self.parser.extract_links(&page.content, &page.url)?;
        
        println!("Worker {} processed {} (depth {}) - found {} links", 
                 self.id, page.url, work_item.depth, links.len());
        
        Ok(links)
    }
}