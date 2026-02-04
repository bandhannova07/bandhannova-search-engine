# Supabase Setup Guide

## ✅ Supabase Database Configuration

Your Supabase database is ready! Here's what to do next:

### 1. Setup Database Schema

**Option A: Supabase Dashboard (Recommended)**

1. Go to: https://supabase.com/dashboard
2. Select your project: `pubzdhbwgewxrgqpiefv`
3. Click **"SQL Editor"** in left sidebar
4. Click **"New Query"**
5. Copy-paste the entire content from `database/schema.sql`
6. Click **"Run"** button
7. ✅ Done! Tables created with seed URLs

**Option B: Local psql**

```bash
psql "postgresql://postgres:xiPouPKFLCKlKWNK@db.pubzdhbwgewxrgqpiefv.supabase.co:5432/postgres" < database/schema.sql
```

---

### 2. Verify Database Setup

**In Supabase Dashboard:**

1. Go to **"Table Editor"**
2. You should see:
   - `urls` table (with 5 seed URLs)
   - `crawl_stats` table
   - `crawl_progress` view

**Or run SQL:**

```sql
-- Check tables
SELECT COUNT(*) as total_urls FROM urls;

-- Should return: 5 (seed URLs)
```

---

### 3. Connection Details

**Connection String:**
```
postgresql://postgres:xiPouPKFLCKlKWNK@db.pubzdhbwgewxrgqpiefv.supabase.co:5432/postgres
```

**Components:**
- Host: `db.pubzdhbwgewxrgqpiefv.supabase.co`
- Port: `5432`
- Database: `postgres`
- User: `postgres`
- Password: `xiPouPKFLCKlKWNK`

---

### 4. Supabase Free Tier Limits

- ✅ Storage: 500MB (enough for 50K-100K URLs)
- ✅ Bandwidth: Unlimited
- ✅ Connections: Up to 200
- ✅ Auto-pause: After 7 days inactivity

---

### 5. Monitor Usage

**Dashboard → Settings → Usage**

Track:
- Database size
- Active connections
- Queries per second

---

### 6. Next Steps

After schema setup:

1. ✅ Push code to GitHub
2. ✅ Deploy to Render (Blueprint will use Supabase)
3. ✅ Verify crawler connects to Supabase
4. ✅ Monitor crawling activity

---

## 🔒 Security Note

**Password is visible in render.yaml** - This is okay for:
- Private GitHub repository
- Render environment variables (encrypted)

**For production:**
- Use Render environment variables (not hardcoded)
- Rotate password periodically
- Enable Supabase IP restrictions (optional)

---

## 💰 Cost Savings

**Using Supabase Free:**
- Render PostgreSQL: $7/month → **$0**
- **Total Render cost: $39/month** (instead of $46)

Services:
- Supabase: $0 (free tier)
- Meilisearch: $25
- Crawler: $7
- API: $7

---

## 📊 Upgrade Path

**When to upgrade to Supabase Pro ($25/month):**
- Database size > 500MB
- Need more connections
- Want better performance
- Production workload

**Benefits:**
- 8GB storage
- Better compute
- Daily backups
- Priority support
