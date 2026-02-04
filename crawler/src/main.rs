use anyhow::Result;
use sqlx::PgPool;
use std::env;
use tracing::{info, error};
use tracing_subscriber;

mod crawler;
mod fetcher;
mod parser;
mod indexer_client;

use crawler::Crawler;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string())
        )
        .init();

    info!("Starting search crawler...");

    // Get environment variables
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let meilisearch_url = env::var("MEILISEARCH_URL")
        .expect("MEILISEARCH_URL must be set");
    let meilisearch_key = env::var("MEILISEARCH_KEY")
        .expect("MEILISEARCH_KEY must be set");
    
    let concurrency: usize = env::var("CRAWL_CONCURRENCY")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100);
    
    let delay_ms: u64 = env::var("CRAWL_DELAY_MS")
        .unwrap_or_else(|_| "1000".to_string())
        .parse()
        .unwrap_or(1000);
    
    let max_depth: i32 = env::var("MAX_DEPTH")
        .unwrap_or_else(|_| "3".to_string())
        .parse()
        .unwrap_or(3);

    // Connect to database
    info!("Connecting to database...");
    let pool = PgPool::connect(&database_url).await?;
    info!("Database connected");

    // Create crawler instance
    let crawler = Crawler::new(
        pool,
        meilisearch_url,
        meilisearch_key,
        concurrency,
        delay_ms,
        max_depth,
    );

    // Run crawler
    info!("Starting crawl loop with {} concurrent workers", concurrency);
    if let Err(e) = crawler.run().await {
        error!("Crawler error: {}", e);
        return Err(e);
    }

    Ok(())
}
