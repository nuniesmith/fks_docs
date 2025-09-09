# FKS Trading Systems - Complete Deployment Guide

## 🎯 **OVERVIEW**

This guide provides complete instructions for deploying your FKS Trading Systems on Linode with automatic GitHub Actions CI/CD integration.

---

## 📋 **PREREQUISITES**

### **1. Linode Account Setup**
- Create a Linode account at [linode.com](https://www.linode.com)
- Add billing information and verify your account
- Generate an API token (optional, for CLI management)

### **2. GitHub Preparation**
- Ensure your repository is accessible (public or with proper token)
- Generate a GitHub Personal Access Token with repo access:
  1. Go to GitHub Settings → Developer settings → Personal access tokens
  2. Generate new token (classic) with `repo` scope
  3. Copy the token (starts with `ghp_`)

### **3. SSH Key Setup (Recommended)**
Generate an SSH key pair if you don't have one:
```bash
ssh-keygen -t ed25519 -f ~/.ssh/ninja-trading -N ""
```
Copy your public key content (`~/.ssh/ninja-trading.pub`)

### **4. Tailscale Account (Optional but Recommended)**
- Create account at [tailscale.com](https://tailscale.com)
- Generate auth key in admin console for VPN access

---

## 🔧 **RECOMMENDED LINODE PLANS**

| Plan | vCPUs | RAM | Storage | Monthly Cost | Use Case |
|------|-------|-----|---------|-------------|----------|
| **Nanode 1GB** | 1 | 1GB | 25GB | $5 | Testing only |
| **Linode 2GB** | 1 | 2GB | 50GB | $10 | Light development |
| **Linode 4GB** ⭐ | 2 | 4GB | 80GB | $20 | **Recommended for production** |
| **Linode 8GB** | 4 | 8GB | 160GB | $40 | Heavy trading loads |

⭐ **Recommended**: Linode 4GB plan provides optimal balance of performance and cost.

---

## 🚀 **DEPLOYMENT STEPS**

### **Step 1: Create StackScript**

1. Log into Linode Cloud Manager
2. Navigate to **StackScripts** → **Create StackScript**
3. Fill in the details:
   - **StackScript Label**: `ninja-trading-system`
   - **Description**: `FKS Trading Systems Deployment`
   - **Script**: Copy the entire content from `scripts/StackScript.sh`
4. Click **Create StackScript**

### **Step 2: Deploy Linode Instance**

1. Go to **Linodes** → **Create Linode**
2. Select **StackScripts** tab
3. Choose your **Account StackScripts**
4. Select the `ninja-trading-system` StackScript

### **Step 3: Configure StackScript Fields**

#### **Required Fields:**
- **GitHub Repository**: `nuniesmith/ninja` (or your fork)
- **GitHub Personal Access Token**: Your GitHub token (ghp_...)
- **SSH Public Key**: Your SSH public key content
- **Ninja User Password**: Set a secure password
- **VNC Password**: Set a secure VNC password

#### **Optional Fields:**
- **Tailscale Auth Key**: For VPN access (recommended for production)
- **Domain Name**: If you have a domain for the system
- **Discord Webhook URL**: For deployment notifications

### **Step 4: Configure Linode Settings**

6. **Select Image**: Ubuntu 22.04 LTS (recommended)
7. **Select Region**: Choose closest to your location or target market
8. **Select Linode Plan**: Linode 4GB (recommended)
9. **Set Root Password**: Strong password for emergency access
10. **Add SSH Keys**: Select your SSH key from the list
11. **Click Create Linode**

---

## 📊 **TWO-STAGE DEPLOYMENT PROCESS**

### **Stage 1: System Foundation (Pre-Reboot)**
*Duration: ~5-10 minutes*

**What Happens:**
1. **System Updates**: Package manager updates and essential package installation
2. **Service Installation**: Docker, .NET SDK, SSH, and other core services
3. **User Creation**: Creates `ninja` user with proper groups and SSH access
4. **Basic Configuration**: Hostname set to `ninja`, firewall setup
5. **Service Enablement**: Services enabled but not fully started
6. **Reboot Preparation**: Stage 2 systemd service configured

### **Stage 2: Service Deployment (Post-Reboot)**
*Duration: ~10-15 minutes*

**What Happens:**
1. **Service Verification**: Ensures all services started properly after reboot
2. **Tailscale Setup**: VPN connection with `--accept-routes --timeout=0`
3. **Repository Cloning**: Secure clone using GitHub credentials
4. **Environment Setup**: Configure `.env` and project settings
5. **Docker Deployment**: Build and start all containers
6. **Monitoring Setup**: System health monitoring and service management
7. **Final Verification**: Service health checks and access validation

---

## 🔍 **MONITORING DEPLOYMENT**

### **Track Progress:**
1. The Linode will boot and start the StackScript automatically
2. You can monitor progress via Linode's console (Lish)
3. Stage 1 takes 5-10 minutes, then system reboots
4. Stage 2 takes 10-15 minutes after reboot
5. Total deployment time: 15-25 minutes

### **Check Deployment Status:**
```bash
# SSH into your instance
ssh ninja@YOUR_IP

# Check deployment status
cat /etc/ninja-trading/deployment-status.json

# Check services
sudo systemctl status ninja-trading
docker-compose ps
```

---

## ✅ **POST-DEPLOYMENT VERIFICATION**

### **Service Status Checks:**
```bash
# Check main service
sudo systemctl status ninja-trading

# Check Docker
sudo systemctl status docker
docker-compose ps

# Check ports
sudo netstat -tlnp | grep -E '(3000|8002|8081|6080|5900)'
```

### **Web Interface Access:**
- **Trading Interface**: `http://YOUR_IP:3000`
- **VNC Web**: `http://YOUR_IP:6080`
- **VS Code Server**: `http://YOUR_IP:8081`
- **Python API**: `http://YOUR_IP:8002`

### **SSH Access:**
```bash
ssh ninja@YOUR_IP
```

### **VNC Direct Access:**
- Host: `YOUR_IP:5900`
- Password: [Your VNC password]

---

## 🔐 **GITHUB ACTIONS CI/CD SETUP**

### **Required GitHub Secrets**

Go to: **Settings** → **Secrets and variables** → **Actions** → **Repository secrets**

#### **1. Tailscale Authentication**
```
TAILSCALE_OAUTH_CLIENT_ID
TAILSCALE_OAUTH_SECRET
```

**How to get these:**
1. Go to [Tailscale Admin Console](https://login.tailscale.com/admin)
2. Navigate to `Settings` → `OAuth clients`
3. Click `Generate OAuth client`
4. Set scopes: `devices:write` and `all:read`
5. Add tag: `tag:ci-cd`
6. Copy Client ID and Client Secret

#### **2. SSH Authentication**
```
NINJA_SSH_PRIVATE_KEY
```

**How to get this:**
```bash
# Generate SSH key for CI/CD
ssh-keygen -t ed25519 -f ~/.ssh/ninja-ci-cd -N ""

# Copy public key to your ninja server
ssh-copy-id -i ~/.ssh/ninja-ci-cd.pub ninja@ninja.tailfef10.ts.net

# Use private key content for GitHub secret
cat ~/.ssh/ninja-ci-cd
```

#### **3. Discord Notifications (Optional)**
```
DISCORD_WEBHOOK
```

### **GitHub Actions Features**
- **Automatic staging deployment** on `main` branch pushes
- **Manual production deployment** via workflow dispatch
- **Secure Tailscale networking** (no public SSH exposure)
- **Docker deployment** with health checks
- **Discord notifications** for deployment status

---

## 🛡️ **SECURITY HARDENING**

### **Firewall Configuration**
```bash
# Check firewall status
sudo ufw status

# Allow specific ports only
sudo ufw allow 22    # SSH
sudo ufw allow 3000  # Trading interface
sudo ufw allow 6080  # VNC web
sudo ufw enable
```

### **SSH Security**
```bash
# Verify SSH security settings
sudo grep -E "(PermitRootLogin|PasswordAuthentication)" /etc/ssh/sshd_config

# Should show:
# PermitRootLogin no
# PasswordAuthentication no
```

### **Update Passwords**
```bash
# Change ninja user password
sudo passwd ninja

# Update VNC password
vncpasswd
```

---

## 🔧 **CONFIGURATION FILES**

### **Environment Configuration**
```bash
# Check .env file
cat /opt/ninja-trading/.env

# Check deployment config
cat /etc/ninja-trading/deployment-config.json
```

### **Service Configuration**
```bash
# Main service file
sudo systemctl cat ninja-trading

# Docker compose configuration
cat /home/ninja/ninja/docker-compose.yml
```

---

## 📁 **IMPORTANT PATHS**

| Purpose | Path | Description |
|---------|------|-------------|
| **Main Repository** | `/home/ninja/ninja` | Cloned GitHub repository |
| **Configuration** | `/etc/ninja-trading/` | System configuration files |
| **Logs** | `/var/log/ninja-setup/` | Setup and deployment logs |
| **Application Logs** | `/home/ninja/ninja/logs/` | Runtime application logs |
| **Docker Data** | `/opt/ninja-trading/` | Docker volumes and data |

---

## 📈 **PERFORMANCE OPTIMIZATION**

### **Recommended System Monitoring**
```bash
# Check system resources
htop
df -h
free -h

# Check Docker resource usage
docker stats

# Monitor service logs
journalctl -u ninja-trading -f
```

### **Backup Strategy**
```bash
# Backup configuration
sudo tar -czf ninja-config-backup.tar.gz /etc/ninja-trading/

# Backup user data
tar -czf ninja-data-backup.tar.gz /home/ninja/ninja/

# Backup to external storage (recommended)
```

---

## ⚡ **QUICK DEPLOYMENT CHECKLIST**

### **Pre-Deployment:**
- [ ] Linode account created and verified
- [ ] GitHub Personal Access Token generated
- [ ] SSH key pair generated
- [ ] Repository access verified
- [ ] Discord webhook URL (optional)
- [ ] Tailscale auth key (optional)

### **Deployment Configuration:**
- [ ] StackScript created in Linode Cloud Manager
- [ ] Target image: Ubuntu 22.04 LTS
- [ ] Plan: Linode 4GB (recommended)
- [ ] Required fields filled in StackScript
- [ ] SSH keys added to Linode account

### **Post-Deployment Verification:**
- [ ] SSH access works: `ssh ninja@YOUR_IP`
- [ ] Web interfaces accessible
- [ ] Docker containers running
- [ ] Services healthy
- [ ] GitHub Actions secrets configured
- [ ] Firewall properly configured

### **GitHub Actions Setup:**
- [ ] Repository secrets configured
- [ ] Test staging deployment (push to main)
- [ ] Test manual production deployment
- [ ] Discord notifications working (if configured)

---

## 🆘 **NEED HELP?**

If you encounter issues during deployment, check the **Troubleshooting Guide** for common problems and solutions.

**Common issues:**
- Docker startup failures (especially on Arch Linux)
- SSH connection problems
- Service startup issues
- GitHub Actions authentication problems

The deployment is designed to be resilient and will attempt to continue even if some components fail initially.
