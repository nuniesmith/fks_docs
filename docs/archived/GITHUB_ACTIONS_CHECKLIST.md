# GitHub Actions Setup Checklist

## 📋 Quick Setup Steps

### 1. Set up Tailscale OAuth for GitHub Actions

1. Go to [Tailscale Admin Console](https://login.tailscale.com/admin)
2. Navigate to **Settings** → **OAuth clients**
3. Click **Generate OAuth client**
4. Configure the client:
   - **Name**: `GitHub Actions CI/CD`
   - **Scopes**: `devices:write`, `all:read`
   - **Tags**: `tag:ci-cd`
5. Copy the **Client ID** and **Client Secret**

### 2. Generate SSH Key for CI/CD

```bash
# Generate a new SSH key for CI/CD
ssh-keygen -t ed25519 -f ~/.ssh/ninja-ci-cd -N ""

# Copy public key to your ninja server
ssh-copy-id -i ~/.ssh/ninja-ci-cd.pub ninja@ninja.tailfef10.ts.net

# Get private key content for GitHub secret
cat ~/.ssh/ninja-ci-cd
```

### 3. Add GitHub Repository Secrets

Go to: **Settings** → **Secrets and variables** → **Actions** → **Repository secrets**

Add these secrets:

| Secret Name | Value | Description |
|-------------|-------|-------------|
| `TAILSCALE_OAUTH_CLIENT_ID` | From step 1 | Tailscale OAuth client ID |
| `TAILSCALE_OAUTH_SECRET` | From step 1 | Tailscale OAuth client secret |
| `NINJA_SSH_PRIVATE_KEY` | From step 2 | SSH private key for ninja user |
| `DISCORD_WEBHOOK` | Your Discord webhook URL | (Optional) For notifications |

### 4. Test the Setup

1. Push a commit to the `main` branch
2. Check **Actions** tab in GitHub
3. Verify the workflow runs successfully
4. Check your ninja server to confirm deployment

### 5. Manual Production Deployment

1. Go to **Actions** → **FKS Trading Systems CI/CD**
2. Click **Run workflow**
3. Select `production` environment
4. Click **Run workflow**

## 🔧 Troubleshooting

### SSH Connection Test
```bash
ssh ninja@ninja.tailfef10.ts.net
```

### Check Tailscale Status
```bash
sudo tailscale status
```

### Verify Repository Access
```bash
cd /home/ninja/ninja
git pull origin main
```

## 📁 Updated Deployment Paths

The workflow now uses the correct paths from your StackScript:

- **Repository**: `/home/ninja/ninja`
- **User**: `ninja`
- **SSH**: `ninja@ninja.tailfef10.ts.net`
- **Docker**: Handled gracefully if not running

## 🎯 Key Improvements

✅ Uses Tailscale DNS name for secure connections  
✅ No public SSH exposure required  
✅ Handles Docker startup issues gracefully  
✅ Deploys to correct StackScript paths  
✅ Creates backups before production deployments  
✅ Non-blocking health checks with warnings
