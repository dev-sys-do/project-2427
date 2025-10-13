# Web Scraper Architecture

## Project Definition

### What is it?
The web scraper is a command-line application written in Rust that recursively downloads and processes web pages starting from given URLs. It crawls websites by following links, downloads pages concurrently using multiple async workers, and stores them locally in a depth-organized directory structure that maintains domain hierarchy while tracking the crawling depth of each page.

### Goals
- **Concurrent web crawling**: Download multiple pages simultaneously using async/await and tokio
- **Recursive link following**: Discover and follow links up to a specified depth with same-domain filtering
- **Depth-organized storage**: Organize downloaded content in folders that track crawling depth (depth_0, depth_1, etc.)
- **Command-line interface**: Provide an intuitive CLI with configurable output directory, depth, and worker count
- **Robust error handling**: Gracefully handle network errors, invalid URLs, and file system issues with detailed logging

## Components and Modules

### 1. CLI Module (`cli.rs`)
**Purpose**: Handle command-line argument parsing and validation.
- Parse command-line arguments (URL, output directory, depth, concurrency)
- Validate input parameters
- Display help information

### 2. Crawler Engine (`crawler.rs`)
**Purpose**: Core crawling logic and coordination using SimpleCrawler.
- Manage the crawling queue and visited URLs HashSet for deduplication
- Coordinate multiple async worker tasks via mpsc channels
- Implement depth-limited crawling with round-robin work distribution
- Handle graceful worker shutdown and result processing

### 3. Downloader Module (`downloader.rs`)
**Purpose**: Handle HTTP requests and page downloading with async support.
- Make asynchronous HTTP requests using reqwest with 30-second timeout
- Custom user-agent and proper error handling
- Return page content and metadata for processing

### 4. Parser Module (`parser.rs`)
**Purpose**: Extract links from downloaded HTML pages with filtering.
- Parse HTML content using scraper crate with CSS selectors
- Extract and normalize URLs from anchor tags (`<a href="...">`)
- Filter to same-domain links only (excludes external sites)
- Remove URL fragments and handle duplicates

### 5. Storage Module (`storage.rs`)
**Purpose**: Manage file system operations with depth-based organization.
- Create hierarchical directory structures organized by crawling depth
- Save downloaded pages to depth-specific folders (depth_0, depth_1, etc.)
- Handle file naming conflicts and path sanitization
- Convert URLs to appropriate file paths maintaining domain structure

### 6. Worker Module (`worker.rs`)
**Purpose**: Handle concurrent downloading tasks with message-passing coordination.
- Define WorkItem and WorkResult message types for communication
- Implement async workers that process URLs from a shared channel
- Coordinate downloader, parser, and storage operations
- Handle round-robin work distribution through mpsc channels

## Module Interactions

```
    CLI
     |
     v
  Crawler ←→ Worker Pool
     |           |
     v           v
  Parser ←→  Downloader
     |           |
     v           v
        Storage
```

1. **CLI** parses arguments and initializes the **Crawler**
2. **Crawler** creates a pool of **Workers** and manages the crawling queue
3. **Workers** use the **Downloader** to fetch pages
4. Downloaded content is processed by the **Parser** to extract links
5. **Storage** saves pages and creates directory structure
6. New links are fed back to the **Crawler** queue

### Architecture Justification

This modular design provides:
- **Separation of concerns**: Each module has a single responsibility
- **Testability**: Modules can be unit tested independently (29 comprehensive unit tests included)
- **Concurrency**: Async worker-based design enables efficient parallel processing
- **Extensibility**: Easy to add features like robots.txt support or different output formats
- **Error isolation**: Failures in one component don't crash the entire application

### Key Technologies
- **Rust 2024 Edition**: Memory-safe systems programming with excellent async support
- **Tokio**: Async runtime for concurrent operations and channels
- **Reqwest**: HTTP client for reliable web requests with timeout handling
- **Scraper**: HTML parsing with CSS selector support
- **Clap**: Command-line argument parsing with derive macros
- **Anyhow**: Unified error handling across all modules

## Usage

### Installation
To use the `webcrawl` command directly from anywhere in your system:

```bash
# Install to ~/.cargo/bin (make sure it's in your PATH)
cargo install --path .

# Then you can use webcrawl directly
webcrawl --output ./crawled_url --depth 10 https://example.com
```

### Basic Usage
```bash
# Crawl a website with default settings
webcrawl https://example.com

# Specify output directory and depth
webcrawl --output ./crawled_data --depth 3 https://example.com

# Control concurrency
webcrawl --output ./output --depth 2 --workers 5 https://example.com
```

### Command-line Options
- `<URL>`: Starting URL to crawl (required)
- `--output, -o`: Output directory for downloaded pages (default: "./crawled")
- `--depth, -d`: Maximum crawling depth (default: 2)
- `--workers, -w`: Number of concurrent workers (default: 4)
- `--help, -h`: Display help information

### Output Structure
The downloaded pages are organized in a hierarchical structure based on crawling depth and URL structure:

```
output/
├── depth_0/
│   └── example.com/
│       └── index.html              # Root page (depth 0)
├── depth_1/
│   └── example.com/
│       ├── about/
│       │   └── index.html          # /about page (depth 1)
│       └── products/
│           └── index.html          # /products page (depth 1)
└── depth_2/
    └── example.com/
        ├── about/
        │   └── team/
        │       └── index.html      # /about/team page (depth 2)
        └── products/
            └── software/
                └── index.html      # /products/software page (depth 2)
```

This depth-based organization allows easy tracking of how deep each page was discovered in the crawling process and provides clear separation between different crawling levels.

### Example Usage Scenarios

1. **Website backup**: `webcrawl --depth 5 --output ./backup https://mysite.com`
2. **Content analysis**: `webcrawl --depth 2 --workers 8 https://news.site.com`
3. **Link validation**: `webcrawl --depth 1 https://example.com` (shallow crawl)
