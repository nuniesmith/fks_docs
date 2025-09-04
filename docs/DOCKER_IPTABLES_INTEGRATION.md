# Docker iptables Integration Guide

## Overview

The FKS Trading Systems GitHub Actions workflow now includes comprehensive Docker iptables handling to prevent common networking issues that can occur during deployment.

## Features Added

### 1. Workflow Configuration Options

Two new workflow inputs have been added to `00-complete.yml`:

- **`fix_docker_iptables`** (default: `true`)
  - Runs the Docker iptables fix script during deployment
  - Resolves common Docker networking chain issues
  - Uses the existing `fix-docker-iptables.sh` script

- **`disable_iptables_for_dev`** (default: `false`) 
  - Completely disables iptables/firewall for development environments
  - ⚠️ **WARNING**: Should only be used in development/testing environments
  - Not recommended for production deployments

### 2. Automatic iptables Handling

The workflow now includes three modes of operation:

#### Mode 1: Development Mode (iptables disabled)
```yaml
disable_iptables_for_dev: true
```
- Stops and disables iptables/ufw firewall services
- Provides maximum compatibility for development
- Shows warning about security implications

#### Mode 2: Production Mode with iptables fix (default)
```yaml
fix_docker_iptables: true
disable_iptables_for_dev: false
```
- Runs the comprehensive iptables fix script
- Cleans broken Docker chains
- Restarts Docker with proper networking
- Verifies Docker networking functionality

#### Mode 3: Manual Mode (no automatic handling)
```yaml
fix_docker_iptables: false
disable_iptables_for_dev: false
```
- Skips all iptables configuration
- Assumes manual setup or existing working configuration

## Implementation Details

### Deployment Stage Integration

The iptables handling is integrated into the **"Deploy Docker services"** stage of the workflow:

1. **Pre-deployment iptables configuration**
   - Checks workflow configuration flags
   - Runs appropriate iptables handling mode
   - Provides verbose logging for troubleshooting

2. **Docker startup with verification**
   - Starts Docker daemon
   - Waits for Docker to be ready (30s timeout)
   - Tests Docker networking functionality
   - Verifies Docker can create networks

3. **Application deployment**
   - Only proceeds if Docker networking is confirmed working
   - Deploys FKS services using existing deployment scripts

### Error Handling

The workflow includes robust error handling:

- **Graceful degradation**: If iptables fix fails, deployment continues with warnings
- **Timeout protection**: Docker startup has a 30-second timeout
- **Verification steps**: Multiple checks ensure Docker is working before deployment
- **Detailed logging**: Verbose output for troubleshooting issues

## Usage Examples

### Manual Workflow Dispatch

When manually triggering the workflow, you can configure the iptables handling:

#### For Development/Testing
```yaml
target_server: "auto-detect"
fix_docker_iptables: false
disable_iptables_for_dev: true
```

#### For Production with iptables fix
```yaml
target_server: "auto-detect"
fix_docker_iptables: true
disable_iptables_for_dev: false
```

#### For Production with existing setup
```yaml
target_server: "custom"
custom_host: "your-server-ip"
fix_docker_iptables: false
disable_iptables_for_dev: false
```

### Automatic Push Deployment

For automatic deployments (push to main/develop), the workflow uses defaults:
- `fix_docker_iptables: true` (runs iptables fix)
- `disable_iptables_for_dev: false` (keeps security)

## Troubleshooting

### Common iptables Issues Resolved

The integration handles these common Docker iptables errors:

1. **"No chain/target/match by that name"**
   - Caused by broken Docker iptables chains
   - Fixed by cleaning and recreating chains

2. **"Failed to Setup IP tables: Unable to enable DROP INCOMING rule"**
   - Caused by conflicting iptables rules
   - Fixed by resetting Docker networking state

3. **Network creation failures during docker-compose up**
   - Caused by stale network interfaces
   - Fixed by cleaning Docker bridges and restarting

### Debugging Steps

If deployment fails with Docker networking issues:

1. Check the workflow logs for iptables handling output
2. Look for Docker daemon startup errors
3. Review the networking verification step results
4. Consider using development mode temporarily to isolate issues

### Manual iptables Fix

If you need to run the iptables fix manually on a server:

```bash
# SSH to your server
ssh root@your-server-ip

# Navigate to the FKS directory
cd /home/fks_user/fks

# Run the iptables fix script
sudo ./scripts/deployment/staged/fix-docker-iptables.sh --verbose

# Verify Docker networking
docker network ls
docker run --rm alpine:latest echo "Docker test successful"
```

## Security Considerations

### Production Deployments
- Always use `fix_docker_iptables: true` in production
- Never use `disable_iptables_for_dev: true` in production
- The iptables fix maintains security while resolving networking issues

### Development Environments
- `disable_iptables_for_dev: true` can be used for quick testing
- Remember to re-enable iptables for any internet-facing development servers
- Consider using isolated development environments when disabling iptables

### Network Security
- The iptables fix script creates backups before making changes
- Docker networking rules are rebuilt according to Docker's standard configuration
- Custom iptables rules may need to be reapplied after the fix

## Script Details

The workflow utilizes the existing `fix-docker-iptables.sh` script with these features:

- **Backup creation**: Saves current iptables rules before changes
- **Clean shutdown**: Properly stops Docker before cleaning chains
- **Comprehensive cleanup**: Removes broken chains from filter, nat, and mangle tables
- **Interface cleanup**: Removes stale Docker bridge interfaces
- **Verification**: Tests Docker functionality after the fix
- **Cross-platform**: Works on Ubuntu, Debian, CentOS, and Arch Linux

## Future Enhancements

Potential improvements to consider:

1. **Custom iptables rules preservation**: Save and restore custom rules
2. **Network policy integration**: Support for Kubernetes-style network policies
3. **Monitoring integration**: Report iptables fix events to monitoring systems
4. **Rollback capability**: Automatic rollback if networking verification fails

## Related Documentation

- [`DOCKER_IPTABLES_FIX.md`](DOCKER_IPTABLES_FIX.md) - Detailed iptables fix script documentation
- [`DEPLOYMENT_GUIDE.md`](DEPLOYMENT_GUIDE.md) - Overall deployment process
- [`TROUBLESHOOTING_GUIDE.md`](TROUBLESHOOTING_GUIDE.md) - General troubleshooting
