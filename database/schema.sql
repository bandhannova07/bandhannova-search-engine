-- URL Queue Table
CREATE TABLE IF NOT EXISTS urls (
    id SERIAL PRIMARY KEY,
    url TEXT UNIQUE NOT NULL,
    status VARCHAR(20) DEFAULT 'pending',
    depth INTEGER DEFAULT 0,
    priority INTEGER DEFAULT 0,
    last_crawled TIMESTAMP,
    error_count INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_status_priority ON urls(status, priority DESC);
CREATE INDEX IF NOT EXISTS idx_last_crawled ON urls(last_crawled);
CREATE INDEX IF NOT EXISTS idx_depth ON urls(depth);

-- Crawl Statistics Table
CREATE TABLE IF NOT EXISTS crawl_stats (
    id SERIAL PRIMARY KEY,
    date DATE UNIQUE NOT NULL DEFAULT CURRENT_DATE,
    pages_crawled INTEGER DEFAULT 0,
    pages_indexed INTEGER DEFAULT 0,
    errors INTEGER DEFAULT 0,
    avg_response_time_ms INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger to auto-update updated_at
CREATE TRIGGER update_urls_updated_at BEFORE UPDATE ON urls
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert some seed URLs to start crawling
INSERT INTO urls (url, priority, depth) VALUES
    ('https://en.wikipedia.org/wiki/Artificial_intelligence', 10, 0),
    ('https://news.ycombinator.com/', 9, 0),
    ('https://www.reddit.com/r/programming/', 8, 0),
    ('https://github.com/trending', 8, 0),
    ('https://stackoverflow.com/', 7, 0)
ON CONFLICT (url) DO NOTHING;

-- View for monitoring crawl progress
CREATE OR REPLACE VIEW crawl_progress AS
SELECT 
    status,
    COUNT(*) as count,
    AVG(depth) as avg_depth,
    MAX(last_crawled) as last_activity
FROM urls
GROUP BY status;
