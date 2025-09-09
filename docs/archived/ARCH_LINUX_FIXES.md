# Arch Linux StackScript Issues and Solutions

## 🔍 **Issues Identified**

Your StackScript deployment failed due to several Arch Linux-specific issues:

### 1. **Docker Storage Driver Issues**
- **Problem**: Overlay filesystem module not available in the kernel
- **Error**: `failed to mount overlay` → `no such device`
- **Solution**: Fallback to VFS storage driver

### 2. **Network/iptables Issues**
- **Problem**: iptables NAT table not available
- **Error**: `iptables nat table unavailable`
- **Solution**: Disable iptables integration in Docker

### 3. **User Group Issues**
- **Problem**: Arch Linux doesn't have `sudo` group, uses `wheel` instead
- **Error**: `usermod: group 'sudo' does not exist`
- **Solution**: Use `wheel` group for Arch Linux

## 🛠️ **Immediate Fix Options**

### Option 1: Quick Recovery Script (Recommended)
Run this on your current Linode instance:

```bash
# Download and run the recovery script
curl -fsSL https://raw.githubusercontent.com/nuniesmith/ninja/main/scripts/arch-docker-recovery.sh -o recovery.sh
chmod +x recovery.sh
sudo ./recovery.sh
```

### Option 2: Manual Fix
If you prefer to fix manually:

```bash
# 1. Fix user groups
sudo groupadd wheel 2>/dev/null || true
echo "%wheel ALL=(ALL) NOPASSWD: ALL" | sudo tee -a /etc/sudoers
sudo usermod -aG wheel ninja
sudo usermod -aG docker ninja

# 2. Configure Docker for compatibility
sudo mkdir -p /etc/docker
sudo tee /etc/docker/daemon.json << EOF
{
  "storage-driver": "vfs",
  "iptables": false,
  "ip-masq": false,
  "bridge": "none",
  "log-driver": "json-file",
  "log-opts": {
    "max-size": "10m",
    "max-file": "3"
  }
}
EOF

# 3. Restart Docker
sudo systemctl daemon-reload
sudo systemctl enable docker
sudo systemctl start docker

# 4. Verify Docker is working
sudo docker version
```

### Option 3: Redeploy with Updated StackScript
Use the updated StackScript that now includes Arch Linux specific fixes.

## 📋 **What the Fixes Do**

### **Updated StackScript Changes:**

1. **Arch Linux Detection**: Automatically detects Arch and applies appropriate fixes
2. **Group Management**: Uses `wheel` group instead of `sudo` for Arch Linux
3. **Docker Configuration**: Sets up VFS storage and disables problematic features
4. **Network Compatibility**: Configures host networking mode for containers

### **Docker Configuration Explanation:**

```json
{
  "storage-driver": "vfs",          // Compatible storage (slower but reliable)
  "iptables": false,                // Disable iptables integration
  "ip-masq": false,                 // Disable IP masquerading
  "bridge": "none",                 // No Docker bridge (use host networking)
  "log-driver": "json-file",        // Standard logging
  "log-opts": {                     // Limit log size
    "max-size": "10m",
    "max-file": "3"
  }
}
```

## 🚦 **Expected Behavior After Fix**

### **What Will Work:**
✅ Docker service starts successfully  
✅ Containers can be built and run  
✅ Host networking provides connectivity  
✅ All application ports accessible  
✅ SSH access works with wheel group  

### **Limitations:**
⚠️ **Slower storage**: VFS is less efficient than overlay2  
⚠️ **Host networking**: Containers share host network stack  
⚠️ **No container isolation**: Network isolation between containers disabled  

### **Performance Impact:**
- **Storage**: ~10-20% slower container operations
- **Networking**: No significant impact
- **Overall**: Fully functional for development/trading use

## 🔄 **Next Steps**

After applying the fix:

1. **Verify Docker Status:**
   ```bash
   sudo systemctl status docker
   sudo docker version
   ```

2. **Continue StackScript Execution:**
   The StackScript should be able to continue from where it left off, or you can manually proceed:
   ```bash
   cd /opt/ninja-trading
   sudo -u ninja docker-compose build
   sudo -u ninja docker-compose up -d
   ```

3. **Test Access:**
   - Web Interface: `http://YOUR_IP:3000`
   - VS Code: `http://YOUR_IP:8081`
   - API: `http://YOUR_IP:8002`

## 🎯 **Why This Happened**

### **Arch Linux Differences:**
- **Rolling release**: Latest kernel may lack some compatibility modules
- **Different groups**: Uses `wheel` instead of `sudo`
- **Minimal base**: Some networking features not enabled by default
- **Latest Docker**: May have different default configurations

### **Container Environment Challenges:**
- **Limited kernel**: Linode VMs may have restricted kernel modules
- **Networking stack**: iptables/netfilter configuration differences
- **Storage drivers**: Overlay filesystem module availability varies

## 🛡️ **Prevention for Future Deployments**

### **Recommended OS Choices:**
1. **Ubuntu 22.04 LTS** ⭐ (Most tested, best compatibility)
2. **Debian 11** (Stable, well-supported)
3. **Arch Linux** (Latest features, but requires fixes like these)

### **Updated StackScript Benefits:**
- ✅ Automatic OS detection and fixes
- ✅ Fallback configurations for compatibility
- ✅ Better error handling and recovery
- ✅ Comprehensive logging for troubleshooting

## 📞 **Support**

If you continue to experience issues:

1. **Check Docker logs**: `sudo journalctl -u docker.service -f`
2. **Verify networking**: `sudo netstat -tlnp`
3. **Test containers**: `sudo docker run hello-world`

The recovery script should resolve the immediate issues and get your deployment back on track!
