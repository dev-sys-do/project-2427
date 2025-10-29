use crate::downloader::Downloader;
use crate::parser::Parser;
use crate::storage::Storage;
use std::collections::{HashSet, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

/// Represents a URL to be crawled with its depth and parent path
#[derive(Clone, Debug)]
struct CrawlTask {
    url: String,
    depth: usize,
    parent_path: Option<PathBuf>,
}

/// Scraper engine: manages crawl queue, concurrency, and depth limits
pub struct Scraper;

impl Scraper {
    /// Creates a new Scraper instance
    pub fn new() -> Self {
        Scraper
    }

    /// Runs the web scraper
    ///
    /// # Arguments
    /// * `output` - Output directory for scraped pages
    /// * `max_depth` - Maximum crawl depth
    /// * `urls` - Starting URLs to crawl
    pub fn run(&self, output: &str, max_depth: usize, urls: &[String]) {
        println!("Starting web scraper...");
        println!("Output directory: {}", output);
        println!("Maximum depth: {}", max_depth);
        println!("Starting URLs: {:?}", urls);

        // Initialize storage
        let storage = match Storage::new(output) {
            Ok(s) => Arc::new(s),
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

        // Track visited URLs to avoid duplicates
        let visited = Arc::new(Mutex::new(HashSet::new()));

        // Initialize crawl queue with starting URLs
        let queue = Arc::new(Mutex::new(VecDeque::new()));
        for url in urls {
            queue.lock().unwrap().push_back(CrawlTask {
                url: url.clone(),
                depth: 0,
                parent_path: None,
            });
        }

        // Number of worker threads
        let num_threads = 4;
        let mut handles = vec![];

        // Spawn worker threads
        for thread_id in 0..num_threads {
            let queue = Arc::clone(&queue);
            let visited = Arc::clone(&visited);
            let storage = Arc::clone(&storage);
            let downloader = Downloader::new();

            let handle = thread::spawn(move || {
                loop {
                    // Get next task from queue
                    let task = {
                        let mut q = queue.lock().unwrap();
                        q.pop_front()
                    };

                    match task {
                        Some(task) => {
                            // Check if already visited
                            let should_process = {
                                let mut v = visited.lock().unwrap();
                                if v.contains(&task.url) {
                                    false
                                } else {
                                    v.insert(task.url.clone());
                                    true
                                }
                            };

                            if !should_process {
                                continue;
                            }

                            println!("[Thread {}] Crawling (depth {}): {}", thread_id, task.depth, task.url);

                            // Download the page
                            let html = match downloader.download(&task.url) {
                                Ok(content) => content,
                                Err(e) => {
                                    eprintln!("[Thread {}] {}", thread_id, e);
                                    continue;
                                }
                            };

                            // Save the page and get its path
                            let saved_file_path = match storage.save_page(&task.url, &html, task.parent_path.as_deref()) {
                                Ok(path) => path,
                                Err(e) => {
                                    eprintln!("[Thread {}] {}", thread_id, e);
                                    continue;
                                }
                            };

                            println!("[Thread {}] Saved: {} (depth {})", thread_id, task.url, task.depth);

                            // If we haven't reached max depth, extract and queue links
                            if task.depth < max_depth {
                                let links = Parser::extract_links(&html, &task.url);
                                let new_depth = task.depth + 1;

                                // Create subdirectory path from the saved file
                                // Convert "example.com.html" to "example.com/" for storing child pages
                                let child_directory = saved_file_path
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .map(|s| {
                                        if let Some(parent) = saved_file_path.parent() {
                                            parent.join(s)
                                        } else {
                                            PathBuf::from(s)
                                        }
                                    })
                                    .unwrap_or_else(|| PathBuf::from("links"));

                                let mut q = queue.lock().unwrap();
                                for link in links {
                                    // Only queue if not visited
                                    let should_queue = {
                                        let v = visited.lock().unwrap();
                                        !v.contains(&link)
                                    };

                                    if should_queue {
                                        q.push_back(CrawlTask {
                                            url: link,
                                            depth: new_depth,
                                            parent_path: Some(child_directory.clone()),
                                        });
                                    }
                                }
                            }
                        }
                        None => {
                            // Queue is empty, check if we should exit
                            thread::sleep(std::time::Duration::from_millis(100));

                            // If queue is still empty after waiting, all threads will exit
                            let q = queue.lock().unwrap();
                            if q.is_empty() {
                                break;
                            }
                        }
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        let visited_count = visited.lock().unwrap().len();
        println!("\nScraping completed!");
        println!("Total pages crawled: {}", visited_count);
    }
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
}

