# GitHub Actions Deployment Troubleshooting Guide

## 🔧 Common Issues and Solutions

### 1. **Interactive Prompts (Linode CLI)**

**Problem**: Linode CLI asks for interactive input causing EOF errors
```
EOFError: EOF when reading a line
Default Region (Optional): Default Region (Optional):
```

**Solution**: Use the pre-configuration script
```bash
# Run this before any Linode CLI commands
./scripts/configure_linode_cli.sh
```

**Or set environment variables**:
```bash
export LINODE_CLI_REGION="us-east"
export LINODE_CLI_TYPE="g6-nanode-1"
export LINODE_CLI_IMAGE="linode/ubuntu22.04"
```

### 2. **Server Setup/Reboot Conflicts**

**Problem**: Server is in setup/reboot state when deployment tries to run

**Solution**: Use the deployment status manager
```bash
# Check server status
./scripts/deployment_status.sh status

# Wait for server to be ready
./scripts/deployment_status.sh wait

# Run deployment only when ready
./scripts/deployment_status.sh deploy
```

### 3. **Cleanup Failed Servers**

**Problem**: Server cleanup fails, leaving it in inconsistent state

**Solution**: Use the server maintenance workflow
1. Go to Actions → Server Cleanup and Recovery
2. Choose "cleanup" action
3. Run the workflow
4. Then try "recovery" if needed

### 4. **Missing FKS Directory**

**Problem**: `/root/fks` directory not found

**Solution**: The updated workflow now checks multiple locations:
- `/root/fks` (primary)
- `/home/jordan/fks` (fallback)
- `/opt/fks` (alternative)
- `/var/www/fks` (web directory)

### 5. **Docker Service Issues**

**Problem**: Docker not running or services failing to start

**Solution**: The workflow now includes Docker health checks:
```bash
# Check Docker status
systemctl status docker

# Start Docker if needed
systemctl start docker

# Check service status
docker-compose ps
```

## 🚀 **Updated Deployment Flow**

### Phase 1: Pre-Deployment Checks
1. ✅ Check for server setup locks
2. ✅ Verify FKS directory exists
3. ✅ Ensure Docker is running
4. ✅ Validate auto_update.sh exists

### Phase 2: Deployment Execution
1. ✅ Run auto_update.sh
2. ✅ Pull latest code
3. ✅ Stop existing services
4. ✅ Start new services
5. ✅ Verify service health

### Phase 3: Post-Deployment Validation
1. ✅ Check Docker service status
2. ✅ Verify logs for errors
3. ✅ Report deployment status

## 🛠️ **Manual Fixes**

### Reset Server State
```bash
# SSH to server
ssh root@your-server-ip

# Remove all locks
rm -f /tmp/server_setup_in_progress
rm -f /tmp/auto_update*.lock
rm -f /tmp/deployment_status.lock

# Reset FKS repository
cd /root/fks
git reset --hard HEAD
git clean -fd
git pull origin main
chmod +x auto_update.sh

# Test deployment
./auto_update.sh
```

### Configure Linode CLI Manually
```bash
# Create config directory
mkdir -p ~/.config/linode-cli

# Create config file
cat > ~/.config/linode-cli/config << EOF
[DEFAULT]
default-user = DEFAULT
region = us-east
type = g6-nanode-1
image = linode/ubuntu22.04
EOF

# Test configuration
linode-cli regions list --text --no-headers | head -5
```

### Check Deployment Status
```bash
# Check current status
./scripts/deployment_status.sh status

# Check logs
tail -f /root/fks/logs/auto_update.log
tail -f /root/fks/logs/deployment_status.log

# Check Docker services
docker-compose ps
docker-compose logs -f
```

## 🔍 **Debugging Commands**

### Server Health Check
```bash
# System resources
free -h
df -h
ps aux | grep -E "(docker|fks)"

# Network connectivity
ping -c 3 google.com
curl -I https://github.com

# Git status
cd /root/fks
git status
git log -1 --oneline
```

### Docker Troubleshooting
```bash
# Docker system info
docker info
docker system df

# Container logs
docker-compose logs --tail=50
docker logs $(docker ps -q)

# Restart Docker
systemctl restart docker
```

## 📊 **Monitoring and Alerts**

### GitHub Actions Monitoring
1. **Actions Tab**: Monitor deployment status
2. **Email Notifications**: Set up for failed deployments
3. **Slack/Discord**: Configure webhooks for alerts

### Server Monitoring
1. **Log Files**: Monitor `/root/fks/logs/`
2. **Resource Usage**: Check CPU/memory/disk
3. **Service Health**: Monitor Docker containers

## 🔄 **Recovery Procedures**

### Full Server Recovery
```bash
# 1. Stop all services
docker-compose down --remove-orphans

# 2. Clean up locks and temp files
rm -f /tmp/server_setup_in_progress
rm -f /tmp/auto_update*.lock
rm -f /tmp/deployment_status.lock

# 3. Reset repository
cd /root/fks
git reset --hard HEAD
git clean -fd
git pull origin main

# 4. Fix permissions
chmod +x auto_update.sh
chmod +x start.sh
chmod +x scripts/*.sh

# 5. Test deployment
./auto_update.sh
```

### Rollback to Previous Version
```bash
# Check commit history
git log --oneline -10

# Rollback to specific commit
git reset --hard <commit-hash>

# Restart services
docker-compose down
docker-compose up -d --build
```

## 🚨 **Emergency Procedures**

### If Server is Completely Broken
1. Use server maintenance workflow with "cleanup" action
2. If that fails, use "recovery" action
3. If both fail, manually SSH and run recovery commands
4. Last resort: Recreate server instance

### If Deployment Keeps Failing
1. Check server status: `./scripts/deployment_status.sh status`
2. Clean up locks: `rm -f /tmp/*lock /tmp/server_setup_in_progress`
3. Test manually: `./auto_update.sh`
4. Check logs: `tail -f logs/auto_update.log`

## 📞 **Support Resources**

- **Documentation**: `docs/GITHUB_ACTIONS_AUTO_DEPLOY.md`
- **Test Script**: `./test_auto_update.sh`
- **Status Manager**: `./scripts/deployment_status.sh`
- **Linode Config**: `./scripts/configure_linode_cli.sh`

---

*Last updated: $(date)*
