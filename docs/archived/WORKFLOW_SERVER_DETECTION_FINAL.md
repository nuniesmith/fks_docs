# FKS Workflow Server Detection - Final Implementation

## Overview

The GitHub Actions workflow has been updated with robust server detection logic that properly connects to Linode using the CLI token to check for running servers and creates new ones when needed.

## Server Detection Modes

### 1. `auto-detect` (Default)
- **Behavior**: Preferred mode for most deployments
- **Logic**:
  - Connects to Linode using CLI token
  - Lists all FKS servers and checks their status
  - If a running server is found: Uses it for deployment
  - If no running servers found: Creates a new server
  - If no servers exist at all: Creates a new server
- **Use Cases**: Standard deployments, CI/CD automation

### 2. `auto-detect-no-create`
- **Behavior**: Conservative mode that avoids creating new servers
- **Logic**:
  - Connects to Linode using CLI token
  - Lists all FKS servers and checks their status
  - If a running server is found: Uses it for deployment
  - If no running servers found: Skips deployment entirely
  - If no servers exist at all: Skips deployment entirely
- **Use Cases**: Testing workflows, avoiding unexpected server costs

### 3. `force-new`
- **Behavior**: Always creates a fresh server
- **Logic**:
  - Ignores any existing servers (even if running)
  - Always creates a new Linode server
  - Useful for testing or creating parallel environments
- **Use Cases**: Testing new configurations, creating staging environments

### 4. `custom`
- **Behavior**: Use a specific server IP/hostname
- **Logic**:
  - Skips all Linode detection/creation
  - Uses the provided custom_host input
  - Assumes the server is ready and accessible
- **Use Cases**: Deploying to non-Linode servers, using existing servers

## Technical Implementation

### Server Detection Process

1. **Linode CLI Setup**:
   ```bash
   pip3 install --user linode-cli
   export PATH="$HOME/.local/bin:$PATH"
   
   # Configure with token
   mkdir -p ~/.config/linode-cli
   cat > ~/.config/linode-cli/config << EOF
   [DEFAULT]
   token = $LINODE_CLI_TOKEN
   region = ca-central
   type = g6-standard-2
   image = linode/arch
   EOF
   ```

2. **Server Search**:
   ```bash
   # List all FKS servers with status and IPs
   EXISTING_SERVERS=$(~/.local/bin/linode-cli linodes list --text --no-headers --format="label,status,ipv4" 2>/dev/null | grep -i "fks" || echo "")
   
   # Look for running servers specifically
   RUNNING_SERVER=$(echo "$EXISTING_SERVERS" | grep -E "(running|Running)" | head -1)
   ```

3. **Decision Logic**:
   - Prioritizes running servers over stopped ones
   - Provides clear logging of all found servers
   - Makes decisions based on the selected mode
   - Sets appropriate output variables for downstream jobs

### Job Dependencies and Skipping

The workflow properly handles deployment skipping through the `deployment_skipped` output:

- **setup-server**: Sets `deployment_skipped=true` when appropriate
- **create-server**: Only runs if `deployment_skipped != 'true'`
- **initial-setup**: Only runs if `deployment_skipped != 'true'`
- **All downstream jobs**: Depend on successful completion of earlier jobs

### Output Variables

The `setup-server` job outputs:
- `target_host`: IP address of the server to use
- `is_new_server`: Boolean indicating if a new server will be created
- `server_ready`: Boolean indicating if an existing server is ready
- `deployment_skipped`: Boolean indicating if deployment should be skipped

## Error Handling

### Missing Secrets
The workflow validates required secrets before attempting any operations:
- `LINODE_CLI_TOKEN`: Required for all Linode operations
- `FKS_DEV_ROOT_PASSWORD`: Required for server access

### Linode CLI Failures
- Network timeouts are handled gracefully
- Invalid tokens result in clear error messages
- Failed server creation stops the workflow with appropriate errors

### Server Creation Issues
- SSH availability is tested with timeouts
- Server IP extraction is validated
- Boot process includes stabilization delays

## Logging and Visibility

### Decision Summary
Each run provides a clear summary:
```
🎯 Decision Summary:
  Target Server Mode: auto-detect
  Target Host: 192.168.1.100
  Is New Server: false
  Server Ready: true
  Deployment Skipped: false
```

### Server Listing
When servers are found, they are clearly displayed:
```
📋 Found FKS servers:
  - fks-trading-20250107-123456 running [192.168.1.100]
  - fks-trading-20250106-234567 shutdown [192.168.1.101]
```

## Workflow Triggers

### Automatic (Push to main/develop)
- Uses `auto-detect` mode by default
- Will create servers if none are running
- Suitable for production deployments

### Manual (workflow_dispatch)
- Allows selection of any mode
- Provides full control over server behavior
- Includes additional options like Docker rebuilding

## Benefits of This Implementation

1. **Cost Control**: `auto-detect-no-create` mode prevents unexpected server creation
2. **Reliability**: Always checks actual server status via Linode API
3. **Flexibility**: Multiple modes for different use cases
4. **Visibility**: Clear logging of all decisions and server states
5. **Safety**: Proper secret validation and error handling
6. **Efficiency**: Reuses running servers when appropriate

## Usage Examples

### Standard Deployment
```yaml
# Triggered by push to main - uses auto-detect mode
# Will reuse running server or create new one if needed
```

### Testing Workflow
```yaml
# Manual trigger with auto-detect-no-create
# Will only deploy if a running server already exists
```

### Fresh Environment
```yaml
# Manual trigger with force-new
# Always creates a brand new server for testing
```

### Custom Server
```yaml
# Manual trigger with custom mode
# Deploys to specified IP address (e.g., existing dev server)
```

## Migration from Previous Version

The old mode names have been updated for clarity:
- `auto-detect-existing-only` → `auto-detect-no-create`
- Default changed from `auto-detect-existing-only` to `auto-detect`

This change makes the default behavior more intuitive (create servers when needed) while still providing the conservative option.
