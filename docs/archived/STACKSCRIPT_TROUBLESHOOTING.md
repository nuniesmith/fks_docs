# StackScript Troubleshooting Guide

## Issue: "sudo: unknown user ninja" Error

### Problem Description
The StackScript fails during SSH setup with the error:
```
sudo: unknown user ninja
sudo: error initializing audit plugin sudoers_audit
```

### Root Cause
The `setup_ssh_access()` function was trying to configure SSH for the `ninja` user before the user was created. The user creation happens later in the `setup_docker()` function.

### Solution Applied
✅ **Fixed in Updated StackScript:**
1. Split SSH setup into two functions:
   - `setup_ssh_access()` - System-wide SSH configuration (runs early)
   - `setup_user_ssh_access()` - User-specific SSH setup (runs after user creation)

2. Reordered function calls to ensure proper sequence

### If You Encounter This Error

#### Option 1: Use the Quick Fix Script
If your deployment failed with this error, run the quick fix:

```bash
# Download and run the quick fix
curl -fsSL https://raw.githubusercontent.com/nuniesmith/ninja/main/scripts/stackscript-quickfix.sh -o quickfix.sh
chmod +x quickfix.sh

# Set your parameters
export NINJA_PASSWORD="your_secure_password"
export SSH_KEY="your_ssh_public_key_here"

# Run the fix
sudo ./quickfix.sh
```

#### Option 2: Manual Fix
If you prefer to fix manually:

```bash
# 1. Create the ninja user
sudo useradd -m -s /bin/bash ninja
echo "ninja:your_password" | sudo chpasswd
sudo usermod -aG sudo ninja
sudo usermod -aG docker ninja

# 2. Set up SSH key (if you have one)
sudo -u ninja mkdir -p /home/ninja/.ssh
echo "your_ssh_public_key" | sudo -u ninja tee /home/ninja/.ssh/authorized_keys
sudo -u ninja chmod 600 /home/ninja/.ssh/authorized_keys
sudo -u ninja chmod 700 /home/ninja/.ssh

# 3. Continue with the original StackScript
# The StackScript should now be able to continue from where it left off
```

#### Option 3: Restart with Updated StackScript
1. Use the updated StackScript with the fixes
2. Redeploy your Linode instance

### Updated StackScript Changes

#### Before (Problematic):
```bash
# Phase 2: Service Setup
setup_ssh_access     # ❌ Tried to use ninja user before creation
setup_docker         # Creates ninja user here
```

#### After (Fixed):
```bash
# Phase 2: Service Setup
setup_ssh_access     # ✅ Only system-wide SSH config
setup_docker         # Creates ninja user, then calls setup_user_ssh_access
```

### Verification Steps

After applying the fix, verify everything is working:

```bash
# 1. Check user exists
id ninja

# 2. Check user groups
groups ninja

# 3. Test SSH access (if SSH key was provided)
ssh ninja@localhost -o PreferredAuthentications=publickey

# 4. Check SSH configuration
sudo sshd -T | grep -E "(PermitRootLogin|PasswordAuthentication|PubkeyAuthentication)"

# 5. Continue with Docker setup
sudo systemctl status docker
```

### Prevention

To avoid this issue in future deployments:
1. Use the updated StackScript from this repository
2. Always test StackScripts in a development environment first
3. Monitor deployment logs for early error detection

### Additional Common Issues

#### Docker Not Starting
```bash
# Check Docker status
sudo systemctl status docker

# Check for common issues
sudo journalctl -u docker.service -n 20

# Try manual start
sudo systemctl start docker
```

#### Package Installation Failures (Arch Linux)
```bash
# Update package database
sudo pacman -Syyu --noconfirm

# Clear package cache if needed
sudo pacman -Scc --noconfirm

# Reinstall problematic packages
sudo pacman -S --needed --noconfirm dotnet-sdk
```

#### Network/Firewall Issues
```bash
# Check firewall status
sudo ufw status

# Check if ports are listening
sudo netstat -tlnp | grep -E "(3000|8002|8081|6080|5900)"

# Reset firewall if needed
sudo ufw --force reset
```

### Getting Help

If you continue to experience issues:

1. **Check the logs:**
   ```bash
   # StackScript logs
   ls -la /var/log/ninja-setup/
   tail -f /var/log/ninja-setup/setup-*.log
   
   # System logs
   journalctl -xe
   ```

2. **Gather system information:**
   ```bash
   # OS information
   cat /etc/os-release
   
   # Available resources
   free -h
   df -h
   
   # Network configuration
   ip addr show
   ```

3. **Report the issue:**
   - Include the full error message
   - Provide system information
   - Share relevant log excerpts
   - Mention which Linode plan you're using

### Contact Information

- **GitHub Issues**: [Repository Issues Page]
- **Discord**: [If webhook was configured]
- **Linode Support**: For infrastructure-related issues

---

**Note**: This guide specifically addresses the "unknown user ninja" error. For other StackScript issues, refer to the main troubleshooting section in the deployment guide.
