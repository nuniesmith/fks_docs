# GitHub Actions Setup for Ninja Trading System

## Overview

The GitHub Actions workflow has been updated to deploy to your Ninja server using Tailscale for secure networking. This eliminates the need for exposing SSH over the public internet.

**Note**: If you see "Context access might be invalid" warnings in VS Code for secrets, this is normal behavior when secrets haven't been configured yet in the repository. These warnings will disappear once the secrets are properly set up.

## Required GitHub Secrets

You need to set up the following secrets in your GitHub repository:

### Go to: `Settings` → `Secrets and variables` → `Actions` → `Repository secrets`

### 1. Tailscale Authentication
```
TAILSCALE_OAUTH_CLIENT_ID
TAILSCALE_OAUTH_SECRET
```

**How to get these**:
1. Go to [Tailscale Admin Console](https://login.tailscale.com/admin)
2. Navigate to `Settings` → `OAuth clients`
3. Click `Generate OAuth client`
4. Set the scopes to include `devices:write` and `all:read`
5. Add the tag `tag:ci-cd` to the client
6. Copy the Client ID and Client Secret

### 2. SSH Authentication
```
NINJA_SSH_PRIVATE_KEY
```

**How to get this**:
```bash
# On your local machine, generate an SSH key pair for CI/CD
ssh-keygen -t ed25519 -f ~/.ssh/ninja-ci-cd -N ""

# Copy the public key to your ninja server
ssh-copy-id -i ~/.ssh/ninja-ci-cd.pub ninja@ninja.tailfef10.ts.net

# The private key content goes into the GitHub secret
cat ~/.ssh/ninja-ci-cd
```

### 3. Discord Notifications (Optional)
```
DISCORD_WEBHOOK
```

**How to get this**:
1. In your Discord server, go to Server Settings → Integrations → Webhooks
2. Create a new webhook or use an existing one
3. Copy the webhook URL

### 4. Repository Variables (Optional)

Go to: `Settings` → `Secrets and variables` → `Actions` → `Repository variables`

```
ENABLE_DISCORD_NOTIFICATIONS = true
```

## Updated Workflow Features

### 🔒 **Tailscale Integration**
- Uses Tailscale OAuth for secure network access
- No need to expose SSH ports publicly
- Connects to `ninja.tailfef10.ts.net` directly

### 🚀 **Smart Deployment**
- Deploys to `/home/ninja/ninja` (matches StackScript setup)
- Handles cases where Docker might not be running
- Graceful error handling with informative messages

### 🔄 **Deployment Environments**
- **Staging**: Automatic on `main` branch pushes
- **Production**: Manual workflow dispatch only

### 🛡️ **Safety Features**
- Creates backups before production deployments
- Verifies services are running before declaring success
- Non-blocking health checks (warns instead of failing)

## Workflow Triggers

### Automatic Triggers
```yaml
on:
  push:
    branches: [ main, develop ]  # Tests run on all pushes
  pull_request:
    branches: [ main ]           # Tests run on PRs to main
```

### Manual Deployment
```yaml
workflow_dispatch:
  inputs:
    deploy_environment:
      type: choice
      options: [staging, production]
```

## Deployment Process

### 1. **Tests** (Always Run)
- Python component tests
- .NET component tests  
- React component tests

### 2. **Build Images** (On `main` branch)
- Builds Docker images for all services
- Pushes to GitHub Container Registry

### 3. **Deploy to Staging** (On `main` branch or manual)
- Connects via Tailscale
- Updates code from git
- Restarts Docker services (if Docker is running)

### 4. **Deploy to Production** (Manual only)
- Requires staging deployment to succeed first
- Creates backup before deployment
- Comprehensive health checks

## Manual Deployment Commands

### Deploy to Staging
```bash
# Via GitHub CLI
gh workflow run deploy-workflow.yml -f deploy_environment=staging

# Via GitHub Web UI
# Go to Actions → FKS Trading Systems CI/CD → Run workflow
# Select "staging" environment
```

### Deploy to Production
```bash
# Via GitHub CLI  
gh workflow run deploy-workflow.yml -f deploy_environment=production

# Via GitHub Web UI
# Go to Actions → FKS Trading Systems CI/CD → Run workflow
# Select "production" environment
```

## Troubleshooting

### SSH Connection Issues
```bash
# Test SSH connection manually
ssh ninja@ninja.tailfef10.ts.net

# Verify Tailscale is running on the server
sudo tailscale status

# Check if the ninja user can access the repository
sudo -u ninja git pull origin main
```

### Docker Issues
```bash
# Check Docker status on the server
systemctl status docker

# Start Docker if needed
sudo systemctl start docker

# Manually run deployment
cd /home/ninja/ninja
docker-compose up -d
```

### Permission Issues
```bash
# Ensure ninja user owns the repository
sudo chown -R ninja:ninja /home/ninja/ninja

# Verify ninja user is in docker group
groups ninja
```

## Security Notes

### 🔐 **SSH Key Security**
- Use a dedicated SSH key pair for CI/CD
- Limit the key to the ninja user only
- Regularly rotate SSH keys

### 🌐 **Tailscale Security**
- OAuth client is scoped to necessary permissions only
- CI/CD runners are tagged for easy identification
- Network access is isolated through Tailscale

### 🚫 **What's NOT Exposed**
- No public SSH ports
- No public Docker ports
- All communication through Tailscale mesh

## Monitoring Deployment

### GitHub Actions UI
- Go to `Actions` tab in your repository
- Click on the running workflow
- Monitor real-time logs

### Discord Notifications
- Success/failure notifications sent to Discord
- Includes branch, commit, and environment info

### Server Logs
```bash
# Check deployment logs on the server
tail -f /var/log/ninja-setup/*.log

# Check Docker logs
docker-compose logs -f

# Check system services
journalctl -f -u ninja-trading.service
```

This setup provides a secure, automated deployment pipeline that works seamlessly with your Tailscale-enabled ninja server!
