# Ninja Trading System - Linode Deployment Guide

## Overview

This guide provides step-by-step instructions for deploying your Ninja Trading System on Linode using the provided StackScript.

## Prerequisites

### 1. Linode Account Setup
- Create a Linode account at [linode.com](https://www.linode.com)
- Add billing information and verify your account
- Generate an API token (optional, for CLI management)

### 2. GitHub Preparation
- Ensure your repository is accessible (public or with proper token)
- Generate a GitHub Personal Access Token with repo access:
  1. Go to GitHub Settings → Developer settings → Personal access tokens
  2. Generate new token with `repo` scope
  3. Copy the token (starts with `ghp_`)

### 3. SSH Key Setup (Recommended)
- Generate an SSH key pair if you don't have one:
  ```bash
  ssh-keygen -t rsa -b 4096 -C "your_email@example.com"
  ```
- Copy your public key content (~/.ssh/id_rsa.pub)

## Recommended Linode Plans

| Plan | vCPUs | RAM | Storage | Monthly Cost | Use Case |
|------|-------|-----|---------|-------------|----------|
| **Nanode 1GB** | 1 | 1GB | 25GB | $5 | Testing only |
| **Linode 2GB** | 1 | 2GB | 50GB | $10 | Light development |
| **Linode 4GB** ⭐ | 2 | 4GB | 80GB | $20 | **Recommended for production** |
| **Linode 8GB** | 4 | 8GB | 160GB | $40 | Heavy trading loads |

⭐ **Recommended**: Linode 4GB plan provides optimal balance of performance and cost for most trading scenarios.

## Deployment Steps

### Step 1: Create StackScript

1. Log into Linode Cloud Manager
2. Navigate to **StackScripts** → **Create StackScript**
3. Fill in the details:
   - **StackScript Label**: `ninja-trading-system`
   - **Description**: `Automated deployment for Ninja Trading System`
   - **Target Images**: Select `Ubuntu 22.04 LTS`, `Debian 11`, `Arch Linux`
   - **Script**: Copy the entire content from `scripts/StackScript.sh`
4. Click **Create StackScript**

### Step 2: Deploy Linode Instance

1. Go to **Linodes** → **Create Linode**
2. Select **StackScripts** tab
3. Choose your **Account StackScripts**
4. Select the `ninja-trading-system` StackScript
5. Configure the StackScript fields:

#### Required Fields:
- **GitHub Repository**: `nuniesmith/ninja` (or your fork)
- **GitHub Personal Access Token**: Your GitHub token (ghp_...)
- **SSH Public Key**: Your SSH public key content
- **Ninja User Password**: Set a secure password
- **VNC Password**: Set a secure VNC password

#### Optional Fields:
- **Tailscale Auth Key**: For VPN access (recommended for production)
- **Domain Name**: If you have a domain for the system
- **Discord Webhook URL**: For deployment notifications

6. **Select Image**: Ubuntu 22.04 LTS (recommended)
7. **Select Region**: Choose closest to your location or target market
8. **Select Linode Plan**: Linode 4GB (recommended)
9. **Set Root Password**: Strong password for emergency access
10. **Add SSH Keys**: Select your SSH key from the list
11. **Click Create Linode**

### Step 3: Monitor Deployment

1. The Linode will boot and start the StackScript automatically
2. Monitor progress in the **Lish Console**:
   - Click on your Linode
   - Go to **Lish Console** tab
   - Watch the deployment logs in real-time

3. Deployment phases:
   - **Phase 1**: System updates and package installation (~5-10 minutes)
   - **Reboot**: System will automatically reboot
   - **Phase 2**: Repository clone and application setup (~5-10 minutes)

4. Total deployment time: **15-20 minutes**

### Step 4: Access Your System

Once deployment completes, you'll have access to:

#### Web Interfaces:
- **Trading Web Interface**: `http://YOUR_IP:3000`
- **VNC Web Access**: `http://YOUR_IP:6080` (password: your VNC password)
- **VS Code Server**: `http://YOUR_IP:8081` (password: fksdev123)
- **Python API**: `http://YOUR_IP:8002`

#### SSH Access:
```bash
ssh ninja@YOUR_IP
```

#### VNC Direct Access:
- Host: `YOUR_IP:5900`
- Password: Your VNC password

## Post-Deployment Configuration

### 1. Verify Services
```bash
# Check all services
sudo systemctl status ninja-trading
sudo systemctl status docker
docker-compose ps

# View logs
docker-compose logs -f
```

### 2. Update Domain Configuration (Optional)
If you provided a domain name, configure your DNS:
```
A record: your-domain.com → YOUR_LINODE_IP
```

### 3. Set Up SSL (Optional)
For production use with domain:
```bash
# Install certbot
sudo apt install certbot

# Get SSL certificate
sudo certbot certonly --standalone -d your-domain.com
```

### 4. Configure Tailscale (Recommended for Production)
If you didn't set up Tailscale during deployment:
```bash
# Install Tailscale
curl -fsSL https://tailscale.com/install.sh | sh

# Authenticate
sudo tailscale up
```

## Security Best Practices

### 1. Change Default Passwords
```bash
# Change ninja user password
sudo passwd ninja

# Update VNC password
vncpasswd
```

### 2. Configure SSH Security
```bash
# Edit SSH config
sudo nano /etc/ssh/sshd_config

# Recommended settings:
# PermitRootLogin no
# PasswordAuthentication no  # Only if using SSH keys
# PubkeyAuthentication yes

# Restart SSH
sudo systemctl restart sshd
```

### 3. Set Up Automated Backups
```bash
# Create backup script
sudo nano /usr/local/bin/ninja-backup

# Add to crontab
crontab -e
# Add: 0 2 * * * /usr/local/bin/ninja-backup
```

## Monitoring and Maintenance

### 1. System Health Monitoring
The StackScript installs automatic monitoring:
- System resource checks every 5 minutes
- Docker container health monitoring
- Logs stored in `/var/log/ninja-monitor.log`

### 2. Update Management
```bash
# Update system packages
sudo apt update && sudo apt upgrade

# Update application
cd /opt/ninja-trading
git pull origin main
docker-compose build
docker-compose up -d
```

### 3. Log Management
Key log locations:
- Setup logs: `/var/log/ninja-setup/`
- Application logs: `docker-compose logs`
- System monitor: `/var/log/ninja-monitor.log`
- Configuration: `/etc/ninja-trading/deployment-config.json`

## Troubleshooting

### Common Issues:

#### 1. Services Not Starting
```bash
# Check Docker status
sudo systemctl status docker

# Check application status
cd /opt/ninja-trading
docker-compose ps
docker-compose logs
```

#### 2. Network Connectivity Issues
```bash
# Check firewall status
sudo ufw status

# Check if ports are listening
sudo netstat -tlnp
```

#### 3. Memory Issues
```bash
# Check memory usage
free -h
docker stats

# If low memory, consider upgrading Linode plan
```

#### 4. Repository Access Issues
```bash
# Verify GitHub token
cd /opt/ninja-trading
git pull

# If fails, update token in git credentials
```

### Getting Support:

1. **Check deployment logs**: `/var/log/ninja-setup/setup-*.log`
2. **Discord notifications**: If configured, check webhook channel
3. **Linode Support**: Use Linode's support system for infrastructure issues
4. **GitHub Issues**: Report application-specific issues on GitHub

## Cost Optimization

### 1. Resource Monitoring
Monitor your usage to optimize costs:
- Use Linode's graphs to track CPU/memory usage
- Consider downgrading if consistently under-utilized
- Upgrade only when needed for performance

### 2. Backup Strategy
- Use Linode Backups ($2/month for 4GB plan) for automated backups
- Or implement custom backup scripts to object storage

### 3. Network Transfer
- Monitor bandwidth usage
- Linode includes generous transfer allowances
- Consider using Tailscale to reduce public internet traffic

## Advanced Configuration

### 1. High Availability Setup
For production trading:
- Deploy multiple Linodes in different regions
- Use Linode's NodeBalancer for load distribution
- Implement database clustering

### 2. Performance Optimization
- Enable Linode's SSD storage
- Use Linode's private networking for multi-node setups
- Optimize Docker container resources

### 3. Monitoring Integration
- Connect to external monitoring services
- Set up alerting for trading system issues
- Implement automated failover procedures

---

## Quick Reference Commands

```bash
# Service management
sudo systemctl status ninja-trading
sudo systemctl restart ninja-trading
sudo systemctl start ninja-trading
sudo systemctl stop ninja-trading

# Docker management
docker-compose ps
docker-compose logs -f
docker-compose restart
docker-compose down && docker-compose up -d

# System monitoring
htop
df -h
free -h
systemctl status

# Log viewing
tail -f /var/log/ninja-setup/setup-*.log
docker-compose logs -f
journalctl -u ninja-trading -f
```

This guide should give you everything you need for a successful Linode deployment!
