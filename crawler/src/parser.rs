use scraper::{Html, Selector};
use url::Url;
use anyhow::Result;
use tracing::debug;

pub struct Parser;

pub struct ParsedContent {
    pub title: String,
    pub content: String,
    pub links: Vec<String>,
}

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, html: &str, base_url: &str) -> Result<ParsedContent> {
        let document = Html::parse_document(html);
        let base = Url::parse(base_url)?;

        // Extract title
        let title = self.extract_title(&document);

        // Extract main content
        let content = self.extract_content(&document);

        // Extract links
        let links = self.extract_links(&document, &base);

        debug!("Parsed: title='{}', content_len={}, links={}", 
               title, content.len(), links.len());

        Ok(ParsedContent {
            title,
            content,
            links,
        })
    }

    fn extract_title(&self, document: &Html) -> String {
        let title_selector = Selector::parse("title").unwrap();
        
        document
            .select(&title_selector)
            .next()
            .map(|el| el.text().collect::<String>())
            .unwrap_or_else(|| "Untitled".to_string())
            .trim()
            .to_string()
    }

    fn extract_content(&self, document: &Html) -> String {
        // Remove script and style tags
        let html_str = document.html();
        
        // Convert HTML to plain text
        let text = html2text::from_read(html_str.as_bytes(), 120);
        
        // Clean up whitespace
        text.lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .take(50000) // Limit to 50k chars
            .collect()
    }

    fn extract_links(&self, document: &Html, base: &Url) -> Vec<String> {
        let link_selector = Selector::parse("a[href]").unwrap();
        
        document
            .select(&link_selector)
            .filter_map(|el| el.value().attr("href"))
            .filter_map(|href| base.join(href).ok())
            .filter(|url| {
                // Only HTTP/HTTPS links
                matches!(url.scheme(), "http" | "https")
            })
            .map(|url| {
                // Remove fragment
                let mut url = url.clone();
                url.set_fragment(None);
                url.to_string()
            })
            .collect()
    }
}
