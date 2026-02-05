# Railway Deployment Guide - Crawler Only

## 🚂 Railway te Sirf Crawler Deploy Koro

Railway te **sirf web-crawler** deploy hoga. Baaki services (Meilisearch, API) Render e rahenge.

---

## 📋 **Setup Steps:**

### **Step 1: GitHub Push (Config Files)**

```bash
cd /home/lordbandhan/Scrape-AI-Model

# Add new files
git add railway.json nixpacks.toml

# Commit
git commit -m "Add Railway config for crawler only"

# Push
git push
```

---

### **Step 2: Railway Dashboard Setup**

1. **Go to:** https://railway.app/dashboard
2. **New Project** → **Deploy from GitHub repo**
3. **Select:** `bandhannova-search-engine`
4. **Railway will detect** `railway.json` and build only crawler

---

### **Step 3: Add Environment Variables**

Railway Dashboard → Your Project → **Variables** tab:

```
DATABASE_URL = postgresql://postgres.pubzdhbwgewxrgqpiefv:xiPouPKFLCKlKWNK@aws-0-ap-south-1.pooler.supabase.com:5432/postgres

MEILISEARCH_URL = https://YOUR-MEILISEARCH-URL.onrender.com

MEILISEARCH_KEY = masterKey123

CRAWL_CONCURRENCY = 50

CRAWL_DELAY_MS = 2000

MAX_DEPTH = 2

RUST_LOG = info
```

**Important:** Replace `YOUR-MEILISEARCH-URL` with actual Render Meilisearch URL!

---

### **Step 4: Deploy**

Railway automatically deploy karega after variables add karne ke baad.

**Wait:** 5-10 minutes for Rust build

---

## 📊 **Final Architecture:**

```
┌─────────────────────────────────────┐
│         Railway (Free)              │
│  ┌───────────────────────────────┐  │
│  │   Web Crawler (Background)    │  │
│  │   - Scrapes websites          │  │
│  │   - Indexes to Meilisearch    │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
                 ↓
                 ↓ (indexes)
                 ↓
┌─────────────────────────────────────┐
│         Render (Paid)               │
│  ┌───────────────────────────────┐  │
│  │   Meilisearch ($7/month)      │  │
│  │   - Search database           │  │
│  └───────────────────────────────┘  │
│  ┌───────────────────────────────┐  │
│  │   API Server (Free)           │  │
│  │   - Search endpoint           │  │
│  └───────────────────────────────┘  │
└─────────────────────────────────────┘
                 ↑
                 ↑ (queries)
                 ↑
         ┌───────────────┐
         │     Users     │
         └───────────────┘
```

---

## 💰 **Cost Breakdown:**

| Service | Platform | Cost |
|---------|----------|------|
| Crawler | Railway | **$0** (free credit) |
| Meilisearch | Render | $7/month |
| API | Render | $0 (free + UptimeRobot) |
| Database | Supabase | $0 (free tier) |
| **Total** | | **$7/month** |

---

## ✅ **Verification:**

### **1. Check Railway Logs:**

Railway Dashboard → Deployments → Logs

Should see:
```
INFO search_crawler: Starting search crawler...
INFO search_crawler: Database connected
INFO search_crawler: Starting crawl loop
```

### **2. Check Supabase:**

```sql
SELECT status, COUNT(*) FROM urls GROUP BY status;
```

Should see URLs being processed!

### **3. Test Search:**

```bash
curl -X POST https://your-api.onrender.com/search \
  -H "Content-Type: application/json" \
  -d '{"query": "test", "limit": 20}'
```

---

## 🐛 **Troubleshooting:**

### **Issue: Build Failed**

**Check:** Railway logs for Rust errors

**Fix:** 
- Ensure `Cargo.toml` in `crawler/` folder
- Check `railway.json` config

### **Issue: Database Connection Error**

**Check:** `DATABASE_URL` environment variable

**Fix:** Use Supabase **Session mode** pooler (port 5432)

### **Issue: Meilisearch 502**

**Check:** `MEILISEARCH_URL` correct hai?

**Fix:** Copy exact URL from Render Meilisearch service

---

## 🎯 **Next Steps:**

After successful deployment:

1. ✅ Monitor Railway logs (crawler activity)
2. ✅ Check Supabase (URL status)
3. ✅ Test API search (results)
4. ✅ Setup UptimeRobot for API (keep awake)

---

**Railway Free Credit:** $5/month = ~150 hours crawler runtime = enough! 🎉
