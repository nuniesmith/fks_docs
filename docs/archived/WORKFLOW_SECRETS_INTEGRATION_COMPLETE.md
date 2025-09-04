# FKS Workflow - Complete Secret Integration

## Summary of Changes

The GitHub Actions workflow has been updated to include ALL available secrets and properly configure the FKS server deployment.

## ✅ Fixed Issues

### 1. **Server Creation Dependencies**
- **Problem**: `create-server` job was being skipped due to `docker-build` dependency issues
- **Solution**: Removed `docker-build` dependency from `setup-server` job
- **Result**: Server creation now works independently of Docker builds

### 2. **Server Naming and Hostname**
- **Problem**: Server names were timestamped and hostname wasn't set
- **Solution**: 
  - Server name: `fks-dev` (consistent, easy to find)
  - Hostname: `fks` (set via `hostnamectl` after creation)
- **Result**: Clean, predictable server identification

### 3. **Missing Secrets Integration**
- **Problem**: Stage 1 setup was failing due to missing required parameters
- **Solution**: Added ALL available secrets to the workflow
- **Result**: Complete secret integration for all deployment stages

## 🔐 Secrets Now Used

| Secret Name | Purpose | Stage |
|-------------|---------|-------|
| `LINODE_CLI_TOKEN` | Linode API access for server management | Server Detection & Creation |
| `FKS_DEV_ROOT_PASSWORD` | Root access to newly created servers | Server Access |
| `TAILSCALE_AUTH_KEY` | Tailscale VPN network integration | Stage 1 Setup |
| `JORDAN_PASSWORD` | Jordan user account password | Stage 1 Setup |
| `FKS_USER_PASSWORD` | FKS user account password | Stage 1 Setup |
| `NETDATA_CLAIM_TOKEN` | Netdata monitoring integration | Stage 1 Setup |
| `NETDATA_CLAIM_ROOM` | Netdata monitoring room/space | Stage 1 Setup |
| `DISCORD_WEBHOOK_URL` | Discord notifications (optional) | Notifications |
| `DOCKER_USERNAME` | Docker Hub registry access | Docker Build |
| `DOCKER_TOKEN` | Docker Hub registry access | Docker Build |

## 🔧 Initial Setup Script Parameters

The `stage-1-initial-setup.sh` script now receives all required parameters:

```bash
./scripts/deployment/staged/stage-1-initial-setup.sh \
  --target-host "$TARGET_IP" \
  --root-password "$FKS_DEV_ROOT_PASSWORD" \
  --tailscale-auth-key "$TAILSCALE_AUTH_KEY" \
  --jordan-password "$JORDAN_PASSWORD" \
  --fks-user-password "$FKS_USER_PASSWORD"
```

## 🌐 Environment Variables

All secrets are properly exported as environment variables:

```bash
export FKS_DEV_ROOT_PASSWORD="${{ secrets.FKS_DEV_ROOT_PASSWORD }}"
export TAILSCALE_AUTH_KEY="${{ secrets.TAILSCALE_AUTH_KEY }}"
export JORDAN_PASSWORD="${{ secrets.JORDAN_PASSWORD }}"
export FKS_USER_PASSWORD="${{ secrets.FKS_USER_PASSWORD }}"
export NETDATA_CLAIM_TOKEN="${{ secrets.NETDATA_CLAIM_TOKEN }}"
export NETDATA_CLAIM_ROOM="${{ secrets.NETDATA_CLAIM_ROOM }}"
export TARGET_HOST="$TARGET_IP"
```

## 📋 Server Configuration

### Server Creation
- **Label**: `fks-dev`
- **Type**: `g6-standard-2`
- **Region**: `ca-central` 
- **Image**: `linode/arch`
- **Hostname**: `fks` (set after creation)

### Server Search
- Searches for servers with labels matching: `fks-dev` or `fks`
- Prioritizes running servers over stopped ones
- Provides clear logging of all found servers

## 🚀 Deployment Flow

1. **Stage 0: Build Preparation** - Detect changes, build Docker images if needed
2. **Stage 2: Server Detection** - Check for existing running servers
3. **Stage 3: Server Creation** - Create new server if needed (`fks-dev`)
4. **Stage 4: Initial Setup** - Run Stage 1 with ALL secrets and parameters
5. **Stage 5: Notifications** - Send Discord notification for new servers
6. **Stage 6: Wait & Finalize** - Wait for reboot and Stage 2 completion
7. **Stage 7: Application Deployment** - Deploy Docker services and verify

## ✅ Expected Results

After these changes:
- ✅ Server creation will work properly
- ✅ All required secrets will be available to the setup script
- ✅ Tailscale integration will work
- ✅ User accounts (jordan, fks_user) will be created with correct passwords
- ✅ Netdata monitoring will be configured
- ✅ Server will have consistent naming (`fks-dev`) and hostname (`fks`)
- ✅ Discord notifications will be sent for new server setups

## 🔄 Mode Options

- **`auto-detect`** (default): Use existing running server OR create new if none
- **`auto-detect-no-create`**: Only use existing running servers, skip if none
- **`force-new`**: Always create new server
- **`custom`**: Use specific IP/hostname

## 📝 Additional Notes

1. **Netdata Integration**: The `NETDATA_CLAIM_TOKEN` and `NETDATA_CLAIM_ROOM` are now available for monitoring setup
2. **User Management**: Both `JORDAN_PASSWORD` and `FKS_USER_PASSWORD` are properly passed for user creation
3. **Network Integration**: `TAILSCALE_AUTH_KEY` enables VPN connectivity
4. **Docker Integration**: Docker Hub credentials support image publishing
5. **Notifications**: Discord webhooks provide deployment status updates

The workflow is now fully configured with all available secrets and should complete the entire deployment process successfully.
