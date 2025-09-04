# GitHub Actions Workflow Fixes

## Issues Found and Fixed

### 1. Execution Plan Logic Error
**Problem**: The `skip_deploy` flag was incorrectly set to `true` for `infra-only` mode, preventing infrastructure provisioning.

**Original Logic**:
```bash
if [[ "$DEPLOYMENT_MODE" == "builds-only" || "$DEPLOYMENT_MODE" == "infra-only" ]]; then
  SKIP_DEPLOY=true
fi
```

**Fixed Logic**:
```bash
if [[ "$DEPLOYMENT_MODE" == "builds-only" ]]; then
  SKIP_DEPLOY=true
fi
```

**Result**: Now `infra-only` mode will properly run infrastructure provisioning without skipping deployment tasks.

### 2. Infrastructure Provisioning Conditional
**Problem**: Complex conditional logic that could prevent infrastructure provisioning.

**Original**:
```yaml
if: needs.preflight-checks.outputs.skip_deploy != 'true' && (github.event.inputs.deployment_mode == 'full-deploy' || github.event.inputs.deployment_mode == 'infra-only' || github.event.inputs.create_new_server == 'true')
```

**Fixed**:
```yaml
if: github.event.inputs.deployment_mode == 'full-deploy' || github.event.inputs.deployment_mode == 'infra-only' || github.event.inputs.create_new_server == 'true'
```

**Result**: More explicit and reliable infrastructure provisioning trigger.

### 3. Build Job Dependencies
**Problem**: Build jobs were dependent on infrastructure provisioning, creating unnecessary coupling.

**Original**:
```yaml
needs: [preflight-checks, provision-infrastructure]
```

**Fixed**:
```yaml
needs: [preflight-checks]
```

**Result**: Build jobs can run independently of infrastructure provisioning.

### 4. Deploy Job Conditional Logic
**Problem**: Complex conditional that could prevent deployment.

**Fixed**:
```yaml
if: always() && (github.event.inputs.deployment_mode == 'full-deploy' || github.event.inputs.deployment_mode == 'deploy-only') && (needs.build-nginx.result == 'success' || needs.preflight-checks.outputs.skip_builds == 'true') && (needs.provision-infrastructure.result == 'success' || needs.provision-infrastructure.result == 'skipped')
```

**Result**: Deployment will now run when expected and handle both infrastructure success and skip scenarios.

## Workflow Modes Now Working

### 1. `full-deploy` (Default)
- ✅ **Infrastructure**: Creates new Linode server (`fks-dev`) in Toronto
- ✅ **Builds**: Builds all Docker images (API, Worker, Web, Nginx)
- ✅ **Deployment**: Deploys application to the server

### 2. `infra-only`
- ✅ **Infrastructure**: Creates new Linode server
- ❌ **Builds**: Skipped
- ❌ **Deployment**: Skipped

### 3. `deploy-only`
- ❌ **Infrastructure**: Skipped (uses existing server)
- ❌ **Builds**: Skipped (uses existing images)
- ✅ **Deployment**: Deploys to existing server

### 4. `builds-only`
- ❌ **Infrastructure**: Skipped
- ✅ **Builds**: Builds all Docker images
- ❌ **Deployment**: Skipped

### 5. Manual `create_new_server: true`
- ✅ **Infrastructure**: Creates new server regardless of mode
- Follows normal mode logic for builds/deployment

## Testing the Fix

To test that the workflow now works correctly:

1. **Full Deployment with New Server**:
   - Go to GitHub Actions → Run workflow
   - Select `full-deploy` mode
   - Check `create_new_server: true`
   - This should create a new `fks-dev` server in Toronto and deploy the application

2. **Infrastructure Only**:
   - Select `infra-only` mode
   - This should only create the server without building or deploying

3. **Deploy to Existing Server**:
   - Select `deploy-only` mode
   - This should deploy to an existing server without rebuilding

## Expected Server Configuration

When the workflow runs successfully with infrastructure provisioning:

- **Server Name**: `fks-dev`
- **Hostname**: `fks`
- **Location**: Toronto, Canada (`ca-central`)
- **Type**: `g6-standard-2` (4GB RAM, 2 CPUs)
- **OS**: Ubuntu 24.04
- **Services**: Docker, Tailscale, Netdata (if tokens provided)
- **Users**: `jordan` and `fks_user` with SSH key access
- **Application**: FKS Trading Systems running on Docker Compose

The server will be accessible via the IP address output by the workflow, and all your GitHub secrets will be properly configured during setup.
