# Linode CLI Authentication Fix - Latest Update

## Issue: Authentication Failure (Latest Run)

The GitHub Actions workflow failed with:
```
❌ Linode CLI authentication failed - check LINODE_CLI_TOKEN secret
```

## Solution Applied

### 1. Enhanced Authentication Method
Updated the workflow to use environment variables as the primary authentication method:

```yaml
env:
  LINODE_CLI_TOKEN: ${{ secrets.LINODE_CLI_TOKEN }}
```

### 2. Dual Fallback System
- **Primary**: Environment variable method
- **Fallback**: Configuration file method
- **Testing**: Uses `regions list` command (minimal permissions required)

### 3. All Steps Updated
Added environment variable authentication to:
- Check for existing server
- Create/update StackScript  
- Create Linode server
- Wait for server setup

### 4. Enhanced Debugging
The workflow now provides:
- Token length validation
- Configuration file verification
- Clear success/failure messages
- Debugging info without exposing sensitive data

## Next Steps

1. **Run the workflow again** - the authentication should now work
2. **Check the debug output** if it still fails
3. **Verify GitHub secrets** if debugging shows token issues

The workflow should now successfully authenticate with Linode and proceed with server creation.

---

# Linode Token and StackScript Troubleshooting Guide

## Latest Fix - UDF Naming Issue (Fixed: July 10, 2025)

### Problem
GitHub Actions deployment was failing with:
```
Request failed: 400
actions_user_ssh_pub	This StackScript does not accept UDF actions_user_ssh_pub.
actions_jordan_ssh_pub	This StackScript does not accept UDF actions_jordan_ssh_pub.
```

### Root Cause
The GitHub Actions workflow was sending UDF (User Defined Field) names in lowercase, but the StackScript was expecting them in uppercase.

**StackScript UDF Definition (Correct):**
```bash
#<UDF name="ACTIONS_USER_SSH_PUB" label="GitHub Actions SSH Public Key" default="" />
#<UDF name="ACTIONS_JORDAN_SSH_PUB" label="Jordan SSH Public Key" default="" />
```

**GitHub Actions Workflow (Before Fix - Incorrect):**
```json
{
  "actions_user_ssh_pub": "${{ secrets.ACTIONS_USER_SSH_PUB }}",
  "actions_jordan_ssh_pub": "${{ secrets.ACTIONS_JORDAN_SSH_PUB }}"
}
```

**GitHub Actions Workflow (After Fix - Correct):**
```json
{
  "ACTIONS_USER_SSH_PUB": "${{ secrets.ACTIONS_USER_SSH_PUB }}",
  "ACTIONS_JORDAN_SSH_PUB": "${{ secrets.ACTIONS_JORDAN_SSH_PUB }}"
}
```

### Solution Applied
1. Updated `.github/workflows/00-complete.yml` to use correct UDF naming
2. Fixed the UDF data in the "Create Linode Server" step to match StackScript expectations exactly
3. Added comment explaining that UDF names must match exactly

### Key Learning
**Always ensure UDF names in GitHub Actions workflow match exactly what's defined in the StackScript.** Case sensitivity matters!

## 🚨 Common Issue: 401 Authentication Error

If you're seeing a `401 Unauthorized` error when the workflow tries to use the Linode CLI, this means there's an issue with your `LINODE_CLI_TOKEN` secret.

### Quick Diagnosis

Run this script to test your token locally:

```bash
./scripts/deployment/tools/verify-linode-token.sh YOUR_TOKEN_HERE
```

Or set it as an environment variable:

```bash
export LINODE_CLI_TOKEN="your_token_here"
./scripts/deployment/tools/verify-linode-token.sh
```

## 🔧 Step-by-Step Fix

### 1. Verify Token Exists in GitHub

1. Go to your repository on GitHub
2. Navigate to **Settings** → **Secrets and variables** → **Actions**
3. Look for `LINODE_CLI_TOKEN` in the repository secrets list
4. If missing, proceed to step 2

### 2. Create a New Linode API Token

1. Go to [Linode Cloud Manager → API Tokens](https://cloud.linode.com/profile/tokens)
2. Click **"Create a Personal Access Token"**
3. Configure the token:
   - **Label**: `GitHub Actions FKS Deploy`
   - **Expiry**: `Never` (or your preferred duration)
   - **Permissions** (CRITICAL):
     - ✅ **Linodes**: `Read/Write`
     - ✅ **StackScripts**: `Read/Write`
     - ✅ **Images**: `Read Only`
     - ❌ **Events**: Not required
     - ❌ **Account**: Not required (but harmless if enabled)
4. Click **"Create Token"**
5. **IMPORTANT**: Copy the token immediately (you won't see it again)

### 3. Add Token to GitHub Secrets

1. In your GitHub repository, go to **Settings** → **Secrets and variables** → **Actions**
2. Click **"New repository secret"**
3. Set:
   - **Name**: `LINODE_CLI_TOKEN`
   - **Secret**: Paste your Linode token
4. Click **"Add secret"**

### 4. Verify Token Permissions

Your token must have these exact permissions:

| Permission | Access Level | Why Needed |
|------------|--------------|------------|
| **Linodes** | Read/Write | Create/check/manage servers |
| **StackScripts** | Read/Write | Create/run setup scripts |
| **Images** | Read Only | List available OS images |

### 5. Test the Fix

1. Go to **Actions** tab in your repository
2. Find the **"Deploy to Development Server"** workflow
3. Click **"Run workflow"**
4. Use these settings for a quick test:
   - Server name: `fks-dev-test`
   - Create if missing: `true`
   - Linode type: `g6-standard-1` (cheaper for testing)
   - Region: `ca-central`

## 🔍 Common Token Issues

### Issue: Token Expired

**Symptoms**: 401 error, worked before
**Solution**: Create a new token with "Never" expiry or longer duration

### Issue: Insufficient Permissions

**Symptoms**: 403 error or specific permission errors
**Solution**: Delete old token, create new one with correct permissions above

### Issue: Token Not Set

**Symptoms**: Empty/null token error
**Solution**: Verify secret name is exactly `LINODE_CLI_TOKEN` (case-sensitive)

### Issue: Wrong Account

**Symptoms**: Can't find expected resources
**Solution**: Verify you're using the token from the correct Linode account

## 📝 Testing Checklist

Before running the deployment workflow, verify:

- [ ] `LINODE_CLI_TOKEN` secret exists in GitHub
- [ ] Token has `Linodes: Read/Write` permission
- [ ] Token has `StackScripts: Read/Write` permission  
- [ ] Token has `Images: Read Only` permission
- [ ] Token is not expired
- [ ] You can run `./scripts/deployment/tools/verify-linode-token.sh` successfully

## 🆘 Still Having Issues?

If the above steps don't resolve the issue:

1. **Double-check the token permissions** - This is the most common issue
2. **Try creating a completely new token** - Sometimes old tokens have hidden issues
3. **Verify you're in the right Linode account** - Check the account email matches expectations
4. **Check Linode service status** - Visit [Linode Status Page](https://status.linode.com/)

## 🔐 Security Best Practices

- ✅ Use tokens with minimal required permissions
- ✅ Set reasonable expiry dates (not "Never" for production)
- ✅ Rotate tokens periodically
- ✅ Delete unused tokens
- ❌ Never commit tokens to code
- ❌ Never share tokens in chat/email

## 📞 Getting Help

If you're still stuck:

1. Run `./scripts/deployment/tools/verify-linode-token.sh` and share the output (it won't reveal your token)
2. Check the GitHub Actions logs for specific error messages
3. Verify your Linode account has sufficient resources/limits

---

## Arch Linux 2-Stage Process Fix (Fixed: July 10, 2025)

### Problem
The GitHub Actions workflow was configured for Ubuntu but your StackScript is designed for **Arch Linux only** with a proven 2-stage process.

### Root Cause
1. Workflow was using `linode/ubuntu24.04` image instead of `linode/arch`
2. Wait times didn't account for the 2-stage process with auto-reboot
3. Deployment script assumed Ubuntu directory structure

### Solution Applied

#### 1. Fixed Server Image
**Before:**
```yaml
--image "linode/ubuntu24.04"
```

**After:**
```yaml
--image "linode/arch"  # Arch Linux - TESTED 100%
```

#### 2. Updated StackScript Configuration
**Before:**
```yaml
--images "linode/arch" \
--images "linode/ubuntu24.04" \
```

**After:**
```yaml
--images "linode/arch" \
```

#### 3. Fixed Wait Process for 2-Stage Setup
**Before:** Single 5-minute wait

**After:** Proper 2-stage wait process:
```yaml
# Phase 1: 8 minutes (package installation, user creation, services)
# Auto-reboot: 5 minutes (new kernel and modules loading)  
# Phase 2: 5 minutes (Tailscale, firewall, final configuration)
# Total: 18 minutes
```

#### 4. Updated Deployment Script
**Before:** Used `/opt/fks` directory

**After:** Uses `/home/jordan/fks` directory (created by StackScript)

### Your Tested 2-Stage Process
1. **Phase 1** (StackScript initial run):
   - Package installation (`pacman -Syu`)
   - User creation (`jordan`, `fks_user`, `actions_user`)
   - Service setup (Docker, fail2ban, etc.)
   - Creates Phase 2 systemd service
   - **Auto-reboot** to load new kernel modules

2. **Phase 2** (After reboot via systemd):
   - Firewall configuration with iptables
   - Tailscale setup and connection
   - Final system hardening
   - Welcome scripts and aliases

### Key Benefits
- ✅ Uses your 100% tested Arch Linux StackScript
- ✅ Proper timing for 2-stage process with reboot
- ✅ Matches your proven server configuration
- ✅ Correct directory structure and user permissions
- ✅ No more Ubuntu/Arch conflicts

## SSH Key Generation and Discord Alert Process (Added: July 10, 2025)

### Overview
After the 2-stage Arch Linux setup completes, the workflow now automatically generates SSH keys for all 4 users and sends a Discord notification with the public keys for you to update GitHub secrets.

### Process Flow

#### 1. **Stage 1 & 2 Complete**
- Phase 1: Package installation, user creation, services setup
- Auto-reboot: New kernel and modules loading
- Phase 2: Tailscale, firewall, final configuration

#### 2. **SSH Key Generation** 
The workflow connects to the server and generates ED25519 SSH keys for:
- `root` → `/root/.ssh/fks_server_key`
- `jordan` → `/home/jordan/.ssh/fks_server_key`  
- `fks_user` → `/home/fks_user/.ssh/fks_server_key`
- `actions_user` → `/home/actions_user/.ssh/fks_server_key`

#### 3. **Discord Notification**
Sends Discord alert with:
- Server IP and status
- All 4 public keys formatted for GitHub secrets
- Instructions for updating secrets and deploy keys

#### 4. **10-Minute Wait Period**
Workflow pauses for 10 minutes to allow you to:
- Update GitHub secrets with new public keys
- Add `actions_user` public key as deploy key to fks repo

#### 5. **SSH Key Testing**
Tests connectivity with new keys before deployment

#### 6. **Deployment Continues**
Proceeds with application deployment using updated authentication

### GitHub Secrets to Update

```
ACTIONS_ROOT_SSH_PUB      = root@fks-dev-YYYYMMDD public key
ACTIONS_JORDAN_SSH_PUB    = jordan@fks-dev-YYYYMMDD public key  
ACTIONS_FKS_SSH_PUB       = fks_user@fks-dev-YYYYMMDD public key
ACTIONS_USER_SSH_PUB      = actions_user@fks-dev-YYYYMMDD public key
```

### Deploy Key Setup
Add the `actions_user` public key as a deploy key to the fks repository for secure repo access.

### Benefits
- ✅ Secure SSH key authentication for all users
- ✅ Automated key generation and notification
- ✅ No manual SSH key management required
- ✅ Discord integration for immediate notification
- ✅ Proper timing with server setup completion
- ✅ Deploy key setup for secure repo access

---

# Troubleshooting Guide

## Common Issues and Fixes

### 1. Linode CLI Authentication Failed

**Error Message**:
```
❌ Linode CLI authentication failed - check LINODE_CLI_TOKEN secret
```

**Fix**:
- Ensure `LINODE_CLI_TOKEN` is set in GitHub Secrets
- Token must have `Linodes: Read/Write` and `StackScripts: Read/Write` permissions
- Verify token is not expired

### 2. UDF Naming Issues

**Error Message**:
```
Request failed: 400
actions_user_ssh_pub	This StackScript does not accept UDF actions_user_ssh_pub.
actions_jordan_ssh_pub	This StackScript does not accept UDF actions_jordan_ssh_pub.
```

**Fix**:
- Update GitHub Actions workflow to use uppercase UDF names
- Ensure UDF names match exactly with StackScript definition

### 3. SSH Key Issues

**Symptoms**:
- Unable to connect via SSH after deployment
- `Permission denied` errors

**Fix**:
- Ensure SSH keys are generated and added to GitHub secrets
- For `actions_user`, add the public key as a deploy key to the fks repository
- Test SSH connectivity using the new keys

### 4. Discord Notification Not Received

**Symptoms**:
- No alert received on Discord after deployment

**Fix**:
- Check Discord webhook URL and permissions
- Ensure the workflow has access to the webhook URL
- Verify the server is correctly configured to send notifications

## Debugging Tips

- Use the `--debug` flag in GitHub Actions to get more detailed logs
- Check the Linode Cloud Manager for any failed API requests or logs
- Review the deployment script for any hardcoded values or assumptions
- Temporarily add `set -x` in the script to enable debug output for commands

## When All Else Fails

1. **Re-run the workflow** - Sometimes transient issues can cause failures
2. **Check Linode status page** - Ensure there are no ongoing incidents or maintenance
3. **Contact Linode support** - If you suspect an issue with the Linode API or services
4. **Ask for help** - Reach out to your team or community for assistance

---
