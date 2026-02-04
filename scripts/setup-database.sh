#!/bin/bash
set -e

echo "🗄️  Setting up PostgreSQL Database"
echo "==================================="
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Database configuration
DB_NAME="search_engine_db"
DB_USER="search_user"
DB_PASSWORD="search_password_local"

echo -e "${YELLOW}📋 Creating database and user...${NC}"

# Start PostgreSQL if not running
sudo systemctl start postgresql 2>/dev/null || true

# Create database and user
sudo -u postgres psql <<EOF
-- Drop existing database if exists
DROP DATABASE IF EXISTS ${DB_NAME};
DROP USER IF EXISTS ${DB_USER};

-- Create user
CREATE USER ${DB_USER} WITH PASSWORD '${DB_PASSWORD}';

-- Create database
CREATE DATABASE ${DB_NAME} OWNER ${DB_USER};

-- Grant privileges
GRANT ALL PRIVILEGES ON DATABASE ${DB_NAME} TO ${DB_USER};

\q
EOF

echo -e "${GREEN}✅ Database created: ${DB_NAME}${NC}"
echo -e "${GREEN}✅ User created: ${DB_USER}${NC}"

# Run schema
echo ""
echo -e "${YELLOW}📋 Creating tables...${NC}"

PGPASSWORD=${DB_PASSWORD} psql -U ${DB_USER} -d ${DB_NAME} -f database/schema.sql

echo -e "${GREEN}✅ Tables created successfully${NC}"

# Create .env file
echo ""
echo -e "${YELLOW}📋 Creating .env file...${NC}"

cat > .env <<EOF
# Database
DATABASE_URL=postgresql://${DB_USER}:${DB_PASSWORD}@localhost:5432/${DB_NAME}

# Meilisearch
MEILISEARCH_URL=http://localhost:7700
MEILISEARCH_KEY=masterKey123

# Crawler Settings
CRAWL_CONCURRENCY=50
CRAWL_DELAY_MS=2000
MAX_DEPTH=2
RUST_LOG=info

# API Settings
PORT=8080
RATE_LIMIT=100
GIN_MODE=debug
EOF

echo -e "${GREEN}✅ .env file created${NC}"

# Verify setup
echo ""
echo -e "${YELLOW}📋 Verifying database setup...${NC}"

PGPASSWORD=${DB_PASSWORD} psql -U ${DB_USER} -d ${DB_NAME} -c "SELECT COUNT(*) as seed_urls FROM urls;"

echo ""
echo -e "${GREEN}✅ Database setup complete!${NC}"
echo ""
echo "Database Details:"
echo "  Name: ${DB_NAME}"
echo "  User: ${DB_USER}"
echo "  Password: ${DB_PASSWORD}"
echo "  Connection: postgresql://${DB_USER}:${DB_PASSWORD}@localhost:5432/${DB_NAME}"
