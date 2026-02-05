use scraper::{Html, Selector};
use url::Url;
use anyhow::Result;
use tracing::debug;

pub struct Parser;

pub struct ParsedContent {
    pub title: String,
    pub name: String,
    pub description: String,
    pub icon: String,
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
        
        // Extract Site Name (urlName)
        let name = self.extract_site_name(&document, &base);

        // Extract Meta Description (urlDescription)
        let description = self.extract_description(&document);

        // Extract Favicon (urlIcon)
        let icon = self.extract_icon(&document, &base);

        // Extract main content
        let content = self.extract_content(&document);

        // Extract links
        let links = self.extract_links(&document, &base);

        debug!("Parsed: title='{}', name='{}', icon='{}', content_len={}, links={}", 
               title, name, icon, content.len(), links.len());

        Ok(ParsedContent {
            title,
            name,
            description,
            icon,
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

    fn extract_site_name(&self, document: &Html, base: &Url) -> String {
        // Try Open Graph site_name
        let og_selector = Selector::parse("meta[property='og:site_name']").unwrap();
        if let Some(name) = document.select(&og_selector).next() {
            if let Some(content) = name.value().attr("content") {
                return content.trim().to_string();
            }
        }

        // Try schema.org name
        let item_selector = Selector::parse("meta[itemprop='name']").unwrap();
        if let Some(name) = document.select(&item_selector).next() {
            if let Some(content) = name.value().attr("content") {
                return content.trim().to_string();
            }
        }

        // Fallback to domain name
        base.host_str().unwrap_or("Unknown Site").to_string()
    }

    fn extract_description(&self, document: &Html) -> String {
        // Try standard description meta tag
        let desc_selectors = [
            "meta[name='description']",
            "meta[property='og:description']",
            "meta[name='twitter:description']",
        ];

        for selector_str in desc_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(desc) = document.select(&selector).next() {
                    if let Some(content) = desc.value().attr("content") {
                        let content = content.trim();
                        if !content.is_empty() {
                            return content.to_string();
                        }
                    }
                }
            }
        }

        // If no meta description, we will use the first 200 chars of content in indexer_client
        "".to_string()
    }

    fn extract_icon(&self, document: &Html, base: &Url) -> String {
        let icon_selectors = [
            "link[rel='icon']",
            "link[rel='shortcut icon']",
            "link[rel='apple-touch-icon']",
            "link[rel='mask-icon']",
        ];

        for selector_str in icon_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                if let Some(link) = document.select(&selector).next() {
                    if let Some(href) = link.value().attr("href") {
                        if let Ok(full_url) = base.join(href) {
                            return full_url.to_string();
                        }
                    }
                }
            }
        }

        // Default to /favicon.ico if not found in meta tags
        if let Ok(default_icon) = base.join("/favicon.ico") {
            return default_icon.to_string();
        }

        "".to_string()
    }

    fn extract_content(&self, document: &Html) -> String {
        let content_selectors = [
            ".mw-parser-output > p", // Wikipedia intro paragraphs
            "article p", 
            "main p", 
            "#content p", 
            ".post-content p",
            "p" // Fallback
        ];
        
        let mut paragraphs = Vec::new();

        for selector_str in content_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let text = element.text().collect::<String>().trim().to_string();
                    // Basic heuristic: a good description paragraph is 100-500 chars 
                    // and doesn't look like a menu (lots of Pipe or Brackets)
                    if text.len() > 50 && !text.contains("|") && text.split_whitespace().count() > 10 {
                        // Clean the text: remove [1], [edit], (listen), etc.
                        let mut clean_text = text;
                        // Regex-like removal of squared brackets [..]
                        while let Some(start) = clean_text.find('[') {
                            if let Some(end) = clean_text[start..].find(']') {
                                clean_text.replace_range(start..start + end + 1, "");
                            } else {
                                break;
                            }
                        }
                        
                        paragraphs.push(clean_text.trim().to_string());
                        if paragraphs.len() >= 5 { break; } // Get first 5 good ones
                    }
                }
            }
            if !paragraphs.is_empty() { break; }
        }

        if paragraphs.is_empty() {
             // Total fallback: take the whole body and hope for the best
             let text = html2text::from_read(document.html().as_bytes(), 120);
             return text.chars().take(2000).collect();
        }

        paragraphs.join(" ")
            .chars()
            .take(50000)
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
