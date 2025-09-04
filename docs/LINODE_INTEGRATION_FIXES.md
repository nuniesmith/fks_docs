# Linode Integration Fix Summary

## Issue Resolution

The GitHub Actions workflow was failing at the "Setup Linode Server" stage due to incorrect Linode CLI installation and configuration. The following fixes have been applied:

## 🔧 **Fixed Issues**

### 1. **Setup-Server Job Fixes**

**Problem**: Linode CLI was not properly installed or configured
- `linode-cli configure` was failing because CLI wasn't installed
- Missing proper PATH configuration for user-installed packages
- JSON parsing was unreliable

**Solution Applied**:
```bash
# Install Linode CLI with --user flag (GitHub Actions compatible)
pip3 install --user linode-cli
export PATH="$HOME/.local/bin:$PATH"

# Create proper configuration file
mkdir -p ~/.config/linode-cli
cat > ~/.config/linode-cli/config << EOF
[DEFAULT]
token = $LINODE_CLI_TOKEN
region = ca-central
type = g6-standard-2
image = linode/arch
EOF

# Use text format instead of JSON for better reliability
EXISTING_SERVERS=$(~/.local/bin/linode-cli linodes list --text --no-headers --format="label,ipv4" 2>/dev/null | grep -E "^fks" | head -1 || echo "")
```

### 2. **Create-Server Job Fixes**

**Problem**: Script execution method was incorrect
- Using non-existent `--env-file` parameter
- Incorrect server IP extraction method

**Solution Applied**:
```bash
# Set environment variables directly (the way the script expects)
export LINODE_CLI_TOKEN="${{ secrets.LINODE_CLI_TOKEN }}"
export FKS_DEV_ROOT_PASSWORD="${{ secrets.FKS_DEV_ROOT_PASSWORD }}"

# Use correct script parameters
./scripts/deployment/staged/stage-0-create-server.sh --target-server "auto-detect" --force-new

# Extract server details from the output file created by the script
if [ -f "server-details.env" ]; then
  source server-details.env
  echo "server_ip=$TARGET_HOST" >> $GITHUB_OUTPUT
fi
```

### 3. **Initial-Setup Job Fixes**

**Problem**: Incorrect parameter passing to stage-1 script
- Using non-existent `--env-file` parameter
- Wrong parameter names

**Solution Applied**:
```bash
# Set environment variables directly
export FKS_DEV_ROOT_PASSWORD="${{ secrets.FKS_DEV_ROOT_PASSWORD }}"
export TARGET_HOST="$TARGET_IP"

# Use correct script parameters based on the actual script
./scripts/deployment/staged/stage-1-initial-setup.sh --target-host "$TARGET_IP" --root-password "$FKS_DEV_ROOT_PASSWORD"
```

## 📋 **Script Compatibility Analysis**

### Stage 0 Script (`stage-0-create-server.sh`)
- **Expects**: Environment variables `LINODE_CLI_TOKEN` and `FKS_DEV_ROOT_PASSWORD`
- **Parameters**: `--target-server`, `--force-new`, `--custom-host`, etc.
- **Output**: Creates `server-details.env` file with server information
- **Runs On**: Ubuntu (GitHub Actions) but creates Arch Linux servers

### Stage 1 Script (`stage-1-initial-setup.sh`)  
- **Expects**: Environment variables and command-line parameters
- **Parameters**: `--target-host`, `--root-password`, etc.
- **Function**: Sets up users, SSH keys, packages on Arch Linux server
- **Runs On**: Arch Linux target server (via SSH)

## 🔄 **Workflow Flow Fixed**

The corrected deployment flow now follows this sequence:

1. **setup-server**: Uses Linode CLI to find existing servers or determine if new creation is needed
2. **create-server**: (If needed) Creates new Linode server using stage-0 script
3. **initial-setup**: Configures server using stage-1 script
4. **notify-keys**: Sends Discord notification with SSH key information
5. **wait-and-finalize**: Waits for automatic reboot and stage-2 completion
6. **deploy-application**: Deploys Docker services with iptables handling

## 🛡️ **Required Secrets**

The workflow requires these GitHub secrets:

- `LINODE_CLI_TOKEN` - Linode API token for server management
- `FKS_DEV_ROOT_PASSWORD` - Root password for new servers
- `DISCORD_WEBHOOK_URL` - (Optional) Discord notifications
- `DOCKER_USERNAME` - (Optional) Docker Hub credentials
- `DOCKER_TOKEN` - (Optional) Docker Hub credentials

## 🔍 **Verification Steps**

To verify the fixes:

1. **Manual Dispatch Test**: Use workflow_dispatch with different target_server options
2. **Auto-detect Test**: Should find existing FKS servers
3. **New Server Test**: Should create new server when none found
4. **Custom Host Test**: Should work with specific IP addresses

## 🚀 **Enhanced Features Maintained**

The fixes preserve all the enhanced features:

- ✅ Docker iptables handling (fix or disable options)
- ✅ Comprehensive error handling and logging
- ✅ Multiple deployment modes (auto-detect, custom, force-new)
- ✅ Discord notifications
- ✅ Stage-by-stage deployment visibility
- ✅ Health checks and verification

## 📝 **Testing Recommendations**

1. **Start with existing server**: Use `auto-detect-existing-only` mode first
2. **Test iptables options**: Try both `fix_docker_iptables` and `disable_iptables_for_dev`
3. **Verify notifications**: Check Discord messages are sent at key stages
4. **Monitor logs**: Watch for proper Linode CLI installation and configuration

The workflow should now successfully deploy to Linode servers without the CLI configuration issues that were causing failures.
