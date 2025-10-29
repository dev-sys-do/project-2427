# Web Scraper Architecture

## Overview

A concurrent Rust web scraper that downloads pages from starting URLs, follows links up to a configurable depth, and saves results in a hierarchical directory structure. It uses multiple threads for parallel processing and avoids revisiting URLs.

## Main Modules

- **cli.rs**: Parses command-line arguments (output directory, depth, URLs) using `clap`.
- **downloader.rs**: Downloads HTML content via `reqwest`.
- **parser.rs**: Extracts and resolves links from HTML using `scraper` and `url`.
- **storage.rs**: Saves pages in a directory structure reflecting link relationships.
- **scraper.rs**: Coordinates crawling, concurrency, depth, and duplicate prevention.

## Architecture Flow

```
CLI → Scraper Engine → [Worker Threads]
         ↓
   Task Queue (sync)
         ↓
Downloader, Parser, Storage
```

1. User provides URLs and options via CLI.
2. Scraper initializes shared queue and visited set.
3. Worker threads crawl, download, parse, and save pages, queuing new links until depth is reached.

## Usage

```bash
cargo build --release
./target/release/Despaux_Noa_WebScraper --output ./output --depth 2 https://example.com
```

- `--output <DIR>`: Output directory
- `--depth <N>`: Crawl depth (default: 1)
- `<URL>...`: Starting URLs

## Output Example

```
output/
├── example.com.html
└── example.com/
    ├── example.com_page1.html
    └── example.com_page1/
        └── example.com_subpage1.html
```

## Features

- Concurrent crawling (4 threads)
- Duplicate prevention
- Hierarchical storage
- Progress feedback
- Error handling
- Configurable depth
