# 🔧 SSH Key Variables Fix - Stage 1 Script

## 🐛 **Root Cause Identified**

The `unexpected EOF while looking for matching '"'` error was caused by **undefined SSH key variables** in the stage-1 setup script.

### **Problem Details:**
- SSH key variables (`$ORYX_SSH_PUB`, `$SULLIVAN_SSH_PUB`, etc.) were referenced but not defined
- These variables were being used inside the heredoc setup script
- When undefined variables are used in shell, they become empty strings
- This created malformed echo commands that broke shell syntax

### **Example of the Issue:**
```bash
# When $ORYX_SSH_PUB is undefined, this becomes:
echo "$ORYX_SSH_PUB" >> /path/to/authorized_keys

# Which becomes an empty echo:
echo "" >> /path/to/authorized_keys

# But in complex contexts, this caused quote mismatches
```

## ✅ **Solution Implemented**

### **1. Updated Stage-1 Script**
- Added SSH key parameters to script arguments
- Modified parameter parsing to accept SSH key options:
  - `--oryx-ssh-pub`
  - `--sullivan-ssh-pub` 
  - `--freddy-ssh-pub`
  - `--desktop-ssh-pub`
  - `--macbook-ssh-pub`

### **2. Updated GitHub Workflow**
- Added SSH key parameters to the script execution:
```yaml
$SCRIPT_PATH \
  --target-host "${{ steps.stage0.outputs.server_ip }}" \
  --root-password "$FKS_DEV_ROOT_PASSWORD" \
  --jordan-password "$JORDAN_PASSWORD" \
  --fks-user-password "$FKS_USER_PASSWORD" \
  --tailscale-auth-key "$TAILSCALE_AUTH_KEY" \
  --docker-username "$DOCKER_USERNAME" \
  --docker-token "$DOCKER_TOKEN" \
  --netdata-claim-token "$NETDATA_CLAIM_TOKEN" \
  --netdata-claim-room "$NETDATA_CLAIM_ROOM" \
  --timezone 'America/Toronto' \
  --oryx-ssh-pub "$ORYX_SSH_PUB" \
  --sullivan-ssh-pub "$SULLIVAN_SSH_PUB" \
  --freddy-ssh-pub "$FREDDY_SSH_PUB" \
  --desktop-ssh-pub "$DESKTOP_SSH_PUB" \
  --macbook-ssh-pub "$MACBOOK_SSH_PUB"
```

### **3. Enhanced SSH Key Handling**
- SSH keys are now passed as proper parameters
- Added conditional checks before using SSH keys:
```bash
if [ -n "$ORYX_SSH_PUB" ]; then
    echo "$ORYX_SSH_PUB" >> "$home_dir/.ssh/authorized_keys"
fi
```

## 🔄 **Data Flow Fixed**

### **Before (Broken):**
```
GitHub Secrets → Workflow ENV → ❌ Undefined in Script → Shell Syntax Error
```

### **After (Fixed):**
```
GitHub Secrets → Workflow ENV → Script Parameters → ✅ Properly Used in Setup
```

## 🎯 **Expected Results**

### **✅ Fixed Issues:**
- ❌ `unexpected EOF while looking for matching '"'`
- ❌ SSH key variables undefined
- ❌ Shell syntax errors in heredoc
- ✅ Clean parameter passing
- ✅ Proper SSH key setup for all users

### **🔧 Improved Features:**
- **Conditional SSH Keys**: Only add keys if provided (no empty entries)
- **Robust Parameter Handling**: Proper argument parsing with defaults
- **Environment Fallback**: Can still use environment variables if needed
- **Better Error Messages**: Clear parameter validation

## 🧪 **Testing Workflow**

1. **Push Changes**: `git push`
2. **Trigger Workflow**: Manual or automatic
3. **Monitor Stage 1**: Should pass SSH key generation
4. **Verify SSH Setup**: Check that all users have proper SSH keys
5. **Test SSH Access**: Verify GitHub Actions can connect

## 📝 **Key Changes Made**

### **Files Modified:**
- `scripts/deployment/staged/stage-1-initial-setup.sh` - Complete rewrite with proper parameter handling
- `.github/workflows/00-complete.yml` - Added SSH key parameters to script call

### **New Parameter Structure:**
- Required: `--target-host`, `--root-password`, `--jordan-password`, `--fks-user-password`, `--tailscale-auth-key`
- Optional: `--docker-username`, `--docker-token`, `--netdata-*`, `--*-ssh-pub`

The deployment should now successfully pass Stage 1 setup! 🚀
