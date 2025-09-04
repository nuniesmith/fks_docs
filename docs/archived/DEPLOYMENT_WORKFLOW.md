# FKS Trading Systems - Deployment Guide

## Overview

The FKS Trading Systems uses a two-stage deployment approach that separates infrastructure changes from code changes:

1. **Docker Build Workflow** - Builds and pushes Docker images when infrastructure changes
2. **Deployment Workflow** - Deploys code changes and updates services on the development server

## Deployment Workflows

### 1. Docker Image Building (`build-docker.yml`)

**Triggers:**
- Infrastructure changes (Dockerfiles, dependencies, configs)
- Manual trigger via GitHub Actions UI

**What triggers a build:**
- Changes to `deployment/` directory
- Changes to dependency files (`requirements*.txt`, `package*.json`, `*.csproj`)
- Changes to Docker Compose files
- Changes to config YAML files
- Manual workflow dispatch

**What it does:**
- Builds Docker images for all services
- Pushes images to Docker Hub (`nuniesmith/fks`)
- Tags images with commit hash and latest tags

### 2. Development Deployment (`deploy-dev.yml`)

**Triggers:**
- Successful completion of Docker build workflow
- Manual trigger via GitHub Actions UI

**What it does:**
- SSHs into the development server
- Updates the git repository
- Checks for new Docker images
- Restarts services only if needed
- Verifies deployment health

## Manual Deployment

For quick deployments without GitHub Actions, use the manual script:

```bash
# SSH into your development server
ssh jordan@your-dev-server

# Navigate to the FKS directory
cd ~/fks

# Run the deployment script
./scripts/deployment/manual/deploy-dev.sh
```

### Manual Script Options

```bash
# Full deployment (default)
./scripts/deployment/manual/deploy-dev.sh

# Deploy specific branch
./scripts/deployment/manual/deploy-dev.sh --branch develop

# Restart only specific services
./scripts/deployment/manual/deploy-dev.sh --services api,web

# Force restart without checking for changes
./scripts/deployment/manual/deploy-dev.sh --force

# Deploy without pulling new images
./scripts/deployment/manual/deploy-dev.sh --no-pull

# Help
./scripts/deployment/manual/deploy-dev.sh --help
```

## Required GitHub Secrets

For the GitHub Actions deployment to work, configure these secrets in your repository:

```
DEV_SERVER_HOST=your-server-ip-or-hostname
DEV_SERVER_USER=jordan
DEV_SERVER_SSH_KEY=your-private-ssh-key-content
```

## Deployment Scenarios

### Scenario 1: Code Changes Only
- Push code changes to `main` or `develop`
- No Docker build triggered (fast)
- Deployment workflow runs automatically
- Only restarts if new images available or repository updated

### Scenario 2: Infrastructure Changes
- Change `requirements.txt`, `Dockerfile`, or configs
- Docker build workflow triggers (slower)
- New images built and pushed
- Deployment workflow runs after successful build
- Services restarted with new images

### Scenario 3: Manual Deployment
- Use GitHub Actions UI to trigger manually
- Use SSH + manual script for immediate deployment
- Can specify specific services to restart

## Server Setup Requirements

The development server should have:

1. **Docker and Docker Compose installed**
2. **Git repository cloned** to `/home/jordan/fks`
3. **SSH access configured** for the `jordan` user
4. **Proper permissions** for Docker commands
5. **Firewall configured** for service ports (3000, 8000-8003, etc.)

## Service Ports

Default ports for services:

| Service | Port | Description |
|---------|------|-------------|
| Web | 3000 | React frontend |
| API | 8000 | REST API |
| Data | 8001 | Data service |
| Worker | 8002 | Background worker |
| App | 8003 | Main application |
| Ninja Python | 8002 | Python trading interface |
| Ninja Dev | 5000 | .NET development |
| Ninja Build API | 4000 | Build API |
| Adminer | 8080 | Database admin |
| Redis Commander | 8081 | Redis admin |

## Monitoring Deployment

### GitHub Actions
- Check workflow status in the Actions tab
- Review deployment summaries and logs

### Server Logs
```bash
# Deployment logs
tail -f /var/log/fks-deployment.log

# Service logs
docker compose logs -f

# Specific service logs
docker compose logs -f api
```

### Health Checks
```bash
# Quick status check
docker compose ps

# API health
curl http://localhost:8000/health

# Web interface
curl http://localhost:3000
```

## Troubleshooting

### Deployment Failed
1. Check GitHub Actions logs
2. SSH into server and check service status
3. Review deployment logs: `tail -f /var/log/fks-deployment.log`
4. Check Docker service logs: `docker compose logs`

### Services Not Starting
1. Check resource usage: `free -h` and `df -h`
2. Verify image availability: `docker images nuniesmith/fks`
3. Check for port conflicts: `netstat -tlnp`
4. Review service-specific logs

### Manual Recovery
```bash
# SSH into server
ssh jordan@your-server

cd ~/fks

# Reset to clean state
docker compose down
docker system prune -af
git reset --hard origin/main

# Manual restart
./scripts/deployment/manual/deploy-dev.sh --force
```

## Best Practices

1. **Test infrastructure changes** in a feature branch first
2. **Use manual deployment script** for quick iterations
3. **Monitor deployment logs** for issues
4. **Keep dependency files clean** to minimize rebuilds
5. **Use specific service restarts** when possible for faster deployments

## Development Workflow

### For Code Changes
1. Make changes to Python/JavaScript/C# code
2. Commit and push to `main` or `develop`
3. Deployment runs automatically (no rebuild)
4. Services restart with updated code

### For Infrastructure Changes
1. Update `requirements.txt`, `Dockerfile`, or configs
2. Commit and push
3. Docker build triggers (takes longer)
4. New images pushed to Docker Hub
5. Deployment runs with new images

### For Quick Testing
1. SSH into development server
2. Make changes directly (temporary)
3. Use `./scripts/deployment/manual/deploy-dev.sh --no-pull` for quick restart
4. Remember to commit changes back to git
