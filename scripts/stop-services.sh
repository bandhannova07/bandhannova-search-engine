#!/bin/bash

echo "🧹 Stopping All Services"
echo "========================"
echo ""

# Stop Meilisearch
if docker ps | grep -q meilisearch; then
    echo "Stopping Meilisearch..."
    docker stop meilisearch
    docker rm meilisearch
    echo "✅ Meilisearch stopped"
fi

# Kill any running processes
pkill -f "search-crawler" 2>/dev/null || true
pkill -f "go run main.go" 2>/dev/null || true

echo ""
echo "✅ All services stopped"
