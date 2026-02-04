use anyhow::Result;
use reqwest::Client;
use std::time::Duration;
use tracing::{debug, warn};

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Self {
        let client = Client::builder()
            .user_agent("SearchBot/1.0 (+https://github.com/yourusername/search-engine)")
            .timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(5))
            .build()
            .expect("Failed to create HTTP client");

        Self { client }
    }

    pub async fn fetch(&self, url: &str) -> Result<String> {
        debug!("Fetching URL: {}", url);

        let response = self.client
            .get(url)
            .send()
            .await?;

        if !response.status().is_success() {
            warn!("Non-success status for {}: {}", url, response.status());
            anyhow::bail!("HTTP error: {}", response.status());
        }

        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        // Only process HTML content
        if !content_type.contains("text/html") {
            debug!("Skipping non-HTML content: {}", content_type);
            anyhow::bail!("Not HTML content");
        }

        let html = response.text().await?;
        debug!("Fetched {} bytes from {}", html.len(), url);

        Ok(html)
    }
}
