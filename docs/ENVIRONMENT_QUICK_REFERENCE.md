# Environment Variables Quick Reference

## Essential Development Variables

```bash
# Copy to your .env.local for quick development setup
APP_ENV=development
APP_LOG_LEVEL=DEBUG
DEBUG_MODE=true

# Service Ports (change if conflicts)
API_SERVICE_PORT=8000
DATA_SERVICE_PORT=8001
WORKER_SERVICE_PORT=8002
APP_SERVICE_PORT=8003
WEB_SERVICE_PORT=3000

# Database Passwords (change for security)
POSTGRES_PASSWORD=your-secure-password
REDIS_PASSWORD=your-secure-redis-password

# Build Configuration
BUILD_PYTHON=true
BUILD_RUST_NETWORK=false
BUILD_RUST_EXECUTION=false
BUILD_CONNECTOR=false

# Requirements Files
API_REQUIREMENTS_FILE=requirements_dev.txt
DATA_REQUIREMENTS_FILE=requirements_dev.txt
WORKER_REQUIREMENTS_FILE=requirements_dev.txt
```

## Quick Environment Switching

```bash
# Development (default)
docker-compose up -d

# Production
docker-compose -f docker-compose.yml -f docker-compose.prod.yml --env-file .env.production up -d

# Staging
docker-compose --env-file .env.staging up -d

# Testing
docker-compose --env-file .env.testing up -d
```

## Common Build Variations

### Python Only (Default)
```bash
BUILD_PYTHON=true
BUILD_RUST_NETWORK=false
BUILD_RUST_EXECUTION=false
BUILD_CONNECTOR=false
BUILD_DOTNET=false
BUILD_NODE=false
```

### With Rust Components
```bash
BUILD_PYTHON=true
BUILD_RUST_NETWORK=true
BUILD_RUST_EXECUTION=true
BUILD_CONNECTOR=true
```

### GPU Support
```bash
BUILD_TYPE_CPU=gpu
BUILD_TYPE_GPU=gpu
GPU_COUNT=1
```

## Resource Limits Quick Settings

### Low Resource (Development)
```bash
API_CPU_LIMIT=1
API_MEMORY_LIMIT=1024M
DATA_CPU_LIMIT=1
DATA_MEMORY_LIMIT=1024M
POSTGRES_SHARED_BUFFERS=64MB
REDIS_MAXMEMORY=128mb
```

### High Performance (Production)
```bash
API_CPU_LIMIT=4
API_MEMORY_LIMIT=4096M
DATA_CPU_LIMIT=4
DATA_MEMORY_LIMIT=4096M
POSTGRES_SHARED_BUFFERS=512MB
REDIS_MAXMEMORY=1gb
```

## Validation Commands

```bash
# Validate environment
./scripts/validate-env.sh

# Check Docker Compose config
docker-compose config

# Validate specific environment
./scripts/validate-env.sh .env.production
```
