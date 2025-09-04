# GitHub Actions Auto Deploy Setup Guide

This guide explains how to set up automated deployment using GitHub Actions with the updated `auto_update.sh` script.

## Overview

The updated system allows GitHub Actions to automatically deploy your FKS application to your server whenever you push changes to the main branch. The deployment process:

1. GitHub Actions triggers on push to main branch
2. Connects to your server via SSH as root user
3. Runs the `auto_update.sh` script which:
   - Pulls latest code from GitHub
   - Stops existing Docker services
   - Pulls latest Docker images
   - Rebuilds and starts services
   - Logs the entire process

## Prerequisites

1. A server with Docker and Docker Compose installed
2. SSH access to your server as root user
3. Your FKS repository cloned to `/root/fks` on the server
4. GitHub repository with Actions enabled

## Setup Instructions

### 1. Server Setup

On your server, ensure the repository is in the correct location:

```bash
# As root user
cd /root
git clone https://github.com/yourusername/fks.git
cd fks
chmod +x auto_update.sh
chmod +x start.sh
```

### 2. GitHub Secrets Configuration

Add the following secrets to your GitHub repository:

- Go to your repository → Settings → Secrets and variables → Actions
- Add these repository secrets:

| Secret Name | Description | Example |
|-------------|-------------|---------|
| `SERVER_HOST` | Your server's public IP address | `192.168.1.100` |
| `SERVER_SSH_KEY` | Private SSH key for root access | Contents of your private key file |

### 3. SSH Key Setup

If you don't have SSH keys set up:

```bash
# On your local machine or GitHub Actions runner
ssh-keygen -t ed25519 -C "actions_user@yourproject.com"

# Copy the public key to your server
ssh-copy-id -i ~/.ssh/id_ed25519.pub root@your-server-ip

# Add the private key content to GitHub Secrets as SERVER_SSH_KEY
cat ~/.ssh/id_ed25519
```

### 4. Test the Setup

1. Push a change to your main branch
2. Check the Actions tab in your GitHub repository
3. Monitor the deployment process
4. Check the logs on your server: `/root/fks/logs/auto_update.log`

## How It Works

### Auto Update Script Features

The updated `auto_update.sh` script includes:

- **Environment Detection**: Automatically detects if running as root (GitHub Actions) or regular user
- **Docker Integration**: Prefers Docker Compose for service management
- **Fallback Support**: Falls back to `start.sh` if Docker isn't available
- **Comprehensive Logging**: Detailed logs with timestamps and process IDs
- **Error Handling**: Proper error handling and cleanup
- **Lock File Protection**: Prevents multiple instances from running

### GitHub Actions Workflow

The workflow (`.github/workflows/auto-deploy.yml`):

- Triggers on push to main branch or manual dispatch
- Uses `appleboy/ssh-action` for secure SSH connection
- Runs the auto update script on your server
- Provides deployment status feedback

## Troubleshooting

### Common Issues

1. **SSH Connection Failed**
   - Verify `SERVER_HOST` and `SERVER_SSH_KEY` secrets
   - Test SSH connection manually: `ssh root@your-server-ip`

2. **Permission Denied**
   - Ensure `auto_update.sh` is executable: `chmod +x auto_update.sh`
   - Check file ownership: `chown root:root auto_update.sh`

3. **Docker Services Not Starting**
   - Check Docker is running: `systemctl status docker`
   - Verify docker-compose.yml exists in repository root
   - Check logs: `tail -f /root/fks/logs/auto_update.log`

4. **Git Issues**
   - Ensure repository is properly cloned
   - Check git configuration: `git config --list`

### Log Files

Monitor these log files for troubleshooting:

- **Auto Update Log**: `/root/fks/logs/auto_update.log`
- **Docker Logs**: `docker-compose logs -f`
- **System Logs**: `journalctl -u docker`

### Manual Testing

Test the auto update script manually:

```bash
# On your server as root
cd /root/fks
./auto_update.sh
```

## Security Considerations

1. **SSH Key Security**: Store SSH private keys securely in GitHub Secrets
2. **Server Access**: Limit SSH access to necessary IP addresses
3. **Docker Security**: Keep Docker images updated and use official images
4. **Log Monitoring**: Monitor logs for any suspicious activity

## Customization

### Environment Variables

You can customize the script behavior by modifying these variables in `auto_update.sh`:

- `REPO_DIR`: Repository directory path
- `LOG_FILE`: Log file location
- `LOCK_FILE`: Lock file location

### Docker Compose Files

The script automatically detects and uses:
- `docker-compose.yml` (primary)
- `docker-compose.override.yml` (if exists)
- `docker-compose.prod.yml` (if exists)

## Monitoring

### GitHub Actions Status

Monitor deployment status:
- Repository → Actions tab
- Email notifications for failed deployments
- Slack/Discord webhooks for deployment notifications

### Server Monitoring

Set up monitoring for:
- Docker container health
- Disk space usage
- Memory and CPU usage
- Application logs

## Best Practices

1. **Test Locally**: Always test changes locally before pushing
2. **Staged Deployments**: Consider using staging environments
3. **Backup Strategy**: Implement database and file backups
4. **Health Checks**: Add health checks to your services
5. **Rollback Plan**: Have a rollback strategy for failed deployments

## Example Usage

After setup, your deployment workflow becomes:

```bash
# 1. Make changes to your code
git add .
git commit -m "Add new feature"

# 2. Push to main branch
git push origin main

# 3. GitHub Actions automatically deploys to server
# 4. Monitor progress in Actions tab
# 5. Check server logs if needed
```

That's it! Your FKS application will automatically deploy whenever you push changes to the main branch.
