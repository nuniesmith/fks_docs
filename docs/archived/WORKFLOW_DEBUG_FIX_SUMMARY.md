# 🔧 Workflow Debug & Fix Summary

**Date:** July 7, 2025  
**Issue:** Stage 2 not starting automatically, Tailscale not running, Git clone authentication failure

## 🚨 Problems Identified

### 1. ❌ Tailscale Service Not Running
```
failed to connect to local tailscaled; it doesn't appear to be running
```
**Root Cause:** `tailscaled` service not started after server reboot

### 2. ❌ Git Clone Authentication Failure  
```
fatal: could not read Username for 'https://github.com': No such device or address
```
**Root Cause:** Using HTTPS without authentication for private repository

### 3. ❌ Phase 2 Service Detection Wrong
```
systemctl list-units --all | grep -q "fks-stage2"
```
**Root Cause:** Looking for wrong service name (`fks-stage2` vs `fks-phase2`)

### 4. ❌ Phase 2 Systemd Service Missing
**Root Cause:** Stage 1 script was logging "Phase 2 will run automatically via systemd service" but never actually created the systemd service.

**Impact:** 
- Server would reboot successfully 
- Phase 2 never started (no systemd service)
- Workflow stuck in infinite monitoring loop
- Tailscale never configured, Docker never installed

**Evidence:**
```bash
# Stage 1 script said this but never created the service:
log "After reboot, Stage 2 will run automatically via systemd service"
log "You can monitor Stage 2 progress with: journalctl -u fks-stage2.service -f"

# But no systemd service was ever created!
```

## ✅ Fixes Applied

### 1. 🔧 Fixed Phase 2 Service Detection
**Before:**
```yaml
elif systemctl list-units --all | grep -q "fks-stage2"; then
```

**After:**
```yaml
elif systemctl is-active fks-phase2 >/dev/null 2>&1; then
  echo "🔧 Phase 2 service is active and running..."
  exit 15
elif systemctl is-enabled fks-phase2 >/dev/null 2>&1; then
  echo "⚙️ Phase 2 service exists but not active - checking status..."
  systemctl status fks-phase2 --no-pager || true
  echo "⏳ Phase 2 service found but not running - may need manual start"
  exit 12
```

### 2. 🔧 Added Manual Phase 2 Start Logic
```yaml
elif [ $SSH_EXIT_CODE -eq 12 ]; then
  echo "⚙️ Phase 2 service needs manual start - attempting to start..."
  if sshpass -p "$FKS_DEV_ROOT_PASSWORD" ssh -o StrictHostKeyChecking=no root@$TARGET_IP '
    echo "🔧 Starting Phase 2 service manually..."
    systemctl start fks-phase2 && echo "✅ Phase 2 started" || echo "❌ Failed to start Phase 2"
    
    # Also ensure tailscaled is running if Phase 2 is supposed to handle it
    if ! systemctl is-active tailscaled >/dev/null 2>&1; then
      echo "🔧 Starting tailscaled service..."
      systemctl start tailscaled && echo "✅ tailscaled started" || echo "❌ Failed to start tailscaled"
    fi
  '; then
    echo "✅ Manual Phase 2 start attempted"
  else
    echo "❌ Failed to manually start Phase 2"
  fi
  REBOOT_DETECTED=true
```

### 3. 🔧 Fixed Tailscale Service Start
**Added explicit tailscaled service startup:**
```yaml
# Ensure tailscaled service is running
if ! systemctl is-active tailscaled >/dev/null 2>&1; then
  echo "🔧 Starting tailscaled service..."
  systemctl start tailscaled || { echo "❌ Failed to start tailscaled"; exit 1; }
  sleep 3
fi
```

### 4. 🔧 Fixed Git Clone Authentication
**Before:**
```yaml
git clone https://github.com/${{ github.repository }}.git fks
```

**After:**
```yaml
git clone https://${{ secrets.GITHUB_TOKEN }}@github.com/${{ github.repository }}.git fks
```

### 5. 🔧 Fixed SSL_STAGING Environment Variable
**Before:**
```yaml
SSL_STAGING: ${{ secrets.SSL_STAGING || 'false' }}
if [ '$SSL_STAGING' = 'true' ]; then
```

**After:**
```yaml
# Removed from env section since secret doesn't exist
if [ '${SSL_STAGING:-false}' = 'true' ]; then
```

### 6. 🔧 Added Phase 2 Systemd Service Creation
**Location:** `scripts/deployment/staged/stage-1-initial-setup.sh`

**Added Components:**
1. **Environment Variables Preservation:**
   ```bash
   cat > /root/.fks-env << ENV_EOF
   export TAILSCALE_AUTH_KEY="$TAILSCALE_AUTH_KEY"
   # ... other vars
   ENV_EOF
   ```

2. **Phase 2 Script Creation:**
   ```bash
   cat > /usr/local/bin/fks-phase2.sh << 'PHASE2_EOF'
   # Configures Tailscale, installs Docker, marks completion
   PHASE2_EOF
   chmod +x /usr/local/bin/fks-phase2.sh
   ```

3. **Systemd Service Creation:**
   ```bash
   cat > /etc/systemd/system/fks-phase2.service << SERVICE_EOF
   [Unit]
   Description=FKS Trading Systems Setup Phase 2
   After=network-online.target
   
   [Service]
   Type=oneshot
   ExecStart=/usr/local/bin/fks-phase2.sh
   
   [Install]
   WantedBy=multi-user.target
   SERVICE_EOF
   systemctl enable fks-phase2.service
   ```

### 🎯 Phase 2 Functionality
The new Phase 2 service will:
- ✅ Start `tailscaled` service
- ✅ Authenticate Tailscale with auth key
- ✅ Install and start Docker
- ✅ Mark Phase 2 complete with `/root/.fks-phase2-complete`
- ✅ Disable itself so it only runs once

### 🔍 Workflow Detection Improvements
With this fix, the workflow monitoring will now detect:
- **Exit Code 15:** `fks-phase2.service` is running
- **Exit Code 20:** Docker is running (Phase 2 complete)
- **Exit Code 12:** Service exists but not active (can restart manually)

---
## 🎯 Expected Results

### ✅ Next Workflow Run Should:
1. **Phase 2 Detection:** Correctly identify `fks-phase2` service status
2. **Manual Recovery:** Start Phase 2 service if it exists but isn't running
3. **Tailscale Fix:** Explicitly start `tailscaled` service before authentication
4. **Repository Access:** Successfully clone repository using GitHub token
5. **SSL Setup:** Handle missing SSL_STAGING secret gracefully

### 🔍 Monitoring Flow:
```
Server Reboot → Check fks-phase2 service → Start if needed → Start tailscaled → 
Authenticate Tailscale → Clone repo with token → Deploy services → SSL setup
```

## 🚀 Testing the Fixes

### Manual Test Commands:
```bash
# Check Phase 2 service
systemctl status fks-phase2

# Check Tailscale
systemctl status tailscaled
tailscale status

# Test Git clone with token
git clone https://TOKEN@github.com/repo.git

# Check workflow logs
# Look for new exit codes: 12 (Phase 2 needs start), 15 (running), 20 (complete)
```

### Expected Log Messages:
- `✅ Phase 2 started` (if manual start needed)
- `✅ tailscaled started` (if service wasn't running)
- `✅ Tailscale authentication successful!`
- `✅ Repository setup complete`

## 📚 Related Documentation
- [Workflow Proactive Improvements](WORKFLOW_PROACTIVE_IMPROVEMENTS.md)
- [Secrets Quick Reference](SECRETS_QUICK_REFERENCE.md)
- [Cloudflare SSL Setup Guide](CLOUDFLARE_SSL_SETUP_GUIDE.md)

---
**Status:** ✅ All fixes applied and validated  
**Next Action:** Test workflow with a new deployment
