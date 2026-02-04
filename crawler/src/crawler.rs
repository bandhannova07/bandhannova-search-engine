use anyhow::Result;
use sqlx::{PgPool, Row};
use std::collections::HashSet;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn, error, debug};
use futures::stream::{self, StreamExt};

use crate::fetcher::Fetcher;
use crate::parser::Parser;
use crate::indexer_client::IndexerClient;

pub struct Crawler {
    pool: PgPool,
    fetcher: Arc<Fetcher>,
    parser: Arc<Parser>,
    indexer: Arc<IndexerClient>,
    concurrency: usize,
    delay_ms: u64,
    max_depth: i32,
}

impl Crawler {
    pub fn new(
        pool: PgPool,
        meilisearch_url: String,
        meilisearch_key: String,
        concurrency: usize,
        delay_ms: u64,
        max_depth: i32,
    ) -> Self {
        let fetcher = Arc::new(Fetcher::new());
        let parser = Arc::new(Parser::new());
        let indexer = Arc::new(IndexerClient::new(meilisearch_url, meilisearch_key));

        Self {
            pool,
            fetcher,
            parser,
            indexer,
            concurrency,
            delay_ms,
            max_depth,
        }
    }

    pub async fn run(&self) -> Result<()> {
        // Create index if not exists
        self.indexer.create_index_if_not_exists().await?;

        info!("Crawler started with concurrency={}, delay={}ms, max_depth={}", 
              self.concurrency, self.delay_ms, self.max_depth);

        loop {
            // Fetch pending URLs from database
            let urls = self.fetch_pending_urls(100).await?;

            if urls.is_empty() {
                info!("No pending URLs, waiting...");
                sleep(Duration::from_secs(30)).await;
                continue;
            }

            info!("Processing {} URLs", urls.len());

            // Process URLs concurrently
            stream::iter(urls)
                .map(|(id, url, depth)| {
                    let fetcher = Arc::clone(&self.fetcher);
                    let parser = Arc::clone(&self.parser);
                    let indexer = Arc::clone(&self.indexer);
                    let pool = self.pool.clone();
                    let delay_ms = self.delay_ms;
                    let max_depth = self.max_depth;

                    async move {
                        // Add delay for politeness
                        sleep(Duration::from_millis(delay_ms)).await;

                        match Self::process_url(
                            id,
                            &url,
                            depth,
                            &fetcher,
                            &parser,
                            &indexer,
                            &pool,
                            max_depth,
                        ).await {
                            Ok(_) => debug!("Successfully processed: {}", url),
                            Err(e) => warn!("Failed to process {}: {}", url, e),
                        }
                    }
                })
                .buffer_unordered(self.concurrency)
                .collect::<Vec<_>>()
                .await;

            info!("Batch complete, fetching next batch...");
        }
    }

    async fn fetch_pending_urls(&self, limit: i32) -> Result<Vec<(i32, String, i32)>> {
        let rows = sqlx::query(
            "UPDATE urls 
             SET status = 'processing' 
             WHERE id IN (
                 SELECT id FROM urls 
                 WHERE status = 'pending' 
                 ORDER BY priority DESC, created_at ASC 
                 LIMIT $1
             )
             RETURNING id, url, depth"
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        let urls = rows
            .into_iter()
            .map(|row| {
                let id: i32 = row.get("id");
                let url: String = row.get("url");
                let depth: i32 = row.get("depth");
                (id, url, depth)
            })
            .collect();

        Ok(urls)
    }

    async fn process_url(
        id: i32,
        url: &str,
        depth: i32,
        fetcher: &Fetcher,
        parser: &Parser,
        indexer: &IndexerClient,
        pool: &PgPool,
        max_depth: i32,
    ) -> Result<()> {
        debug!("Processing URL (depth={}): {}", depth, url);

        // Fetch HTML
        let html = match fetcher.fetch(url).await {
            Ok(html) => html,
            Err(e) => {
                Self::mark_url_failed(pool, id).await?;
                return Err(e);
            }
        };

        // Parse content
        let parsed = match parser.parse(&html, url) {
            Ok(parsed) => parsed,
            Err(e) => {
                Self::mark_url_failed(pool, id).await?;
                return Err(e);
            }
        };

        // Index document
        if let Err(e) = indexer.index_document(url, &parsed.title, &parsed.content).await {
            error!("Failed to index {}: {}", url, e);
            Self::mark_url_failed(pool, id).await?;
            return Err(e);
        }

        // Add new links to queue (if not at max depth)
        if depth < max_depth {
            Self::add_new_urls(pool, &parsed.links, depth + 1).await?;
        }

        // Mark as completed
        Self::mark_url_completed(pool, id).await?;

        Ok(())
    }

    async fn mark_url_completed(pool: &PgPool, id: i32) -> Result<()> {
        sqlx::query(
            "UPDATE urls SET status = 'completed', last_crawled = NOW() WHERE id = $1"
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn mark_url_failed(pool: &PgPool, id: i32) -> Result<()> {
        sqlx::query(
            "UPDATE urls 
             SET status = 'failed', 
                 error_count = error_count + 1,
                 last_crawled = NOW() 
             WHERE id = $1"
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    async fn add_new_urls(pool: &PgPool, urls: &[String], depth: i32) -> Result<()> {
        if urls.is_empty() {
            return Ok(());
        }

        // Insert new URLs (ignore duplicates)
        for url in urls {
            let _ = sqlx::query(
                "INSERT INTO urls (url, depth, priority) 
                 VALUES ($1, $2, $3) 
                 ON CONFLICT (url) DO NOTHING"
            )
            .bind(url)
            .bind(depth)
            .bind(10 - depth) // Higher priority for shallower pages
            .execute(pool)
            .await;
        }

        debug!("Added {} new URLs at depth {}", urls.len(), depth);

        Ok(())
    }
}
