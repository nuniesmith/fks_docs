# SSH Connection Quick Fix Guide

## 🚨 **IMMEDIATE FIXES for "Permission denied (publickey,password)"**

### 1. **Check GitHub Secrets** (Most Common Issue)
```bash
# Verify these secrets are set in your GitHub repository:
# Settings > Secrets and variables > Actions

DEV_SERVER_HOST=your.server.ip.or.hostname
DEV_SERVER_USER=your-username
DEV_SERVER_SSH_KEY=your-private-key-content
```

### 2. **Verify SSH Key Format**
Your `DEV_SERVER_SSH_KEY` secret must include headers:
```text
-----BEGIN OPENSSH PRIVATE KEY-----
b3BlbnNzaC1rZXktdjEAAAAABG5vbmUAAAAEbm9uZQAAAAAAAAABFwAAAAdzc2gtcn
... (key content) ...
-----END OPENSSH PRIVATE KEY-----
```

### 3. **Add Public Key to Server**
```bash
# On your deployment server:
mkdir -p ~/.ssh
chmod 700 ~/.ssh

# Add your public key (get it from your private key):
ssh-keygen -y -f /path/to/private/key >> ~/.ssh/authorized_keys
chmod 600 ~/.ssh/authorized_keys
```

### 4. **Quick Server Checks**
```bash
# Check SSH service
sudo systemctl status ssh
sudo systemctl restart ssh

# Check SSH config
sudo nano /etc/ssh/sshd_config
# Ensure these settings:
PubkeyAuthentication yes
AuthorizedKeysFile .ssh/authorized_keys

# Check firewall
sudo ufw allow ssh
sudo ufw status
```

### 5. **Test Connection Manually**
```bash
# Test from your local machine
ssh -vvv username@hostname

# Check what the server sees
sudo tail -f /var/log/auth.log  # On server while testing
```

## 🔧 **Use the Troubleshooting Script**
```bash
# Run the automated troubleshooting script
./scripts/deployment/tools/troubleshoot-ssh.sh

# Or with parameters
./scripts/deployment/tools/troubleshoot-ssh.sh hostname username port
```

## 📝 **Common Fixes Summary**

| Problem | Quick Fix |
|---------|-----------|
| Key not on server | `ssh-copy-id user@host` |
| Wrong key format | Check headers in GitHub secret |
| SSH service down | `sudo systemctl restart ssh` |
| Firewall blocking | `sudo ufw allow ssh` |
| Wrong hostname | Verify `DEV_SERVER_HOST` secret |
| Wrong username | Verify `DEV_SERVER_USER` secret |

## 🆘 **Still Not Working?**

1. **Check the GitHub Actions logs** for detailed error messages
2. **Run the troubleshooting script**: `./scripts/deployment/tools/troubleshoot-ssh.sh`
3. **Test manually** from your local machine first
4. **Check server logs**: `sudo journalctl -u ssh -f`
5. **Verify network connectivity**: `ping your-server-ip`

## 🔐 **Security Notes**

- Never put private keys in code or logs
- Use Ed25519 keys for better security: `ssh-keygen -t ed25519`
- Disable password authentication: `PasswordAuthentication no`
- Use fail2ban to prevent brute force attacks
