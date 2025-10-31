# A Web Scraper

## Description and Goal

Build a command-line application that can download and process web pages from a given list of starting URLs.

The tool should use multiple threads to scrape pages concurrently, following links it finds along the way, up to a certain depth or page count.

All scraped pages should be stored locally, in the same hierarchical order they were scraped: if page `A` points to page `B`, page `B` must be stored under a `B` folder located where page `A` is stored:

```sheell
output/
  |- A.html
  |- B/
  |--- B.html
```

```shell
webcrawl --output ./crawled_url --depth 10 <URL>
```

## Grade Factor

The grade factor for this project is *1.1*.
