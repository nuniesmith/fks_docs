# FKS Auto Update Quick Reference

## 🚀 Updated Features

Your `auto_update.sh` script has been enhanced with these new capabilities:

### ✅ GitHub Actions Compatible
- **Auto-detects root user** (GitHub Actions runner)
- **Flexible path handling** - works from any directory
- **Proper git configuration** for automated environments

### 🐳 Docker-First Approach
- **Docker Compose integration** - prefers Docker over manual processes
- **Automatic image pulling** - ensures latest images are used
- **Graceful service management** - proper stop/start sequences
- **Fallback support** - uses `start.sh` if Docker unavailable

### 📊 Enhanced Logging
- **Detailed timestamps** with process IDs
- **Comprehensive error handling**
- **Lock file protection** prevents concurrent runs
- **Environment logging** shows detection results

## 🔧 Quick Commands

```bash
# Test the setup
./test_auto_update.sh

# Run auto update manually
./auto_update.sh

# Check logs
tail -f logs/auto_update.log

# Monitor Docker services
docker-compose ps
docker-compose logs -f
```

## 🔑 GitHub Secrets Setup

Add these to your repository secrets:

| Secret | Value | Example |
|--------|-------|---------|
| `SERVER_HOST` | Server IP | `192.168.1.100` |
| `SERVER_SSH_KEY` | Private SSH key | `-----BEGIN OPENSSH PRIVATE KEY-----` |

## 📋 Deployment Flow

1. **Push to main** → GitHub Actions triggers
2. **SSH to server** → Connects as root user
3. **Run auto_update** → Pulls latest code
4. **Docker services** → Stops, pulls, rebuilds, starts
5. **Logging** → All activity logged
6. **Health check** → Verifies services are running

## 🔍 Troubleshooting

### Common Issues & Solutions

**SSH Connection Failed:**
```bash
# Test SSH manually
ssh root@your-server-ip
```

**Permission Denied:**
```bash
# Fix permissions
chmod +x auto_update.sh
chown root:root auto_update.sh
```

**Docker Not Starting:**
```bash
# Check Docker status
systemctl status docker
docker-compose config
```

**Git Issues:**
```bash
# Check git status
git status
git config --list
```

## 📁 File Structure

```
/root/fks/
├── auto_update.sh          # Main deployment script
├── test_auto_update.sh     # Test script
├── start.sh               # Fallback startup script
├── docker-compose.yml     # Docker services
├── logs/
│   └── auto_update.log    # Deployment logs
└── .github/workflows/
    └── auto-deploy.yml    # GitHub Actions workflow
```

## 🎯 Best Practices

1. **Test locally first** - Run `./test_auto_update.sh`
2. **Monitor logs** - Check `logs/auto_update.log`
3. **Health checks** - Verify services after deployment
4. **Backup strategy** - Have rollback plan ready
5. **Security** - Keep SSH keys secure

## 📞 Support

- **Documentation**: `docs/GITHUB_ACTIONS_AUTO_DEPLOY.md`
- **Test script**: `./test_auto_update.sh`
- **Logs**: `tail -f logs/auto_update.log`
- **Docker status**: `docker-compose ps`

---

*Updated for GitHub Actions compatibility - $(date)*
