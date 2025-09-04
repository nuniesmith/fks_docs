# 🚀 FKS Linode Automation - Quick Reference

## Required Secrets Setup

Run this to set up secrets interactively:
```bash
chmod +x scripts/deployment/actions_user/setup-github-secrets.sh
./scripts/deployment/actions_user/setup-github-secrets.sh
```

Or set manually in GitHub: `Repository → Settings → Secrets and variables → Actions`

| Secret | Purpose | Required |
|--------|---------|----------|
| `LINODE_CLI_TOKEN` | Linode API access | ✅ |
| `FKS_DEV_ROOT_PASSWORD` | Server root password | ✅ |
| `JORDAN_PASSWORD` | Main user password | ✅ |
| `FKS_USER_PASSWORD` | FKS user password | ✅ |
| `TAILSCALE_AUTH_KEY` | VPN access key | ✅ |
| `DOCKER_USERNAME` | Docker Hub username | ⚠️ |
| `DOCKER_TOKEN` | Docker Hub token | ⚠️ |
| `ACTIONS_ROOT_PRIVATE_KEY` | Deployment key | ⚠️ |
| `DISCORD_WEBHOOK_SERVERS` | Notifications | ⚠️ |

## 🖥️ Server Creation

### GitHub UI
1. Go to **Actions** tab
2. Select **"Create Linode Server with Arch Linux"**
3. Click **"Run workflow"**
4. Configure:
   - **Server name**: `fks`
   - **Plan**: `g6-standard-2` (4GB, $24/month)
   - **Region**: `ca-central` (Toronto)
   - **OS**: `linode/arch`
   - **Backups**: `true` (+$2.4/month)

### GitHub CLI
```bash
gh workflow run create-linode-server.yml \
  -f server_name=fks \
  -f linode_type=g6-standard-2 \
  -f linode_region=ca-central \
  -f linode_image=linode/arch \
  -f enable_backups=true
```

## 🚀 Deployment

### Automatic
- Triggers after successful Docker image builds
- Deploys to your FKS server automatically

### Manual
```bash
gh workflow run deploy-linode.yml \
  -f target_server=fks \
  -f use_tailscale=true \
  -f force_deploy=true
```

## 📊 Server Plans & Costs

| Plan | RAM | CPUs | Storage | Monthly | Use Case |
|------|-----|------|---------|---------|----------|
| `g6-standard-1` | 2GB | 1 | 50GB | $12 | Development |
| `g6-standard-2` | 4GB | 2 | 80GB | $24 | **Recommended** |
| `g6-standard-4` | 8GB | 4 | 160GB | $48 | Production |

**Additional Costs:**
- Backups: +10% of server cost
- Tailscale: Free (up to 100 devices)

## 🔒 Access Methods

### SSH Access
```bash
# Public IP (emergency access)
ssh jordan@<public-ip>

# Tailscale IP (secure, recommended)
ssh jordan@<tailscale-ip>
```

### Web Interfaces (via Tailscale)
```
http://<tailscale-ip>:3000   # Web UI
http://<tailscale-ip>:8000   # API
http://<tailscale-ip>:19999  # Monitoring
```

## 🛠️ Common Commands

After SSH to server:

```bash
# Navigate to project
cd ~/fks

# Check services
docker compose ps
system-status  # Custom command

# View logs
docker compose logs -f
tail -f /var/log/fks-setup.log

# Restart services
docker compose restart
./start.sh

# Update code and restart
git pull
docker compose down
docker compose up -d

# Check Tailscale
tailscale status
tailscale ip
```

## ⚡ Quick Troubleshooting

| Issue | Check | Solution |
|-------|-------|----------|
| Server creation fails | Linode API token | Verify token permissions |
| SSH connection fails | Server status | Wait for setup completion |
| Services not starting | Docker status | `sudo systemctl restart docker` |
| Tailscale not working | Auth key | Check key validity/expiration |
| Deployment fails | Repository | Ensure repo cloned: `git clone <url>` |

## 📍 Important Files

| File | Purpose |
|------|---------|
| `/var/log/fks-setup.log` | StackScript setup logs |
| `/var/log/fks-deployment.log` | Deployment logs |
| `~/fks/` | Main project directory |
| `~/.ssh/authorized_keys` | SSH access keys |

## 🆘 Support Commands

```bash
# Check setup status
tail -f /var/log/fks-setup.log

# Monitor Phase 2 setup
sudo journalctl -u fks-phase2.service -f

# System health
htop
df -h
free -h

# Network status
ss -tuln
iptables -L -n
```

## 🎯 Workflow Summary

1. **Setup secrets** → Run `scripts/deployment/actions_user/setup-github-secrets.sh`
2. **Create server** → Run "Create Linode Server" workflow
3. **Wait 15-20 min** → Monitor in Actions tab
4. **SSH to server** → `ssh jordan@<ip>`
5. **Clone repo** → `git clone <your-repo>`
6. **Start services** → `cd fks && ./start.sh`
7. **Deploy updates** → "Deploy to Linode" workflow runs automatically

**Total setup time: ~20 minutes**
**Monthly cost: ~$26 (server + backups)**

---

📖 **Full Documentation**: [docs/LINODE_AUTOMATION_GUIDE.md](./LINODE_AUTOMATION_GUIDE.md)
