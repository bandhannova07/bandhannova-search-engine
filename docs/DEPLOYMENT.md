# Deployment Guide - Render.com

Complete guide for deploying the search engine backend to Render.

## Prerequisites

- Render account ([signup](https://render.com))
- GitHub repository with your code
- Credit card (for paid services)

## Cost Estimate

| Service | Plan | Cost/Month |
|---------|------|------------|
| PostgreSQL | Starter | $7 |
| Meilisearch | Standard | $25 |
| Crawler | Starter | $7 |
| API | Starter | $7 |
| **Total** | | **$46** |

## Step-by-Step Deployment

### 1. Push Code to GitHub

```bash
cd /home/lordbandhan/Scrape-AI-Model
git init
git add .
git commit -m "Initial commit"
git remote add origin https://github.com/yourusername/search-engine.git
git push -u origin main
```

### 2. Deploy via Render Dashboard

#### Option A: Blueprint (Recommended)

1. Go to [Render Dashboard](https://dashboard.render.com)
2. Click **"New" → "Blueprint"**
3. Connect your GitHub repository
4. Render will automatically detect `render.yaml`
5. Click **"Apply"**
6. Wait for all services to deploy (~10-15 minutes)

#### Option B: Manual Deployment

Follow these steps if blueprint doesn't work:

**Step 1: Create PostgreSQL Database**
1. Dashboard → New → PostgreSQL
2. Name: `search-db`
3. Plan: Starter ($7/month)
4. Region: Singapore
5. Click "Create Database"
6. Copy the **Internal Database URL**

**Step 2: Deploy Meilisearch**
1. Dashboard → New → Web Service
2. Connect GitHub repo
3. Name: `search-engine`
4. Environment: Docker
5. Dockerfile Path: `./meilisearch/Dockerfile`
6. Plan: Standard ($25/month)
7. Add Disk:
   - Name: `meilisearch-data`
   - Mount Path: `/meili_data`
   - Size: 10GB
8. Environment Variables:
   - `MEILI_MASTER_KEY`: (Generate random key)
   - `MEILI_DB_PATH`: `/meili_data`
   - `MEILI_ENV`: `production`
9. Click "Create Web Service"
10. Copy the service URL

**Step 3: Deploy Crawler**
1. Dashboard → New → Background Worker
2. Connect GitHub repo
3. Name: `web-crawler`
4. Environment: Rust
5. Build Command: `cargo build --release`
6. Start Command: `./target/release/search-crawler`
7. Plan: Starter ($7/month)
8. Environment Variables:
   - `DATABASE_URL`: (Paste PostgreSQL Internal URL)
   - `MEILISEARCH_URL`: (Paste Meilisearch URL)
   - `MEILISEARCH_KEY`: (Same as Meilisearch master key)
   - `CRAWL_CONCURRENCY`: `100`
   - `CRAWL_DELAY_MS`: `1000`
   - `MAX_DEPTH`: `3`
   - `RUST_LOG`: `info`
9. Click "Create Background Worker"

**Step 4: Deploy API**
1. Dashboard → New → Web Service
2. Connect GitHub repo
3. Name: `search-api`
4. Environment: Go
5. Build Command: `go build -o api main.go`
6. Start Command: `./api`
7. Plan: Starter ($7/month)
8. Environment Variables:
   - `MEILISEARCH_URL`: (Paste Meilisearch URL)
   - `MEILISEARCH_KEY`: (Same as Meilisearch master key)
   - `PORT`: `8080`
   - `RATE_LIMIT`: `100`
   - `GIN_MODE`: `release`
9. Click "Create Web Service"

### 3. Initialize Database

Once PostgreSQL is deployed:

1. Go to PostgreSQL service → "Connect"
2. Copy the **External Connection String**
3. Run locally:

```bash
psql "postgresql://user:pass@host/db" < database/schema.sql
```

Or use Render Shell:
1. PostgreSQL service → "Shell"
2. Paste contents of `database/schema.sql`

### 4. Verify Deployment

**Check Services:**
- All 4 services should show "Live" status
- Check logs for errors

**Test API:**
```bash
# Replace with your API URL
curl https://search-api.onrender.com/health
```

**Test Search:**
```bash
curl -X POST https://search-api.onrender.com/search \
  -H "Content-Type: application/json" \
  -d '{"query": "test", "limit": 20}'
```

**Monitor Crawler:**
- Check crawler logs for crawling activity
- Query database to see URL queue

```sql
SELECT status, COUNT(*) FROM urls GROUP BY status;
```

### 5. Add Seed URLs

Add URLs to start crawling:

```sql
INSERT INTO urls (url, priority, depth) VALUES
  ('https://en.wikipedia.org/wiki/Main_Page', 10, 0),
  ('https://news.ycombinator.com/', 9, 0);
```

## Monitoring

### Render Dashboard

- **Metrics**: CPU, Memory, Network usage
- **Logs**: Real-time logs for each service
- **Alerts**: Set up email alerts for downtime

### Health Checks

Render automatically monitors:
- API: `/health` endpoint
- Meilisearch: `/health` endpoint
- Crawler: Process running status

### Custom Monitoring

Add to your monitoring service:
- API response times
- Search query volume
- Crawler throughput
- Index size growth

## Scaling

### Horizontal Scaling

**Add More Crawlers:**
1. Duplicate crawler service
2. Same environment variables
3. Crawlers will coordinate via database

**API Auto-scaling:**
- Render automatically scales API based on load
- Configure in service settings

### Vertical Scaling

**Upgrade Plans:**
- Meilisearch: Standard → Pro (8GB RAM)
- Database: Starter → Standard (more storage)
- Crawler: Starter → Standard (more CPU)

## Troubleshooting

### Services Won't Start

**Check logs:**
- Look for environment variable errors
- Verify database connection
- Check Meilisearch connectivity

**Common issues:**
- Missing environment variables
- Wrong database URL (use Internal URL)
- Meilisearch not ready (wait 2-3 min)

### Crawler Not Crawling

**Check:**
1. Database has seed URLs
2. Crawler logs show activity
3. Meilisearch is accessible

**Debug:**
```sql
-- Check URL queue
SELECT * FROM urls LIMIT 10;

-- Check crawl stats
SELECT * FROM crawl_progress;
```

### Search Returns No Results

**Verify:**
1. Meilisearch index exists
2. Documents are indexed
3. API can connect to Meilisearch

**Test Meilisearch directly:**
```bash
curl https://search-engine.onrender.com/indexes/web_pages/stats \
  -H "Authorization: Bearer YOUR_KEY"
```

### High Costs

**Optimize:**
1. Reduce crawler concurrency
2. Use smaller Meilisearch plan for testing
3. Pause crawler when not needed
4. Use free tier for development

## Backup & Recovery

### Database Backups

- Render: Automatic daily backups (7-day retention)
- Manual backup: Export via `pg_dump`

### Meilisearch Backups

- Snapshots stored on persistent disk
- Manual export via Meilisearch API

### Disaster Recovery

1. Restore database from Render backup
2. Restore Meilisearch from snapshot
3. Restart crawler service
4. Verify API functionality

## Security

### Best Practices

- ✅ Use environment variables for secrets
- ✅ Enable Render's automatic HTTPS
- ✅ Use Internal URLs for service communication
- ✅ Implement rate limiting
- ✅ Regular security updates

### API Security

- Rate limiting enabled (100 req/min)
- CORS configured
- Input validation
- No sensitive data in responses

## Next Steps

1. ✅ Deploy all services
2. ✅ Add seed URLs
3. ✅ Monitor crawler activity
4. ✅ Test search functionality
5. ✅ Set up monitoring alerts
6. ✅ Configure custom domain (optional)
7. ✅ Implement analytics (optional)

## Support

- Render Docs: https://render.com/docs
- Meilisearch Docs: https://docs.meilisearch.com
- GitHub Issues: Create issue in your repo
