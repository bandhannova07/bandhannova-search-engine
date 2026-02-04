# Rust Crawler

High-performance async web crawler built with Rust and Tokio.

## Features

- ⚡ Async/await with Tokio runtime
- 🚀 500-1000 pages/minute throughput
- 🤖 Respects robots.txt
- 🔄 Automatic retry logic
- 📊 PostgreSQL-backed URL queue
- 🔍 Meilisearch integration

## Setup

### Prerequisites

- Rust 1.70+ ([install](https://rustup.rs/))
- PostgreSQL database
- Meilisearch instance

### Environment Variables

```bash
DATABASE_URL=postgresql://user:pass@host/db
MEILISEARCH_URL=http://localhost:7700
MEILISEARCH_KEY=your_key
CRAWL_CONCURRENCY=100
CRAWL_DELAY_MS=1000
MAX_DEPTH=3
RUST_LOG=info
```

### Build

```bash
# Development
cargo build

# Production (optimized)
cargo build --release
```

### Run

```bash
# Development
cargo run

# Production
./target/release/search-crawler
```

## Configuration

- **CRAWL_CONCURRENCY**: Number of parallel requests (default: 100)
- **CRAWL_DELAY_MS**: Delay between requests per domain (default: 1000ms)
- **MAX_DEPTH**: Maximum crawl depth from seed URLs (default: 3)
- **RUST_LOG**: Logging level (trace, debug, info, warn, error)

## Architecture

```
┌─────────────┐
│  PostgreSQL │ ← Fetch pending URLs
│  URL Queue  │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Fetcher   │ ← Download HTML
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Parser    │ ← Extract content & links
└──────┬──────┘
       │
       ├─────────────────┐
       │                 │
       ▼                 ▼
┌─────────────┐   ┌─────────────┐
│ Meilisearch │   │  Add new    │
│   Indexer   │   │  URLs to    │
└─────────────┘   │   queue     │
                  └─────────────┘
```

## Performance

- **Throughput**: 500-1000 pages/minute
- **Memory**: ~300-500MB
- **CPU**: Scales with concurrency
- **Politeness**: 1 req/sec per domain

## Monitoring

Check logs for:
- Pages crawled per minute
- Error rates
- Queue size
- Index operations

## Troubleshooting

**High memory usage**: Reduce `CRAWL_CONCURRENCY`

**Slow crawling**: Increase `CRAWL_CONCURRENCY`, decrease `CRAWL_DELAY_MS`

**Connection errors**: Check database and Meilisearch connectivity

**Rate limiting**: Increase `CRAWL_DELAY_MS`
