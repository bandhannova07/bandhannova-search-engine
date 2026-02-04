#!/bin/bash
set -e

echo "🚀 Starting All Services"
echo "========================"
echo ""

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Load environment variables
if [ -f .env ]; then
    export $(cat .env | grep -v '^#' | xargs)
else
    echo -e "${RED}❌ .env file not found. Run setup-database.sh first.${NC}"
    exit 1
fi

# Start Meilisearch
echo -e "${YELLOW}🔍 Starting Meilisearch...${NC}"

# Check if Meilisearch is already running
if docker ps | grep -q meilisearch; then
    echo -e "${GREEN}✅ Meilisearch already running${NC}"
else
    docker run -d \
        --name meilisearch \
        -p 7700:7700 \
        -e MEILI_MASTER_KEY=masterKey123 \
        -e MEILI_ENV=development \
        -v $(pwd)/meili_data:/meili_data \
        getmeili/meilisearch:v1.6
    
    echo -e "${GREEN}✅ Meilisearch started${NC}"
    echo "   Waiting for Meilisearch to be ready..."
    sleep 5
fi

# Verify Meilisearch
if curl -s http://localhost:7700/health > /dev/null; then
    echo -e "${GREEN}✅ Meilisearch is healthy${NC}"
else
    echo -e "${RED}❌ Meilisearch is not responding${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}✅ All services started!${NC}"
echo ""
echo "Service URLs:"
echo "  Meilisearch: http://localhost:7700"
echo "  API (will be): http://localhost:8080"
echo ""
echo "Next steps:"
echo "1. Terminal 1: cd crawler && cargo run"
echo "2. Terminal 2: cd api && go run main.go"
echo "3. Test: curl http://localhost:8080/health"
