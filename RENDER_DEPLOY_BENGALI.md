# 🚀 Render Deployment Guide (Bengali)

## Render e Deploy Kora - Complete Guide

Ami tomake step-by-step sob kichu explain korchi. Khub easy hobe!

---

## 📋 Tomar Questions er Answer:

### 1. **Database - Render er database use korbo?**

✅ **Haan, bilkul!** Render er PostgreSQL use korbe:
- **Plan**: Starter ($7/month)
- **Storage**: 1GB (enough for 100K-500K URLs)
- **Automatic backups**: Daily
- **Internal connection**: Automatic setup

**Tomar nijeder database setup korte hobe na!** Render automatically create korbe.

### 2. **Manual work lagbe?**

✅ **Khub kom manual work:**
1. GitHub e code push korte hobe (5 minutes)
2. Render e Blueprint deploy korte hobe (1 click)
3. Database schema run korte hobe (1 command)
4. Verify korte hobe (testing)

**Total time: 15-20 minutes**

---

## 🎯 Complete Deployment Steps

### Step 1: GitHub e Code Push Koro (5 minutes)

```bash
cd /home/lordbandhan/Scrape-AI-Model

# Git initialize
git init

# Add all files
git add .

# Commit
git commit -m "Search engine backend - initial commit"

# GitHub e repository create koro (browser e):
# 1. github.com e jao
# 2. New repository click koro
# 3. Name: search-engine-backend
# 4. Public/Private select koro
# 5. Create repository

# Remote add koro (replace with your username)
git remote add origin https://github.com/YOUR_USERNAME/search-engine-backend.git

# Push koro
git branch -M main
git push -u origin main
```

✅ **Done!** Code GitHub e upload hoyeche.

---

### Step 2: Render e Deploy Koro (Blueprint Method - Easiest!)

#### 2.1 Render Account Setup

1. **Render.com e jao**: https://render.com
2. **Sign up** koro (GitHub diye login koro - easier)
3. **Dashboard** e jao

#### 2.2 Blueprint Deploy (Automatic - Recommended!)

1. **Dashboard e "New" button** click koro
2. **"Blueprint"** select koro
3. **GitHub repository connect** koro:
   - "Connect GitHub" click koro
   - Tomar repository select koro: `search-engine-backend`
   - "Connect" click koro
4. **Render automatically detect korbe** `render.yaml` file
5. **Service names review** koro:
   - `search-db` (PostgreSQL)
   - `search-engine` (Meilisearch)
   - `web-crawler` (Rust crawler)
   - `search-api` (Go API)
6. **"Apply"** button click koro

✅ **Render ekhon automatically sob services deploy korbe!**

**Wait koro 10-15 minutes** - Render sob kichu setup korche:
- PostgreSQL database create korche
- Meilisearch container deploy korche
- Rust crawler build korche (time lagbe)
- Go API build korche

---

### Step 3: Database Schema Setup (Manual - 1 Command)

Services deploy hoye gele, database e tables create korte hobe.

#### 3.1 PostgreSQL Service e Jao

1. Render Dashboard → **"search-db"** service click koro
2. **"Connect"** tab e jao
3. **"External Connection String"** copy koro
   - Example: `postgresql://user:pass@host.render.com/dbname`

#### 3.2 Schema Run Koro

**Option A: Render Shell (Easiest)**

1. `search-db` service e **"Shell"** tab e jao
2. Ei command run koro:

```sql
-- Copy-paste from database/schema.sql
-- (Open the file and copy all contents)
```

**Option B: Local psql (If you have psql installed)**

```bash
# Replace with your connection string
psql "postgresql://user:pass@host.render.com/dbname" < database/schema.sql
```

✅ **Database ready!** Tables create hoyeche ar seed URLs add hoyeche.

---

### Step 4: Verify Deployment (Testing)

#### 4.1 Check All Services

Render Dashboard e jao, check koro:

- ✅ **search-db**: Status "Available" (green)
- ✅ **search-engine**: Status "Live" (green)
- ✅ **web-crawler**: Status "Live" (green)
- ✅ **search-api**: Status "Live" (green)

Jodi kono service "Failed" dekhao, tar **Logs** check koro.

#### 4.2 Test API

API service e click koro, URL copy koro (e.g., `https://search-api.onrender.com`)

**Test 1: Health Check**
```bash
curl https://search-api.onrender.com/health
```

**Expected response:**
```json
{
  "status": "healthy",
  "meilisearch": true,
  "timestamp": 1706889600
}
```

**Test 2: Stats**
```bash
curl https://search-api.onrender.com/stats
```

**Test 3: Search (initially empty)**
```bash
curl -X POST https://search-api.onrender.com/search \
  -H "Content-Type: application/json" \
  -d '{"query": "test", "limit": 20}'
```

#### 4.3 Check Crawler Activity

1. **web-crawler** service e jao
2. **Logs** tab e jao
3. Dekhbe:
   - "Starting search crawler..."
   - "Processing X URLs"
   - "Successfully processed: https://..."

✅ **Crawler kaj korche!** URLs fetch ar index korche.

---

### Step 5: Monitor & Wait (15-30 minutes)

Crawler ekhon websites scrape korche. Wait koro 15-30 minutes, then:

**Check Database:**
```sql
-- Render Shell e run koro
SELECT status, COUNT(*) FROM urls GROUP BY status;
```

**Expected:**
- `pending`: 50-100 (new URLs)
- `completed`: 10-50 (crawled URLs)
- `processing`: 5-10 (currently crawling)

**Test Search Again:**
```bash
curl -X POST https://search-api.onrender.com/search \
  -H "Content-Type: application/json" \
  -d '{"query": "artificial intelligence", "limit": 20}'
```

Ekhon results pabe! 🎉

---

## 💰 Cost Breakdown

| Service | Plan | Monthly Cost |
|---------|------|--------------|
| PostgreSQL (1GB) | Starter | $7 |
| Meilisearch (2GB RAM) | Standard | $25 |
| Crawler Worker | Starter | $7 |
| API Server | Starter | $7 |
| **Total** | | **$46/month** |

---

## 🐛 Common Issues & Solutions

### Issue 1: Crawler Build Failed
**Solution**: Wait koro, first build 10-15 min lagbe

### Issue 2: API Can't Connect to Meilisearch
**Solution**: Environment variables check koro, same `MEILI_MASTER_KEY` use koro

### Issue 3: Database Connection Error
**Solution**: Internal Connection String use koro, External na

### Issue 4: No Search Results
**Solution**: Wait 15-30 minutes, crawler index korche

---

## ✅ Deployment Checklist

- [ ] GitHub e code pushed
- [ ] Render Blueprint deployed
- [ ] All 4 services "Live"
- [ ] Database schema executed
- [ ] API health check passes
- [ ] Crawler logs show activity
- [ ] Search returns results

---

## 🎉 Summary

**Tomar jonno steps:**

1. ✅ **5 min**: GitHub e push
2. ✅ **2 min**: Render Blueprint deploy
3. ✅ **2 min**: Database schema run
4. ✅ **5 min**: Verify & test
5. ✅ **30 min**: Wait for indexing

**Total: ~15 minutes active work**

Sob kichu automatic! 🚀
