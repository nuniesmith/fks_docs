# FKS Trading Systems - GitHub Actions Deployment Guide

## 🚀 Automated Linode Server Deployment

The FKS Trading Systems now includes automated infrastructure provisioning using GitHub Actions and Linode. This setup will create a server named **fks-dev** with hostname **fks** in **Toronto, Canada**.

## 📋 Server Specifications

- **Server Name:** fks-dev
- **Hostname:** fks  
- **Location:** Toronto, Canada (ca-central region)
- **Instance Type:** g6-standard-2 (4GB RAM, 2 CPUs, 80GB Storage)
- **Operating System:** Ubuntu 24.04 LTS
- **Backups:** Enabled
- **Cost:** ~$24/month + $2.4/month for backups

## 🔑 Required GitHub Secrets

Before running the deployment, ensure these secrets are configured in your GitHub repository:

### Linode Configuration
```
LINODE_CLI_TOKEN         # Your Linode API token
FKS_DEV_ROOT_PASSWORD     # Root password for new servers
```

### User Passwords
```
JORDAN_PASSWORD          # Password for jordan user
FKS_USER_PASSWORD        # Password for fks_user
```

### Docker Registry
```
DOCKER_USERNAME          # Docker Hub username
DOCKER_TOKEN             # Docker Hub access token
```

### Tailscale (VPN)
```
TAILSCALE_AUTH_KEY       # Tailscale auth key for VPN access
```

### Optional Monitoring
```
NETDATA_CLAIM_TOKEN      # Netdata Cloud claim token (optional)
NETDATA_CLAIM_ROOM       # Netdata Cloud room ID (optional)
```

### SSH Access
```
SSH_PRIVATE_KEY          # SSH private key for server access
```

## 🎯 Deployment Modes

### 1. Full Deployment
Creates infrastructure + builds + deploys application
```bash
# Workflow Dispatch Input:
deployment_mode: full-deploy
create_new_server: true  # Creates new fks-dev server
```

### 2. Infrastructure Only
Creates only the Linode server without application deployment
```bash
# Workflow Dispatch Input:
deployment_mode: infra-only
```

### 3. Application Only
Deploys application to existing server
```bash
# Workflow Dispatch Input:
deployment_mode: deploy-only
```

### 4. Builds Only
Only builds Docker images without deployment
```bash
# Workflow Dispatch Input:
deployment_mode: builds-only
```

## 🌐 Service Endpoints

Once deployed, your services will be available at:

- **Web Interface:** `http://[SERVER_IP]/`
- **API Health:** `http://[SERVER_IP]/health`
- **Direct API:** `http://[SERVER_IP]:8000/health`
- **Direct Web:** `http://[SERVER_IP]:3000/`

## 🔧 Server Configuration

The StackScript automatically configures:

### System Setup
- ✅ Hostname set to `fks`
- ✅ Timezone set to `America/Toronto`
- ✅ Docker and Docker Compose installed
- ✅ User accounts created (jordan, fks_user)
- ✅ SSH keys configured

### Security
- ✅ Tailscale VPN configured
- ✅ Firewall configured (UFW)
- ✅ SSH key-based authentication
- ✅ Automatic security updates

### Monitoring
- ✅ Netdata monitoring (optional)
- ✅ Docker monitoring
- ✅ System health checks

## 📱 Quick Start

1. **Configure GitHub Secrets** (see above list)

2. **Run Workflow**
   - Go to Actions tab in GitHub
   - Select "FKS Trading Systems - Optimized Deployment Pipeline"
   - Click "Run workflow"
   - Choose "full-deploy" mode
   - Enable "create_new_server"

3. **Wait for Completion** (~10-15 minutes)
   - Infrastructure provisioning: ~5 minutes
   - Docker builds: ~5 minutes  
   - Application deployment: ~2 minutes

4. **Access Your Services**
   - Check the workflow summary for IP address
   - Test endpoints listed above

## 🔍 Troubleshooting

### Server Creation Issues
```bash
# Check Linode CLI token
curl -H "Authorization: Bearer $LINODE_CLI_TOKEN" \
  https://api.linode.com/v4/profile

# Verify region availability
linode-cli regions list
```

### Deployment Issues
```bash
# SSH into server
ssh jordan@[SERVER_IP]

# Check Docker services
docker compose ps
docker compose logs

# Check system status
sudo systemctl status docker
sudo systemctl status tailscaled
```

### Common Issues
- **Server creation fails:** Check LINODE_CLI_TOKEN and account limits
- **SSH connection fails:** Verify SSH_PRIVATE_KEY matches configured public keys
- **Application fails to start:** Check Docker logs and ensure all secrets are set

## 📚 Additional Resources

- [Linode API Documentation](https://www.linode.com/api/v4/)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Docker Compose Documentation](https://docs.docker.com/compose/)
- [Tailscale Setup Guide](https://tailscale.com/kb/installation/)

## 🔄 Server Management

### Update Application
```bash
# Automatically handled by workflow
# Manual update:
cd /opt/fks
git pull origin main
docker-compose pull
docker-compose up -d --remove-orphans
```

### Scale Resources
```bash
# Resize server via Linode CLI
linode-cli linodes resize [SERVER_ID] --type g6-standard-4
```

### Backup Management
```bash
# Backups are automatically enabled
# Manual snapshot:
linode-cli linodes snapshot [SERVER_ID] --label "manual-backup-$(date +%Y%m%d)"
```

---

**Note:** This setup is optimized for development and staging environments. For production, consider additional security hardening, monitoring, and backup strategies.
