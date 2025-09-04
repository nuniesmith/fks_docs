# Docker iptables Fix for FKS Trading Systems

This document describes the Docker iptables networking fixes integrated into the FKS deployment system.

## Problem

Docker networking can fail with errors like:
```
failed to create network fks_database: Error response from daemon: Failed to Setup IP tables: Unable to enable DROP INCOMING rule: (iptables failed: iptables --wait -t filter -I DOCKER-ISOLATION-STAGE-1 -i br-d8bb095bdd53 ! -d 172.18.0.0/16 -j DROP: iptables: No chain/target/match by that name.
```

This happens when Docker's iptables chains become corrupted or missing, preventing network creation.

## Solutions Integrated

### 1. Enhanced start.sh Script

The main `start.sh` script now includes automatic Docker iptables fixes:

```bash
# Basic usage (auto-detects issues)
./start.sh

# Force fix even if no issues detected
./start.sh --force-fix

# Skip fix attempt (if you're sure Docker is working)
./start.sh --skip-fix

# Verbose output for debugging
./start.sh --verbose
```

### 2. Manual Fix Scripts

#### Quick Fix (Development)
```bash
# Quick manual fix for development environments
sudo ./scripts/utils/fix-docker-startup.sh
```

#### Comprehensive Fix (Production)
```bash
# Full featured fix script with backup and verification
sudo ./scripts/deployment/staged/fix-docker-iptables.sh --verbose
```

### 3. Deployment Integration

#### Stage 1 (Initial Setup)
- Installs iptables packages correctly for Arch Linux
- Creates embedded Docker fix script
- Enables Docker service (but doesn't start it yet)

#### Stage 2 (Post-Reboot)
- Applies Docker iptables fix automatically
- Starts Docker with clean networking state
- Tests Docker networking functionality

#### GitHub Actions
- Preventive Docker iptables fix in build environment
- Ensures reliable image building and pushing

## How the Fix Works

1. **Stop Docker Daemon**
   - Gracefully stops Docker service
   - Prevents new network operations

2. **Clean iptables Chains**
   - Removes broken Docker chains:
     - `DOCKER`
     - `DOCKER-ISOLATION-STAGE-1`
     - `DOCKER-ISOLATION-STAGE-2`
     - `DOCKER-USER`

3. **Clean NAT/MANGLE Tables**
   - Removes Docker rules from NAT table
   - Removes Docker rules from MANGLE table

4. **Remove Bridge Interfaces**
   - Removes `docker0` bridge if it exists
   - Removes any Docker-created bridges (`br-*`)

5. **Restart Docker**
   - Starts Docker daemon with clean state
   - Allows Docker to recreate proper chains

6. **Verify Fix**
   - Tests network creation functionality
   - Confirms containers can run

## Usage in Different Scenarios

### Development Server
```bash
# If docker-compose fails with networking errors:
cd ~/fks
sudo ./scripts/utils/fix-docker-startup.sh
./start.sh
```

### Production Deployment
The fix is automatically applied during deployment:
- Stage 1: Prepares fix scripts
- Stage 2: Applies fix and starts Docker
- No manual intervention needed

### GitHub Actions
The fix is automatically applied during image builds to prevent CI/CD failures.

### Manual Troubleshooting
```bash
# Check Docker status
sudo systemctl status docker

# Check iptables chains
sudo iptables -L | grep -i docker

# Apply comprehensive fix
sudo ./scripts/deployment/staged/fix-docker-iptables.sh --verbose

# Test Docker networking
docker network create test-network
docker network rm test-network
```

## Prevention

### Arch Linux Specific
- Uses `iptables-nft` backend when possible
- Handles iptables package conflicts correctly
- Loads required kernel modules after reboot

### Cross-Platform
- Works on Ubuntu, Debian, Arch Linux
- Detects OS and adapts accordingly
- Provides fallback methods

## Files Modified

1. **Enhanced Scripts:**
   - `start.sh` - Enhanced with automatic fix detection
   - `scripts/deployment/staged/stage-1-initial-setup.sh` - Embedded fix script
   - `.github/workflows/00-default.yml` - Preventive GitHub Actions fix

2. **New Scripts:**
   - `scripts/utils/fix-docker-startup.sh` - Quick manual fix
   - `scripts/deployment/staged/fix-docker-iptables.sh` - Comprehensive fix

3. **Documentation:**
   - `docs/DOCKER_IPTABLES_FIX.md` - This document

## Testing

After applying fixes, verify with:
```bash
# Test network creation
docker network create test-fix && docker network rm test-fix

# Test compose functionality
cd ~/fks && docker-compose config

# Test full stack
./start.sh --verbose
```

## Troubleshooting

If fixes don't work:

1. **Reboot the system** (often resolves kernel module issues)
2. **Check kernel modules:** `lsmod | grep netfilter`
3. **Check iptables version:** `iptables --version`
4. **Check Docker logs:** `journalctl -u docker.service -f`
5. **Manual cleanup:** Remove `/var/lib/docker/network/` and restart Docker

## Support

For issues related to Docker iptables fixes:
1. Check the deployment logs: `journalctl -u fks-stage2.service -f`
2. Run diagnostic: `sudo ./scripts/deployment/staged/fix-docker-iptables.sh --verbose`
3. Manual intervention: `sudo ./scripts/utils/fix-docker-startup.sh`
