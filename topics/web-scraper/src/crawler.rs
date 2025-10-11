use crate::storage::Storage;
use crate::worker::{Worker, WorkItem, WorkResult};
use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use tokio::sync::mpsc;
use url::Url;

/// Simple crawler coordinator that manages workers
pub struct SimpleCrawler {
    storage: Storage,
    max_depth: usize,
    num_workers: usize,
    visited: HashSet<Url>,
    queue: VecDeque<WorkItem>,
}

impl SimpleCrawler {
    /// Create a new simple crawler
    pub fn new(
        storage: Storage,
        max_depth: usize,
        num_workers: usize,
        start_url: Url,
    ) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(WorkItem {
            url: start_url,
            depth: 0,
        });
        
        SimpleCrawler {
            storage,
            max_depth,
            num_workers,
            visited: HashSet::new(),
            queue,
        }
    }
    
    /// Start the crawling process
    pub async fn crawl(&mut self) -> Result<()> {
        println!("Starting crawler with {} workers, max depth {}", 
                 self.num_workers, self.max_depth);
        
        // Initialize storage
        self.storage.init().await?;
        
        // Create channels for worker communication
        let (result_sender, mut result_receiver) = mpsc::channel::<WorkResult>(100);
        
        // Create individual work channels and start workers
        let mut work_senders = Vec::new();
        let mut worker_handles = Vec::new();
        
        for i in 0..self.num_workers {
            let (work_sender, work_receiver) = mpsc::channel::<WorkItem>(10);
            work_senders.push(work_sender);
            
            let worker_storage = Storage::new(self.storage.base_path.clone());
            let worker = Worker::new(i, worker_storage)?;
            let result_tx = result_sender.clone();
            
            let handle = tokio::spawn(async move {
                worker.run(work_receiver, result_tx).await;
            });
            worker_handles.push(handle);
        }
        
        // Drop the original result sender so we can detect when all workers finish
        drop(result_sender);
        
        let mut active_work = 0;
        let mut total_processed = 0;
        let mut current_worker = 0;
        
        // Main crawling loop
        loop {
            // Send work items to workers (round-robin)
            while let Some(work_item) = self.queue.pop_front() {
                if self.visited.contains(&work_item.url) {
                    continue;
                }
                
                self.visited.insert(work_item.url.clone());
                
                // Send to next worker (round-robin)
                if work_senders[current_worker].send(work_item).await.is_err() {
                    // Worker channel is closed, break the loop
                    break;
                }
                
                current_worker = (current_worker + 1) % self.num_workers;
                active_work += 1;
            }
            
            // If no active work and queue is empty, we're done
            if active_work == 0 {
                break;
            }
            
            // Process results from workers
            if let Some(result) = result_receiver.recv().await {
                active_work -= 1;
                total_processed += 1;
                
                if result.success {
                    // Add new links to queue if we haven't reached max depth
                    if result.depth < self.max_depth {
                        for link in result.links {
                            if !self.visited.contains(&link) {
                                self.queue.push_back(WorkItem {
                                    url: link,
                                    depth: result.depth + 1,
                                });
                            }
                        }
                    }
                }
            } else {
                // All workers have finished
                break;
            }
        }
        
        // Close all work channels to signal workers to stop
        drop(work_senders);
        
        // Wait for all workers to finish
        for handle in worker_handles {
            let _ = handle.await;
        }
        
        println!("Crawling complete! Processed {} pages.", total_processed);
        Ok(())
    }
}