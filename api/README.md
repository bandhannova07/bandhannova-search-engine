# Search API

High-performance REST API built with Go and Gin framework.

## Features

- ⚡ <100ms response time
- 🔒 Rate limiting (100 req/min)
- 🌐 CORS enabled
- 📊 Health checks
- 🔍 Full-text search

## Setup

### Prerequisites

- Go 1.21+ ([install](https://go.dev/doc/install))
- Meilisearch instance

### Environment Variables

```bash
MEILISEARCH_URL=http://localhost:7700
MEILISEARCH_KEY=your_key
PORT=8080
RATE_LIMIT=100
GIN_MODE=release
```

### Install Dependencies

```bash
go mod download
```

### Build

```bash
go build -o api main.go
```

### Run

```bash
# Development
go run main.go

# Production
./api
```

## API Endpoints

### GET /

Root endpoint with API information.

**Response:**
```json
{
  "message": "Search Engine API",
  "version": "1.0.0",
  "status": "running"
}
```

---

### POST /search

Search for web pages.

**Request:**
```json
{
  "query": "artificial intelligence",
  "limit": 20,
  "offset": 0
}
```

**Response:**
```json
{
  "query": "artificial intelligence",
  "total": 1547,
  "results": [
    {
      "title": "AI in 2026",
      "url": "https://example.com/ai",
      "snippet": "Latest developments in AI...",
      "score": 0.95
    }
  ],
  "search_time_ms": 45
}
```

---

### GET /health

Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "meilisearch": true,
  "timestamp": 1706889600
}
```

---

### GET /stats

Index statistics.

**Response:**
```json
{
  "total_indexed": 150000,
  "index_size_mb": 450,
  "last_crawl": "2026-02-04T14:30:00Z"
}
```

## Rate Limiting

- Default: 100 requests per minute
- Returns `429 Too Many Requests` when exceeded
- Configurable via `RATE_LIMIT` environment variable

## CORS

- Enabled for all origins
- Allowed methods: GET, POST, OPTIONS
- Allowed headers: Origin, Content-Type, Accept

## Performance

- **Response time**: <100ms (p95)
- **Throughput**: 1000+ req/sec
- **Concurrent users**: 10,000+

## Testing

```bash
# Search test
curl -X POST http://localhost:8080/search \
  -H "Content-Type: application/json" \
  -d '{"query": "test", "limit": 20}'

# Health check
curl http://localhost:8080/health

# Stats
curl http://localhost:8080/stats
```

## Deployment

See [Deployment Guide](../docs/DEPLOYMENT.md) for Render deployment instructions.
