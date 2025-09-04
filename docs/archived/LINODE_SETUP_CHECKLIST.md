# Ninja Trading System - Linode Setup Checklist

## Pre-Deployment Checklist

### ✅ Requirements
- [ ] Linode account created and verified
- [ ] GitHub Personal Access Token generated (ghp_...)
- [ ] SSH key pair generated
- [ ] Repository access verified
- [ ] Discord webhook URL (optional)
- [ ] Tailscale auth key (optional, for VPN)

### ✅ StackScript Configuration
- [ ] StackScript created in Linode Cloud Manager
- [ ] Target images selected (Ubuntu 22.04 LTS recommended)
- [ ] Script content copied from `scripts/StackScript.sh`

## Deployment Configuration

### Required Fields
- **GitHub Repository**: `nuniesmith/ninja`
- **GitHub Token**: `ghp_xxxxxxxxxxxxxxxxx`
- **SSH Public Key**: `ssh-rsa AAAAB3NzaC1yc2...`
- **Ninja Password**: `[secure_password]`
- **VNC Password**: `[secure_password]`

### Optional Fields
- **Tailscale Auth Key**: `tskey-auth-...` (recommended for production)
- **Domain Name**: `ninja.yourdomain.com`
- **Discord Webhook**: `https://discord.com/api/webhooks/...`

### Linode Configuration
- **Image**: Ubuntu 22.04 LTS
- **Region**: [Choose closest to your location]
- **Plan**: Linode 4GB (recommended)
- **Root Password**: [Strong password]
- **SSH Keys**: [Select your key]

## Post-Deployment Verification

### ✅ Service Status
```bash
# Check main service
sudo systemctl status ninja-trading

# Check Docker
sudo systemctl status docker
docker-compose ps

# Check ports
sudo netstat -tlnp | grep -E '(3000|8002|8081|6080|5900)'
```

### ✅ Web Interface Access
- [ ] Trading Interface: `http://YOUR_IP:3000`
- [ ] VNC Web: `http://YOUR_IP:6080`
- [ ] VS Code Server: `http://YOUR_IP:8081`
- [ ] Python API: `http://YOUR_IP:8002`

### ✅ SSH Access
```bash
ssh ninja@YOUR_IP
```

### ✅ VNC Direct Access
- Host: `YOUR_IP:5900`
- Password: [Your VNC password]

## Configuration Files to Review

### ✅ Environment Configuration
```bash
# Check .env file
cat /opt/ninja-trading/.env

# Check deployment config
cat /etc/ninja-trading/deployment-config.json
```

### ✅ Log Files
```bash
# Setup logs
ls -la /var/log/ninja-setup/

# Application logs
cd /opt/ninja-trading
docker-compose logs
```

## Security Hardening

### ✅ Firewall Status
```bash
sudo ufw status
```

### ✅ SSH Configuration
```bash
# Verify SSH security settings
sudo grep -E "(PermitRootLogin|PasswordAuthentication|PubkeyAuthentication)" /etc/ssh/sshd_config
```

### ✅ Password Updates
```bash
# Change ninja user password
sudo passwd ninja

# Update VNC password if needed
vncpasswd
```

## Troubleshooting Commands

### If Services Won't Start
```bash
# Check Docker logs
journalctl -u docker.service -n 20

# Check application logs
cd /opt/ninja-trading
docker-compose logs --tail=50

# Restart services
sudo systemctl restart docker
sudo systemctl restart ninja-trading
```

### If Can't Access Web Interfaces
```bash
# Check if ports are open
sudo ufw status
sudo netstat -tlnp

# Check container status
docker-compose ps
```

### If Git Operations Fail
```bash
# Check repository status
cd /opt/ninja-trading
git status
git remote -v

# Verify token access
git pull origin main
```

## Deployment Timeline

### Phase 1 (5-10 minutes)
- [ ] System package updates
- [ ] .NET SDK installation
- [ ] Docker installation
- [ ] SSH configuration
- [ ] Firewall setup

### Reboot (1-2 minutes)
- [ ] System automatically reboots

### Phase 2 (5-10 minutes)
- [ ] Repository cloning
- [ ] Environment setup
- [ ] Docker containers build
- [ ] Services start
- [ ] Monitoring setup

### Total Time: 15-20 minutes

## Success Indicators

### ✅ Deployment Complete When:
- [ ] All web interfaces accessible
- [ ] SSH access working
- [ ] Docker containers running
- [ ] No errors in logs
- [ ] System monitoring active
- [ ] Firewall properly configured

### ✅ Ready for Trading When:
- [ ] NinjaTrader environment accessible via VNC
- [ ] Python API responding
- [ ] Real-time data connections established
- [ ] Strategy compilation successful
- [ ] Monitoring alerts configured

## Support Resources

### Log Locations
- Setup logs: `/var/log/ninja-setup/`
- Application logs: `docker-compose logs`
- System logs: `journalctl -u ninja-trading`
- Monitor logs: `/var/log/ninja-monitor.log`

### Configuration Files
- Main config: `/etc/ninja-trading/deployment-config.json`
- Environment: `/opt/ninja-trading/.env`
- Docker config: `/opt/ninja-trading/docker-compose.yml`

### Key Commands
```bash
# Service management
sudo systemctl {status|start|stop|restart} ninja-trading

# Container management
docker-compose {ps|logs|restart|down|up -d}

# System monitoring
htop
df -h
free -h

# Network debugging
sudo netstat -tlnp
sudo ufw status
```

---

**Note**: Keep this checklist handy during deployment and save the IP address and passwords in a secure location!
