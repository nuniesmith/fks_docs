# =================================================================
# FKS Trading Systems - Multi-Server Architecture Guide
# =================================================================

## 🏗️ Architecture Overview

This guide documents the complete multi-server architecture split that transforms the single $24 server deployment into 3 optimized servers totaling $22/month:

### 💰 Cost Optimization
- **Before**: 1x g6-standard-4 ($24/month) = $24/month
- **After**: 3x optimized servers = $22/month
  - Auth Server: g6-nanode-1 ($5/month) - 1GB RAM, 1 CPU
  - API Server: g6-standard-2 ($12/month) - 4GB RAM, 2 CPU  
  - Web Server: g6-nanode-1 ($5/month) - 1GB RAM, 1 CPU

### 🌐 Service Distribution

#### Auth Server (auth.fkstrading.xyz)
**Purpose**: SSO and authentication services
**Services**:
- Authentik SSO (Server + Worker)
- PostgreSQL (Auth database)
- Redis (Session storage)
- Nginx (SSL termination)

**Files**:
- `docker-compose.auth.yml` - Auth-specific services
- `config/nginx/auth/nginx.conf` - Nginx configuration
- Tailscale IP for internal communication

#### API Server (api.fkstrading.xyz)  
**Purpose**: Trading API, workers, and data processing
**Services**:
- FastAPI trading server
- Celery workers (background tasks)
- Celery beat (scheduler)
- PostgreSQL (main database)
- Redis (queue + cache)
- Market data services
- Risk management

**Files**:
- `docker-compose.api.yml` - API and data services
- Resource-heavy services optimized for 4GB RAM

#### Web Server (fkstrading.xyz)
**Purpose**: Static React frontend and reverse proxy
**Services**:
- React frontend (static build)
- Nginx (static serving + API proxy)
- SSL termination
- Asset optimization

**Files**:
- `docker-compose.web.yml` - Web services
- `config/nginx/web/nginx.conf` - Nginx configuration
- Proxies to API and Auth servers via Tailscale

## 🚀 Deployment Scripts

### GitHub Actions Workflow
**File**: `.github/workflows/01-multi-server-deployment.yml`
- Multi-server provisioning
- Service-specific builds
- DNS configuration with Tailscale IPs
- Health checks and deployment verification

### Server Management Scripts

#### Server Provisioning
**File**: `scripts/deployment/provision-server.sh`
```bash
./provision-server.sh auth    # Provision auth server
./provision-server.sh api     # Provision API server  
./provision-server.sh web     # Provision web server
```

#### Server Setup
**File**: `scripts/deployment/setup-server.sh`
```bash
./setup-server.sh auth        # Configure auth server
./setup-server.sh api         # Configure API server
./setup-server.sh web         # Configure web server
```

#### Service Deployment
**File**: `scripts/deployment/deploy-services.sh` 
```bash
./deploy-services.sh auth     # Deploy auth services
./deploy-services.sh api      # Deploy API services
./deploy-services.sh web      # Deploy web services
```

### Local Management Scripts

#### Multi-Server Start Script
**File**: `start-multi.sh`
```bash
# Auto-detect mode
./start-multi.sh

# Specific server modes
./start-multi.sh --auth       # Auth server only
./start-multi.sh --api        # API server only
./start-multi.sh --web        # Web server only
./start-multi.sh --multi      # All servers
./start-multi.sh --single     # Traditional single server

# With options
./start-multi.sh --api --gpu  # API server with GPU
./start-multi.sh --dev        # Development mode
```

#### Multi-Server Stop Script
**File**: `stop-multi.sh`
```bash
# Auto-detect mode
./stop-multi.sh

# Specific server modes  
./stop-multi.sh --auth        # Stop auth server
./stop-multi.sh --api         # Stop API server
./stop-multi.sh --web         # Stop web server
./stop-multi.sh --multi       # Stop all servers
./stop-multi.sh --remote      # Stop remote servers via SSH

# With cleanup options
./stop-multi.sh --cleanup-all # Full cleanup
./stop-multi.sh --force       # Force stop
```

## 🌐 Network Configuration

### Tailscale VPN Integration
- Each server gets a Tailscale IP address
- Private network communication between servers
- Secure inter-service communication

### Cloudflare DNS Configuration
The GitHub Actions workflow automatically configures:
- `auth.fkstrading.xyz` → Auth server Tailscale IP
- `api.fkstrading.xyz` → API server Tailscale IP  
- `fkstrading.xyz` → Web server Tailscale IP

### SSL/TLS Termination
- Each server handles its own SSL certificates
- Let's Encrypt integration via Certbot
- Automatic certificate renewal

## 🔧 Configuration Files

### Docker Compose Files
1. **docker-compose.auth.yml**
   - Authentik SSO stack
   - PostgreSQL for auth data
   - Redis for sessions
   - Nginx for SSL termination
   - Resource limits for $5 server

2. **docker-compose.api.yml**
   - FastAPI trading server
   - Celery workers and scheduler
   - PostgreSQL main database
   - Redis for queues and caching
   - Market data and risk services
   - Optimized for $12 server resources

3. **docker-compose.web.yml**
   - React build process
   - Nginx static file serving
   - API/Auth reverse proxy
   - Asset optimization
   - Resource limits for $5 server

### Nginx Configurations
1. **config/nginx/auth/nginx.conf**
   - SSL termination for auth.fkstrading.xyz
   - Proxy to Authentik services
   - Security headers and rate limiting

2. **config/nginx/web/nginx.conf**
   - Static React app serving
   - API proxy to api.fkstrading.xyz
   - Auth proxy to auth.fkstrading.xyz
   - WebSocket support for real-time data
   - Asset caching and compression

## 🎯 Environment Configuration

### Environment Variables
Key variables for multi-server operation:

```bash
# Deployment mode
DEPLOYMENT_MODE=multi
SERVER_TYPE=auth|api|web

# Domain configuration  
AUTH_DOMAIN_NAME=auth.fkstrading.xyz
API_DOMAIN_NAME=api.fkstrading.xyz
WEB_DOMAIN_NAME=fkstrading.xyz

# Tailscale IPs
AUTH_SERVER_IP=100.x.x.x
API_SERVER_IP=100.x.x.x  
WEB_SERVER_IP=100.x.x.x

# Database configuration
POSTGRES_USER=fks_user
POSTGRES_PASSWORD=<generated>
POSTGRES_DB=fks_trading

# Authentication
AUTHELIA_SECRET_KEY=<generated>
JWT_SECRET_KEY=<generated>
```

## 🚀 Deployment Process

### 1. GitHub Actions Trigger
```bash
# Trigger multi-server deployment
git push origin main

# Or manually trigger with server selection
# (via GitHub Actions UI)
```

### 2. Server Provisioning
- Creates 3 Linode servers with appropriate sizing
- Installs Docker, Tailscale, basic security
- Configures server-specific optimizations

### 3. Service Deployment
- Builds and pushes Docker images
- Deploys services to appropriate servers
- Configures DNS with Tailscale IPs
- Runs health checks

### 4. Verification
- Tests service connectivity
- Verifies SSL certificates
- Checks inter-service communication
- Validates API endpoints

## 🔍 Monitoring & Health Checks

### Service Health Endpoints
- **Auth**: `https://auth.fkstrading.xyz/health`
- **API**: `https://api.fkstrading.xyz/health`
- **Web**: `https://fkstrading.xyz/health`

### Inter-Service Communication
- API → Auth for user authentication
- Web → API for trading data
- Web → Auth for OAuth flows

### Resource Monitoring
- Auth Server: CPU < 50%, RAM < 80%
- API Server: CPU < 70%, RAM < 90%
- Web Server: CPU < 40%, RAM < 60%

## 🔧 Maintenance Operations

### Updates
```bash
# Update single service
./deploy-services.sh api

# Update all services
./deploy-services.sh all
```

### Backups
```bash
# Database backup (runs on API server)
docker-compose -f docker-compose.api.yml --profile backup up db-backup
```

### Scaling
- Auth Server: Generally sufficient at $5/month
- API Server: Can upgrade to g6-standard-4 ($24) for high-frequency trading
- Web Server: Can add CDN for global distribution

## 📊 Performance Optimizations

### Auth Server ($5)
- Minimal Authentik configuration
- Optimized PostgreSQL settings
- Redis memory limits
- Nginx caching for static assets

### API Server ($12)
- Multi-worker FastAPI setup
- Celery worker concurrency tuning
- Database connection pooling
- Redis caching strategies

### Web Server ($5)
- Static file compression
- Asset optimization (WebP, gzip)
- Nginx caching
- Proxy keep-alive connections

## 🎉 Benefits of Multi-Server Architecture

1. **Cost Savings**: $24 → $22/month (8% reduction)
2. **Resource Optimization**: Right-sized servers for workloads
3. **Scalability**: Independent scaling of components
4. **Reliability**: Isolated failure domains
5. **Security**: Network segmentation via Tailscale
6. **Performance**: Dedicated resources per service type

## 🔄 Migration from Single Server

1. **Backup existing data**
2. **Run GitHub Actions multi-server workflow**
3. **Update DNS records** (automated)
4. **Verify services** are healthy
5. **Decommission old server**

The architecture maintains backward compatibility and supports both single and multi-server deployments through the same codebase.
