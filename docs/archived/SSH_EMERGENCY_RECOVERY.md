# 🚨 SSH Connection Failure - Action Plan

## Current Situation
- **Server**: `fks.tailfef10.ts.net` (172.105.97.209)
- **Issue**: SSH connection failing for both `actions_user` and `jordan` users
- **Error**: "Permission denied (publickey,password)"

## 🔍 Immediate Diagnosis Steps

### 1. **Run Emergency Troubleshooting Script**
```bash
# From your local machine
./scripts/emergency-ssh-debug.sh fks.tailfef10.ts.net 172.105.97.209 jordan
```

### 2. **Use GitHub Actions for Advanced Diagnosis**
1. Go to: **Actions** → **SSH Key Management** → **Run workflow**
2. Set parameters:
   - **Server host**: `fks.tailfef10.ts.net`
   - **Target user**: `jordan`
   - **Action**: `test_connection`
3. Check the detailed logs for specific failure reasons

## 🔧 Recovery Options

### Option A: **Automated Recovery via GitHub Actions**
1. **Actions** → **SSH Key Management** → **Run workflow**
2. Set parameters:
   - **Action**: `recover_server_access`
   - **Server host**: `fks.tailfef10.ts.net`
   - **Target user**: `jordan`
3. This will attempt to find any working SSH access and install keys

### Option B: **Manual Recovery via Linode Console**
1. **Access Linode Console**:
   - Go to Linode Dashboard
   - Select your FKS server
   - Click **Launch Console** (LISH)

2. **Login and Fix SSH Keys**:
   ```bash
   # Login as root or available user
   
   # Create/fix jordan user
   sudo useradd -m -s /bin/bash jordan
   sudo usermod -aG sudo,docker jordan
   sudo mkdir -p /home/jordan/.ssh
   sudo chmod 700 /home/jordan/.ssh
   
   # Add your public key (get from GitHub Actions secrets or local machine)
   echo "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC..." | sudo tee /home/jordan/.ssh/authorized_keys
   sudo chmod 600 /home/jordan/.ssh/authorized_keys
   sudo chown -R jordan:jordan /home/jordan/.ssh
   
   # Create/fix actions_user user
   sudo useradd -m -s /bin/bash actions_user
   sudo usermod -aG sudo,docker actions_user
   sudo mkdir -p /home/actions_user/.ssh
   sudo chmod 700 /home/actions_user/.ssh
   echo "ssh-rsa AAAAB3NzaC1yc2EAAAADAQABAAABgQC..." | sudo tee /home/actions_user/.ssh/authorized_keys
   sudo chmod 600 /home/actions_user/.ssh/authorized_keys
   sudo chown -R actions_user:actions_user /home/actions_user/.ssh
   
   # Restart SSH service
   sudo systemctl restart ssh
   ```

### Option C: **Check StackScript Status**
The server may still be initializing. Check:
1. **Linode Console** → **Your Server** → **Activity Feed**
2. Look for StackScript completion status
3. If still running, wait for completion

## 🔑 Getting Your Public Key

### From Local Machine:
```bash
cat ~/.ssh/id_rsa.pub
# or
cat ~/.ssh/id_ed25519.pub
```

### From GitHub Actions Private Key:
```bash
# If you have the private key from secrets
echo "-----BEGIN OPENSSH PRIVATE KEY-----..." | ssh-keygen -y -f /dev/stdin
```

## ✅ Verification Steps

After any recovery attempt:

1. **Test SSH Access**:
   ```bash
   ssh jordan@fks.tailfef10.ts.net
   ssh actions_user@fks.tailfef10.ts.net
   ```

2. **Run GitHub Actions Test**:
   - **Actions** → **SSH Key Management** → **Run workflow**
   - **Action**: `test_connection`

3. **Check All Users**:
   - **Actions** → **SSH Key Management** → **Run workflow**
   - **Action**: `check_all_users`

## 🚨 If Nothing Works

1. **Rebuild Server**: 
   - The StackScript may have failed
   - Consider recreating the server with proper SSH keys

2. **Manual SSH Key Setup**:
   - Use Linode Console exclusively
   - Manually configure all users and keys

3. **Alternative Access**:
   - Use password authentication temporarily
   - Fix SSH key setup while logged in

## 📋 Prevention for Next Time

1. **Always provide SSH keys** during Linode server creation
2. **Test SSH access** before running deployments
3. **Keep backup access methods** (console access)
4. **Document recovery procedures**

---

## 🔧 Quick Commands Reference

```bash
# Test connection
ssh -v jordan@fks.tailfef10.ts.net

# Emergency diagnosis
./scripts/emergency-ssh-debug.sh

# GitHub Actions recovery
# Actions → SSH Key Management → recover_server_access

# Linode console access
# Dashboard → Server → Launch Console
```
