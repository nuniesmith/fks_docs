# FKS Trading Systems - Deployment Troubleshooting Guide

## Quick Fix Summary

### Issue 1: Docker Push Authentication Failure
**Error**: `push access denied, repository does not exist or may require authorization`

**Root Cause**: Docker Hub authentication issues or incorrect registry configuration.

**Solution**:
1. Check Docker Hub credentials in GitHub Secrets
2. Verify the repository exists on Docker Hub
3. Ensure proper permissions

### Issue 2: SSH Connection Failure
**Error**: `SSH connection failed to both actions_user and root`

**Root Cause**: Missing or incorrect SSH keys for GitHub Actions.

**Solution**:
1. Generate proper SSH keys
2. Add them to GitHub Secrets
3. Configure server SSH access

## Step-by-Step Troubleshooting

### 1. Docker Authentication Issues

#### Check Docker Hub Repository
1. Go to https://hub.docker.com
2. Log in with your Docker Hub account
3. Check if repositories exist:
   - `nuniesmith/fks` (main repository)
   - Verify you have push permissions

#### Verify GitHub Secrets
1. Go to https://github.com/nuniesmith/fks/settings/secrets/actions
2. Check these secrets exist and are correct:
   - `DOCKER_USERNAME`: Your Docker Hub username
   - `DOCKER_TOKEN`: Your Docker Hub personal access token (not password!)

#### Generate Docker Hub Personal Access Token
1. Go to https://hub.docker.com/settings/security
2. Click "New Access Token"
3. Name: `FKS-GitHub-Actions`
4. Access permissions: `Public Repo Read, Write`
5. Copy the token and add it to GitHub Secrets as `DOCKER_TOKEN`

### 2. SSH Connection Issues

#### Quick SSH Key Setup
Run the SSH key setup script:
```bash
./scripts/setup-ssh-keys.sh
```

Select option 4 for complete setup instructions.

#### Manual SSH Key Generation
```bash
# Generate SSH key for GitHub Actions
ssh-keygen -t ed25519 -f ~/.ssh/actions_user_fks -N "" -C "actions_user@fks-trading-system"

# Generate SSH key for Jordan (personal access)
ssh-keygen -t ed25519 -f ~/.ssh/jordan_fks -N "" -C "jordan@fks-trading-system"
```

#### Add SSH Keys to GitHub

**GitHub Secrets** (go to https://github.com/nuniesmith/fks/settings/secrets/actions):

1. **Secret**: `ACTIONS_USER_SSH_PUB`
   **Value**: Content of `~/.ssh/actions_user_fks.pub`

2. **Secret**: `JORDAN_SSH_PUB`
   **Value**: Content of `~/.ssh/jordan_fks.pub`

**Deploy Key** (go to https://github.com/nuniesmith/fks/settings/keys):

1. Click "Add deploy key"
2. **Title**: `actions_user@fks-trading-system`
3. **Key**: Content of `~/.ssh/actions_user_fks.pub`
4. ✅ Check "Allow write access"
5. Click "Add key"

### 3. Server SSH Access Issues

#### Check Server Status
```bash
# Check if server is running
linode-cli linodes list --text | grep fks-dev

# Check server IP
linode-cli linodes view [SERVER_ID] --text --format="ipv4"
```

#### Test SSH Connection
```bash
# Test SSH to server (replace with actual IP)
ssh -o StrictHostKeyChecking=no jordan@[SERVER_IP] 'echo "SSH working"'

# Test with root if jordan fails
ssh -o StrictHostKeyChecking=no root@[SERVER_IP] 'echo "SSH working"'
```

#### Check SSH Key Installation on Server
```bash
# Connect to server and check authorized_keys
ssh root@[SERVER_IP]
cat /home/actions_user/.ssh/authorized_keys
cat /home/jordan/.ssh/authorized_keys
```

### 4. NPM Dependency and Integrity Issues

#### Issue: NPM Integrity Checksum Failures
**Error**: `npm ERR! Integrity checksum failed when using sha512`

**Root Cause**: Corrupted npm cache or network issues causing incomplete package downloads.

**Solution**:
1. Run the automated fix script:
   ```bash
   cd src/web/react
   ./fix-deps.sh
   ```

2. Or manually fix:
   ```bash
   cd src/web/react
   npm cache clean --force
   rm -rf node_modules package-lock.json
   npm install --legacy-peer-deps --no-audit --no-fund --maxsockets 3
   npm audit fix
   ```

#### Issue: Deprecated Package Warnings
**Error**: Multiple npm deprecated package warnings

**Root Cause**: Outdated dependencies with known vulnerabilities.

**Solution**:
1. Updated dependencies in `package.json`:
   - `@types/node`: upgraded to version 20
   - `typescript`: upgraded to version 5.0
   - Added resolutions/overrides for problematic packages

2. Created `.npmrc` file in React web directory with:
   ```
   legacy-peer-deps=true
   audit=false
   fund=false
   maxsockets=3
   network-retry-count=3
   ```

#### Issue: GitHub Actions Build Failures
**Error**: Web build failing in CI/CD pipeline

**Root Cause**: Same npm integrity and dependency issues in CI environment.

**Solution**:
1. Updated GitHub Actions workflow to:
   - Clean npm cache before install
   - Remove node_modules and package-lock.json
   - Use `--legacy-peer-deps` flag
   - Limit max sockets to prevent corruption
   - Run audit fix automatically

2. Workflow now includes:
   ```yaml
   - name: Clean and install frontend dependencies
     run: |
       cd src/web/react
       npm cache clean --force
       rm -rf node_modules package-lock.json
       npm install --legacy-peer-deps --no-audit --no-fund --maxsockets 3
       npm audit fix
   ```

#### Fixing Local Development
1. **Run the fix script** (recommended):
   ```bash
   cd src/web/react
   ./fix-deps.sh
   ```

2. **Verify fixes**:
   ```bash
   npm run build  # Should complete without errors
   npm test       # Should pass all tests
   ```

3. **Commit changes**:
   ```bash
   git add package.json package-lock.json .npmrc
   git commit -m "Fix npm dependency and integrity issues"
   ```

### 5. Common Deployment Issues

#### GitHub Actions Workflow Failures

**Issue**: Build jobs failing
**Check**: 
- GitHub Actions logs for specific error messages
- Docker Hub rate limits
- Build cache issues

**Solution**:
```bash
# Clear build cache by re-running workflow with "clean-and-build" option
```

#### Server Not Responding

**Issue**: Server doesn't respond after creation
**Check**:
- Server is actually running
- Firewall rules
- Tailscale connection

**Solution**:
```bash
# Check server logs
journalctl -u fks-phase2.service -f

# Check setup log
tail -f /var/log/fks-setup.log
```

#### Services Not Starting

**Issue**: Docker services fail to start
**Check**:
- Docker daemon is running
- User permissions
- Port conflicts

**Solution**:
```bash
# Check Docker status
systemctl status docker

# Check user groups
groups jordan
groups fks_user
groups actions_user

# Test Docker access
sudo -u jordan docker ps
sudo -u fks_user docker ps
```

### 6. Emergency Access

#### Root Password Access
If SSH keys fail, you can still access the server using the root password:

1. Go to Linode Manager
2. Find your server (fks-dev)
3. Use the "Launch Console" option
4. Log in as root with `FKS_DEV_ROOT_PASSWORD`

#### Reset SSH Configuration
```bash
# Reset SSH configuration
systemctl restart sshd

# Re-generate SSH keys for users
sudo -u actions_user ssh-keygen -t ed25519 -f /home/actions_user/.ssh/id_ed25519 -N "" -C "actions_user@fks-dev"
sudo -u jordan ssh-keygen -t ed25519 -f /home/jordan/.ssh/id_ed25519 -N "" -C "jordan@fks-dev"
```

### 7. Deployment Validation

#### Check Deployment Status
```bash
# Check running containers
docker ps

# Check service logs
docker compose logs -f

# Check service health
curl -f http://localhost:8000/health
curl -f http://localhost:3000
```

#### Verify Network Configuration
```bash
# Check open ports
ss -tuln

# Check firewall rules
sudo iptables -L -n

# Check Tailscale status
tailscale status
```

### 8. Preventive Measures

#### Regular Maintenance
1. **Update Docker Hub tokens** every 6 months
2. **Rotate SSH keys** annually
3. **Monitor server resources** (CPU, memory, disk)
4. **Keep server updated** with security patches

#### Backup Strategy
1. **Database backups** automated daily
2. **Configuration backups** in Git
3. **SSH key backups** in secure location

### 9. Getting Help

#### Log Collection
Before asking for help, collect these logs:
```bash
# GitHub Actions logs (from GitHub web interface)
# Server setup logs
cat /var/log/fks-setup.log

# Docker logs
docker compose logs --tail=100

# System logs
journalctl -u fks-phase2.service -n 100
```

#### Common Support Channels
1. **GitHub Issues**: For code-related problems
2. **Server Documentation**: For server configuration issues
3. **Docker Documentation**: For containerization issues

---

## Quick Reference Commands

### SSH Key Management
```bash
# Generate new SSH key
ssh-keygen -t ed25519 -f ~/.ssh/keyname -N "" -C "comment"

# Copy public key to clipboard
cat ~/.ssh/keyname.pub | xclip -selection clipboard

# Test SSH connection
ssh -o StrictHostKeyChecking=no user@server 'echo "Connected"'
```

### Docker Registry Management
```bash
# Test Docker Hub login
echo "TOKEN" | docker login --username USERNAME --password-stdin

# Check Docker Hub repositories
curl -s https://hub.docker.com/v2/repositories/USERNAME/

# Push test image
docker tag hello-world USERNAME/test:latest
docker push USERNAME/test:latest
```

### Server Management
```bash
# Check server status
systemctl status docker
systemctl status tailscaled

# Check user permissions
groups USERNAME
sudo -u USERNAME docker ps

# Check logs
journalctl -u SERVICE_NAME -f
tail -f /var/log/fks-setup.log
```

---

This guide covers the most common deployment issues. For specific problems not covered here, check the GitHub Actions logs and server logs for more detailed error messages.
