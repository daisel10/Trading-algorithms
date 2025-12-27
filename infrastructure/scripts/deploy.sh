#!/bin/bash

# KAIRÓS Deployment Script

set -e

echo "🚀 Deploying KAIRÓS Trading Platform..."

# Check if Docker is running
if ! docker info > /dev/null 2>&1; then
    echo "❌ Docker is not running. Please start Docker first."
    exit 1
fi

# Build and start services
echo "📦 Building Docker images..."
docker-compose -f infrastructure/docker-compose.yml build

echo "🏃 Starting services..."
docker-compose -f infrastructure/docker-compose.yml up -d

echo "⏳ Waiting for services to be healthy..."
sleep 10

echo "✅ KAIRÓS platform deployed successfully!"
echo ""
echo "📊 Services:"
echo "  - Trading Core (gRPC):  http://localhost:50051"
echo "  - API (GraphQL):        http://localhost:4000/graphql"
echo "  - Dashboard:            http://localhost:4200"
echo "  - DragonflyDB:          redis://localhost:6379"
echo "  - TimescaleDB:          postgresql://localhost:5432"
echo ""
echo "📝 View logs: docker-compose -f infrastructure/docker-compose.yml logs -f"
echo "🛑 Stop: docker-compose -f infrastructure/docker-compose.yml down"
