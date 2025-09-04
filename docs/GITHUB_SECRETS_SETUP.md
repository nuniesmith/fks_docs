# 🔐 GitHub Actions Secrets Setup Guide

## Required Secrets (Workflow will fail without these)

| Secret Name | Description | Required For |
|-------------|-------------|--------------|
| `LINODE_CLI_TOKEN` | Linode API token for server management | Server creation/management |
| `FKS_DEV_ROOT_PASSWORD` | Root password for new servers | Server access |
| `JORDAN_PASSWORD` | Password for jordan user account | User account creation |
| `FKS_USER_PASSWORD` | Password for fks_user account | User account creation |
| `TAILSCALE_AUTH_KEY` | Tailscale authentication key | VPN setup |

## Optional Secrets (Features will be skipped if missing)

### Docker Building
| Secret Name | Description | Impact if Missing |
|-------------|-------------|-------------------|
| `DOCKER_USERNAME` | Docker Hub username | Docker builds skipped |
| `DOCKER_TOKEN` | Docker Hub access token | Docker builds skipped |

### SSL/Domain Setup
| Secret Name | Description | Impact if Missing |
|-------------|-------------|-------------------|
| `CLOUDFLARE_API_TOKEN` | Cloudflare API token | SSL setup skipped |
| `CLOUDFLARE_ZONE_ID` | Cloudflare zone ID | SSL setup skipped |
| `ADMIN_EMAIL` | Email for SSL certificates | SSL certificates cannot be generated |
| `DOMAIN_NAME` | Custom domain (default: fkstrading.xyz) | Uses default domain |
| `SSL_STAGING` | Use staging SSL (true/false) | Uses production SSL |

### Monitoring & Notifications
| Secret Name | Description | Impact if Missing |
|-------------|-------------|-------------------|
| `NETDATA_CLAIM_TOKEN` | Netdata monitoring token | Monitoring skipped |
| `NETDATA_CLAIM_ROOM` | Netdata room ID | Monitoring skipped |
| `DISCORD_WEBHOOK_SERVERS` | Discord webhook URL | No Discord notifications |

### SSH Key (Generated During Deployment)
| Secret Name | Description | Impact if Missing |
|-------------|-------------|-------------------|
| `ACTIONS_USER_SSH_PUB` | SSH public key for GitHub Actions | Will be generated and provided in Discord |

## 🚀 Quick Setup Steps

1. **Go to your repository secrets:**
   ```
   https://github.com/YOUR_USERNAME/YOUR_REPO/settings/secrets/actions
   ```

2. **Add required secrets first** (workflow will fail without these):
   - `LINODE_CLI_TOKEN`
   - `FKS_DEV_ROOT_PASSWORD` 
   - `JORDAN_PASSWORD`
   - `FKS_USER_PASSWORD`
   - `TAILSCALE_AUTH_KEY`

3. **Add optional secrets** for additional features:
   - Docker: `DOCKER_USERNAME`, `DOCKER_TOKEN`
   - SSL: `CLOUDFLARE_API_TOKEN`, `CLOUDFLARE_ZONE_ID`, `ADMIN_EMAIL`
   - Monitoring: `NETDATA_CLAIM_TOKEN`, `NETDATA_CLAIM_ROOM`
   - Notifications: `DISCORD_WEBHOOK_SERVERS`

4. **SSH Key will be provided during deployment** - The workflow will generate SSH keys and send them via Discord notification. You'll need to copy the `ACTIONS_USER_SSH_PUB` key to your repository secrets.

## 🔍 Automatic Validation

The workflow now includes automatic secrets validation that will:
- ✅ Check all required secrets are present
- ⚠️ Warn about missing optional secrets  
- 🚫 Stop workflow if required secrets are missing
- 📋 Provide clear instructions for missing secrets

## 🔧 Secret Name Changes

**Note:** If you previously had these secrets with different names, please rename them:
- `DOCKER_USERNAME` → `DOCKER_USERNAME`
- `DOCKER_TOKEN` → `DOCKER_TOKEN`
- `ACTIONS_GITHUB_USER_SSH_PUB` → `ACTIONS_USER_SSH_PUB`

The workflow has been updated to use the correct names that match your current secrets.
