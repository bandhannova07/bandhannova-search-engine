use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Sha256, Digest};
use chrono::Utc;
use tracing::{debug, error};

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: String,
    pub url: String,
    pub name: String,
    pub title: String,
    pub description: String,
    pub icon: String,
    pub content: String,
    pub timestamp: String,
}

pub struct IndexerClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl IndexerClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        let client = Client::new();
        Self {
            client,
            base_url,
            api_key,
        }
    }

    pub async fn index_document(&self, url: &str, name: &str, title: &str, description: &str, icon: &str, content: &str) -> Result<()> {
        // Generate unique ID from URL
        let id = self.generate_id(url);

        // Fallback for description if empty
        let final_description = if description.is_empty() {
            content
                .chars()
                .take(300)
                .collect::<String>()
                .trim()
                .to_string()
        } else {
            description.to_string()
        };

        let document = Document {
            id,
            url: url.to_string(),
            name: name.to_string(),
            title: title.to_string(),
            description: final_description,
            icon: icon.to_string(),
            content: content.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        };

        self.send_to_meilisearch(&document).await?;

        Ok(())
    }

    async fn send_to_meilisearch(&self, document: &Document) -> Result<()> {
        let url = format!("{}/indexes/web_pages/documents", self.base_url);

        debug!("Indexing document: {}", document.url);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&vec![document])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await?;
            error!("Meilisearch error {}: {}", status, body);
            anyhow::bail!("Failed to index document: {}", status);
        }

        debug!("Successfully indexed: {}", document.url);

        Ok(())
    }

    fn generate_id(&self, url: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(url.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub async fn create_index_if_not_exists(&self) -> Result<()> {
        let url = format!("{}/indexes", self.base_url);

        let index_config = json!({
            "uid": "web_pages",
            "primaryKey": "id"
        });

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&index_config)
            .send()
            .await?;

        // 201 = created, 202 = already exists
        if response.status().as_u16() == 201 || response.status().as_u16() == 202 {
            debug!("Index created or already exists");
            Ok(())
        } else {
            let status = response.status();
            let body = response.text().await?;
            error!("Failed to create index {}: {}", status, body);
            anyhow::bail!("Failed to create index: {}", status);
        }
    }
}
