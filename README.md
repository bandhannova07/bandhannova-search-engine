# Search Engine Backend

A fully independent, high-performance search engine backend with custom web crawler, self-hosted search engine, and fast API layer.

## Architecture

```
┌─────────────────────────────────────────────┐
│           Render Cloud Platform             │
├─────────────────────────────────────────────┤
│                                             │
│  ┌──────────────┐      ┌──────────────┐   │
│  │   Crawler    │─────▶│ Meilisearch  │   │
│  │  (Rust BG    │      │  (Docker)    │   │
│  │   Worker)    │      │              │   │
│  └──────┬───────┘      └──────▲───────┘   │
│         │                     │            │
│         ▼                     │            │
│  ┌──────────────┐             │            │
│  │  PostgreSQL  │             │            │
│  │ (URL Queue)  │             │            │
│  └──────────────┘             │            │
│                    ┌───────────┴────────┐  │
│                    │    API Server      │  │
│                    │    (Go/Gin)        │  │
│                    └────────────────────┘  │
└─────────────────────────────────────────────┘
```

## Components

### 1. Web Crawler (Rust)
- Custom-built async web crawler
- 500-1000 pages/minute throughput
- Respects robots.txt
- PostgreSQL-backed URL queue

### 2. Meilisearch (Search Engine)
- Self-hosted, Rust-based search engine
- <50ms search response time
- Typo-tolerant full-text search
- 10M+ document capacity

### 3. API Server (Go)
- High-performance REST API
- Returns 20+ search results
- Rate limiting and validation
- <100ms response time

### 4. PostgreSQL Database
- URL queue management
- Crawl status tracking
- Statistics storage

## Project Structure

```
Scrape-AI-Model/
├── crawler/              # Rust web crawler
│   ├── src/
│   │   ├── main.rs
│   │   ├── crawler.rs
│   │   ├── fetcher.rs
│   │   ├── parser.rs
│   │   └── indexer_client.rs
│   └── Cargo.toml
│
├── api/                  # Go API server
│   ├── main.go
│   ├── handlers/
│   │   └── search.go
│   ├── services/
│   │   └── meilisearch.go
│   └── go.mod
│
├── meilisearch/          # Meilisearch config
│   ├── Dockerfile
│   └── index_config.json
│
├── database/             # Database schemas
│   └── schema.sql
│
├── render.yaml           # Render deployment config
└── README.md
```

## Performance Targets

- **Crawler**: 500-1000 pages/minute
- **Search**: <50ms query time
- **API**: <100ms response time
- **Capacity**: 10M+ indexed pages

## Deployment

Optimized for Render.com deployment:
- Crawler: Background Worker ($7/month)
- Meilisearch: Docker Service ($25/month)
- API: Web Service ($7/month)
- Database: PostgreSQL ($7/month)

**Total: $46/month**

## Getting Started

See individual component READMEs:
- [Crawler Setup](./crawler/README.md)
- [API Setup](./api/README.md)
- [Deployment Guide](./docs/DEPLOYMENT.md)

## License

MIT
