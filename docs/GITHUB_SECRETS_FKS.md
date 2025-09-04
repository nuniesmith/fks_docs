# GitHub Secrets Required for FKS Deployment

This document lists all the required GitHub secrets for the FKS project deployment workflows.

## Container Registry Access
```
GITHUB_TOKEN
# Auto-provided by GitHub Actions for GHCR access
```

## Tailscale VPN Configuration
```
TAILSCALE_OAUTH_CLIENT_ID
# OAuth client ID from Tailscale Admin Console > Settings > OAuth clients

TAILSCALE_OAUTH_SECRET  
# OAuth secret from Tailscale Admin Console > Settings > OAuth clients
```

## Linode Cloud Server (Production)
```
LINODE_HOST
# Tailscale IP of your Linode server (e.g., 100.64.x.x)

LINODE_SSH_PRIVATE_KEY
# SSH private key for root access to Linode server
# Generate with: ssh-keygen -t ed25519 -C "github-actions-fks"

LINODE_KNOWN_HOSTS
# SSH known hosts entry for Linode server
# Get with: ssh-keyscan -H [LINODE_IP] >> known_hosts
```

## Home Servers
```
SULLIVAN_HOST
# Tailscale IP of Sullivan server (e.g., 100.64.x.x)

SULLIVAN_SSH_PRIVATE_KEY
# SSH private key for jordan user on Sullivan
# Generate with: ssh-keygen -t ed25519 -C "github-actions-sullivan"

FREDDY_HOST
# Tailscale IP of Freddy server (e.g., 100.64.x.x)

FREDDY_SSH_PRIVATE_KEY
# SSH private key for jordan user on Freddy  
# Generate with: ssh-keygen -t ed25519 -C "github-actions-freddy"
```

## Optional: Cloudflare DNS (if using)
```
CLOUDFLARE_API_TOKEN
# API token with Zone:Edit permissions for 7gram.xyz

CLOUDFLARE_ZONE_ID
# Zone ID for 7gram.xyz domain
```

## Setup Instructions

### 1. Tailscale OAuth Setup
1. Go to [Tailscale Admin Console](https://login.tailscale.com/admin/settings/oauth)
2. Create new OAuth client with tags: `tag:github-actions`
3. Copy Client ID and Secret to GitHub secrets

### 2. SSH Key Generation
```bash
# For each server, generate dedicated keys:
ssh-keygen -t ed25519 -C "github-actions-linode" -f ~/.ssh/fks_linode
ssh-keygen -t ed25519 -C "github-actions-sullivan" -f ~/.ssh/fks_sullivan  
ssh-keygen -t ed25519 -C "github-actions-freddy" -f ~/.ssh/fks_freddy

# Copy public keys to servers:
ssh-copy-id -i ~/.ssh/fks_linode.pub root@[LINODE_IP]
ssh-copy-id -i ~/.ssh/fks_sullivan.pub jordan@[SULLIVAN_IP]
ssh-copy-id -i ~/.ssh/fks_freddy.pub jordan@[FREDDY_IP]

# Add private keys to GitHub secrets (copy entire file content)
```

### 3. Server Preparation

**Linode Server:**
```bash
# Install Docker and Docker Compose
curl -fsSL https://get.docker.com | sh
systemctl enable --now docker

# Create deployment directory
mkdir -p /opt/fks/{backup,logs}
chown -R root:root /opt/fks

# Install Tailscale
curl -fsSL https://tailscale.com/install.sh | sh
tailscale up --auth-key=[AUTH_KEY]
```

**Home Servers (Sullivan/Freddy):**
```bash
# Clone repository
cd ~
git clone https://github.com/jordan7590/repos.git
cd repos/fks

# Create backup directory
mkdir -p backup

# Ensure Docker is installed and user has access
sudo usermod -aG docker jordan
```

### 4. Environment Configuration

Create `/opt/fks/.env` on Linode with:
```env
# Copy from .env.example and customize:
DOMAIN=fks.7gram.xyz
DATABASE_URL=postgresql://fks:password@db:5432/fks
REDIS_URL=redis://redis:6379
API_SECRET_KEY=your-secret-key-here
```

### 5. GitHub Secrets Setup
1. Go to your repository > Settings > Secrets and variables > Actions
2. Add all the secrets listed above
3. Test deployment with workflow dispatch

## Workflow Triggers

- **Push to main**: Deploys to all servers
- **Push to develop**: Builds and tests only  
- **Pull Request**: Tests only
- **Manual dispatch**: Choose deployment target

## Monitoring

After deployment, check:
- https://fks.7gram.xyz (production)
- Tailscale IPs for home servers
- Docker logs: `docker-compose logs -f`
- Health checks in workflow output

## Troubleshooting

**SSH Connection Issues:**
```bash
# Test SSH access:
ssh -i ~/.ssh/fks_linode root@[LINODE_IP]
ssh -i ~/.ssh/fks_sullivan jordan@[SULLIVAN_IP]

# Check Tailscale connectivity:
tailscale ping [TARGET_IP]
```

**Docker Issues:**
```bash
# Check container status:
docker-compose ps
docker-compose logs [service-name]

# Restart services:
docker-compose restart
```

**Deployment Rollback:**
```bash
# On server:
cd /opt/fks
cp backup/docker-compose.yml.[timestamp] docker-compose.yml
docker-compose up -d
```
