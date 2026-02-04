# Local Setup Guide

Quick guide to run the search engine locally for testing.

## Prerequisites

- PostgreSQL
- Docker
- Rust (will install if needed)
- Go (will install if needed)

## Quick Start

### Step 1: Install Rust & Go (if needed)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install Go
wget https://go.dev/dl/go1.21.6.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go1.21.6.linux-amd64.tar.gz
export PATH=$PATH:/usr/local/go/bin
```

### Step 2: Setup Database

```bash
# Create database (will ask for sudo password)
sudo -u postgres psql <<EOF
CREATE DATABASE search_engine_db;
CREATE USER search_user WITH PASSWORD 'search_password';
GRANT ALL PRIVILEGES ON DATABASE search_engine_db TO search_user;
\q
EOF

# Run schema
PGPASSWORD=search_password psql -U search_user -d search_engine_db -f database/schema.sql
```

### Step 3: Create .env File

```bash
cat > .env <<'EOF'
DATABASE_URL=postgresql://search_user:search_password@localhost:5432/search_engine_db
MEILISEARCH_URL=http://localhost:7700
MEILISEARCH_KEY=masterKey123
CRAWL_CONCURRENCY=50
CRAWL_DELAY_MS=2000
MAX_DEPTH=2
RUST_LOG=info
PORT=8080
RATE_LIMIT=100
GIN_MODE=debug
EOF
```

### Step 4: Start Meilisearch

```bash
docker run -d \
  --name meilisearch \
  -p 7700:7700 \
  -e MEILI_MASTER_KEY=masterKey123 \
  -e MEILI_ENV=development \
  getmeili/meilisearch:v1.6

# Wait a few seconds for it to start
sleep 5

# Test
curl http://localhost:7700/health
```

### Step 5: Start API Server

```bash
# Terminal 1
cd api
go mod download
go run main.go

# Should see: "Starting API server on port 8080"
```

### Step 6: Start Crawler (Optional)

```bash
# Terminal 2
cd crawler
cargo build --release
cargo run

# Should see: "Starting search crawler..."
```

### Step 7: Test Everything

```bash
# Test API health
curl http://localhost:8080/health

# Test search (will be empty initially)
curl -X POST http://localhost:8080/search \
  -H "Content-Type: application/json" \
  -d '{"query": "test", "limit": 20}'

# Check database
PGPASSWORD=search_password psql -U search_user -d search_engine_db -c "SELECT COUNT(*) FROM urls;"
```

## Troubleshooting

### PostgreSQL Connection Error

```bash
# Start PostgreSQL
sudo systemctl start postgresql

# Check status
sudo systemctl status postgresql
```

### Meilisearch Not Starting

```bash
# Check if already running
docker ps

# Stop and restart
docker stop meilisearch
docker rm meilisearch
# Then run the docker run command again
```

### Rust/Go Not Found

```bash
# Add to PATH
echo 'export PATH=$PATH:$HOME/.cargo/bin:/usr/local/go/bin' >> ~/.bashrc
source ~/.bashrc
```

## Stop Everything

```bash
# Stop Meilisearch
docker stop meilisearch
docker rm meilisearch

# Stop crawler (Ctrl+C in terminal)
# Stop API (Ctrl+C in terminal)
```

## What to Expect

1. **API starts** - Should see "Starting API server on port 8080"
2. **Crawler starts** - Should see "Starting search crawler..."
3. **Crawler activity** - Will fetch seed URLs and start indexing
4. **Search works** - After a few minutes, search will return results

## Next Steps

Once local testing works:
1. Push to GitHub
2. Deploy to Render using `render.yaml`
3. Monitor production deployment
