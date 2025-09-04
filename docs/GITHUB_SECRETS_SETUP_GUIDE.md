# GitHub Secrets Setup Guide

This guide explains how to configure the required GitHub secrets for the FKS Trading Systems deployment pipeline.

## 🔐 Required Secrets

### Core Deployment Secrets
These secrets are required for full deployment:

| Secret | Description | Where to get it |
|--------|-------------|-----------------|
| `LINODE_CLI_TOKEN` | Linode API token for server provisioning | [Linode Cloud Manager](https://cloud.linode.com/profile/tokens) |
| `FKS_DEV_ROOT_PASSWORD` | Root password for Linode servers | Create a strong password |
| `JORDAN_PASSWORD` | Password for jordan user account | Create a strong password |
| `FKS_USER_PASSWORD` | Password for fks_user account | Create a strong password |
| `DOCKER_USERNAME` | Docker Hub username | Your Docker Hub username |
| `DOCKER_TOKEN` | Docker Hub access token | [Docker Hub Settings](https://hub.docker.com/settings/security) |
| `TAILSCALE_AUTH_KEY` | Tailscale authentication key for VPN | [Tailscale Admin Console](https://login.tailscale.com/admin/settings/keys) |

### Optional Secrets
These secrets enable additional features but are not required:

| Secret | Description | Required For |
|--------|-------------|--------------|
| `DOMAIN_NAME` | Your domain name (e.g., example.com) | SSL certificates |
| `ADMIN_EMAIL` | Email for Let's Encrypt certificates | SSL certificates |
| `CLOUDFLARE_API_TOKEN` | Cloudflare API token | DNS management |
| `CLOUDFLARE_ZONE_ID` | Cloudflare zone ID | DNS management |
| `NETDATA_CLAIM_TOKEN` | Netdata cloud claim token | Monitoring |
| `NETDATA_CLAIM_ROOM` | Netdata cloud room ID | Monitoring |
| `JWT_SECRET_KEY` | Secret key for JWT tokens | API authentication |
| `RITHMIC_USERNAME` | Rithmic trading username | Trading functionality |
| `RITHMIC_PASSWORD` | Rithmic trading password | Trading functionality |
| `DISCORD_WEBHOOK_SERVERS` | Discord webhook for server alerts | Discord notifications |
| `DISCORD_WEBHOOK_SIGNALS` | Discord webhook for trading signals | Discord notifications |

### SSH Keys (Optional)
These SSH keys enable direct server access:

| Secret | Description |
|--------|-------------|
| `ACTIONS_USER_SSH_PUB` | Public SSH key for actions_user |
| `ORYX_SSH_PUB` | Public SSH key for oryx access |
| `SULLIVAN_SSH_PUB` | Public SSH key for sullivan access |
| `FREDDY_SSH_PUB` | Public SSH key for freddy access |
| `DESKTOP_SSH_PUB` | Public SSH key for desktop access |
| `MACBOOK_SSH_PUB` | Public SSH key for macbook access |

## 🛠️ How to Add Secrets

1. **Navigate to Repository Settings**
   - Go to your GitHub repository
   - Click on **Settings** tab
   - Click on **Secrets and variables** → **Actions**

2. **Add New Secret**
   - Click **New repository secret**
   - Enter the secret name (exactly as shown above)
   - Enter the secret value
   - Click **Add secret**

3. **Repeat for all required secrets**

## 🚀 Deployment Modes

The workflow supports different deployment modes based on available secrets:

### Code Check Only
- **Mode**: `code-check-only`
- **Required secrets**: None
- **What it does**: Runs code quality checks only

### Builds Only
- **Mode**: `builds-only`
- **Required secrets**: `DOCKER_USERNAME`, `DOCKER_TOKEN`
- **What it does**: Builds and pushes Docker images

### Full Deploy
- **Mode**: `full-deploy`
- **Required secrets**: All core deployment secrets
- **What it does**: Complete infrastructure provisioning and deployment

## 🔍 Getting Secret Values

### Linode CLI Token
1. Go to [Linode Cloud Manager](https://cloud.linode.com/profile/tokens)
2. Click **Create a Personal Access Token**
3. Set expiry and scopes (needs read/write access for Linodes, Domains, etc.)
4. Copy the generated token

### Docker Hub Token
1. Go to [Docker Hub Settings](https://hub.docker.com/settings/security)
2. Click **New Access Token**
3. Enter a description and set permissions
4. Copy the generated token

### Tailscale Auth Key
1. Go to [Tailscale Admin Console](https://login.tailscale.com/admin/settings/keys)
2. Click **Generate auth key**
3. Set expiry and options as needed
4. Copy the generated key

### Cloudflare API Token
1. Go to [Cloudflare API Tokens](https://dash.cloudflare.com/profile/api-tokens)
2. Click **Create Token**
3. Use the **Custom token** template
4. Set permissions: `Zone:Zone:Read`, `Zone:DNS:Edit`
5. Include your specific zone
6. Copy the generated token

### Cloudflare Zone ID
1. Go to your domain in Cloudflare dashboard
2. Scroll down to **API** section on the right sidebar
3. Copy the **Zone ID**

## ⚠️ Security Best Practices

1. **Use strong passwords** for user accounts
2. **Set expiry dates** for tokens when possible
3. **Use minimal permissions** for API tokens
4. **Rotate secrets regularly**
5. **Never share secrets** in code or logs
6. **Use organization secrets** for shared repositories

## 🆘 Troubleshooting

### Common Issues

1. **"Missing required secrets" error**
   - Check secret names are exactly as listed (case-sensitive)
   - Verify secrets are added to the correct repository

2. **"Linode API connection failed"**
   - Verify `LINODE_CLI_TOKEN` has correct permissions
   - Check token hasn't expired

3. **"Docker login failed"**
   - Verify `DOCKER_USERNAME` and `DOCKER_TOKEN` are correct
   - Ensure Docker Hub account is active

4. **"SSH connection failed"**
   - Check passwords are correct
   - Verify server is running and accessible

## 📞 Support

If you need help setting up secrets:
1. Check the workflow logs for specific error messages
2. Verify secret values are correct
3. Try running in `code-check-only` mode first
4. Check Discord notifications for additional context

---

**Note**: This guide is specific to the FKS Trading Systems deployment pipeline. Secret requirements may change based on the deployment mode selected.
