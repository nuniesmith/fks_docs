# FKS Trading Systems - GitHub Secrets Configuration

## Complete Secrets List for Automated Deployment

This document outlines all GitHub secrets required for the complete FKS deployment pipeline including server creation, SSL automation with Cloudflare, and service deployment.

## 🔐 Current Required Secrets

### Linode Server Management
1. **`LINODE_CLI_TOKEN`**
   - Description: Linode API token for server creation and management
   - Source: Linode Cloud Manager > API Tokens
   - Permissions: Read/Write for Linodes, Volumes, Images
   - Used for: Creating/detecting servers, managing infrastructure

2. **`FKS_DEV_ROOT_PASSWORD`**
   - Description: Root password for server access
   - Requirements: Strong password (12+ chars, mixed case, numbers, symbols)
   - Used for: Initial SSH access, server configuration

### User Account Passwords
3. **`JORDAN_PASSWORD`**
   - Description: Password for jordan user account
   - Used for: Creating jordan user during server setup

4. **`FKS_USER_PASSWORD`**
   - Description: Password for fks_user account
   - Used for: Creating main application user account

### Networking & VPN
5. **`TAILSCALE_AUTH_KEY`**
   - Description: Tailscale authentication key for VPN mesh
   - Source: Tailscale Admin Console > Settings > Keys
   - Type: Reusable auth key (recommended)
   - Used for: Connecting server to Tailscale network

### Monitoring
6. **`NETDATA_CLAIM_TOKEN`**
   - Description: Netdata Cloud claim token
   - Source: Netdata Cloud > Spaces > Connect Nodes
   - Used for: Connecting server monitoring to Netdata Cloud

7. **`NETDATA_CLAIM_ROOM`**
   - Description: Netdata room/space ID
   - Source: Netdata Cloud > Space URL or API
   - Used for: Organizing monitored nodes

### Container Registry
8. **`DOCKER_USERNAME`**
   - Description: Docker Hub username
   - Used for: Pushing/pulling Docker images

9. **`DOCKER_TOKEN`**
   - Description: Docker Hub access token
   - Source: Docker Hub > Account Settings > Security > Access Tokens
   - Used for: Authenticating with Docker Hub

### Notifications
10. **`DISCORD_WEBHOOK_SERVERS`**
    - Description: Discord webhook URL for server notifications
    - Source: Discord Server > Channel Settings > Integrations > Webhooks
    - Used for: Sending deployment status notifications

## 🌐 NEW: Cloudflare & SSL Automation Secrets

### Cloudflare API Access
11. **`CLOUDFLARE_API_TOKEN`** ⭐ NEW
    - Description: Cloudflare API token with Zone:Edit permissions
    - Source: Cloudflare Dashboard > My Profile > API Tokens
    - Permissions Required:
      - Zone:Zone:Read
      - Zone:DNS:Edit
      - Zone:Zone Settings:Edit
    - Used for: DNS management, SSL certificate validation

12. **`CLOUDFLARE_ZONE_ID`** ⭐ NEW
    - Description: Zone ID for fkstrading.xyz domain
    - Source: Cloudflare Dashboard > Domain > Overview (right sidebar)
    - Format: 32-character hex string
    - Used for: Targeting the correct DNS zone

### SSL & Domain Configuration
13. **`DOMAIN_NAME`** ⭐ NEW
    - Description: Primary domain name
    - Value: `fkstrading.xyz`
    - Used for: SSL certificate generation, nginx configuration

14. **`ADMIN_EMAIL`** ⭐ NEW
    - Description: Email for Let's Encrypt certificate registration
    - Requirements: Valid email address
    - Used for: Let's Encrypt account registration and notifications

### Optional: Advanced SSL Configuration
15. **`CLOUDFLARE_EMAIL`** ⭐ NEW (Optional)
    - Description: Cloudflare account email (if using Global API Key instead of token)
    - Only needed if using legacy authentication method
    - Recommended: Use CLOUDFLARE_API_TOKEN instead

16. **`SSL_STAGING`** ⭐ NEW (Optional)
    - Description: Use Let's Encrypt staging environment for testing
    - Value: `true` or `false` (default: `false`)
    - Used for: Testing SSL setup without rate limits

## 📋 How to Set Up New Cloudflare Secrets

### Step 1: Create Cloudflare API Token

1. **Login to Cloudflare Dashboard**
   - Go to [Cloudflare Dashboard](https://dash.cloudflare.com/)
   - Login to your account

2. **Create API Token**
   - Go to **My Profile** (top right) > **API Tokens**
   - Click **Create Token**
   - Use **Custom token** template
   - Configure permissions:
     ```
     Zone - Zone:Read - Include - All zones
     Zone - DNS:Edit - Include - Specific zone - fkstrading.xyz
     Zone - Zone Settings:Edit - Include - Specific zone - fkstrading.xyz
     ```
   - Add IP restriction if desired (optional but recommended)
   - Click **Continue to summary** > **Create Token**
   - **COPY THE TOKEN IMMEDIATELY** (it won't be shown again)

3. **Get Zone ID**
   - Go to **Cloudflare Dashboard**
   - Select your **fkstrading.xyz** domain
   - In the right sidebar under **API**, copy the **Zone ID**

### Step 2: Add Secrets to GitHub

Navigate to your GitHub repository:
**Settings > Secrets and variables > Actions > New repository secret**

Add these new secrets:

```
Name: CLOUDFLARE_API_TOKEN
Value: [Your API token from Step 1]

Name: CLOUDFLARE_ZONE_ID  
Value: [Your Zone ID from Step 1]

Name: DOMAIN_NAME
Value: fkstrading.xyz

Name: ADMIN_EMAIL
Value: [Your email address]
```

## 🔧 Workflow Integration

The new secrets will be integrated into the workflow for:

1. **DNS Management**: Automatic A record creation/updates
2. **SSL Certificate Generation**: Let's Encrypt with Cloudflare DNS challenge
3. **Nginx Configuration**: Automatic SSL setup with domain
4. **Certificate Renewal**: Automated renewal via cron job

## 🚀 Enhanced Deployment Flow

With these secrets, the deployment will include:

1. **Server Creation** → SSH Keys → Stage 1 Setup
2. **Stage 2 Completion** → Tailscale Authentication  
3. **🆕 DNS Configuration** → Create/update A record for server IP
4. **🆕 SSL Certificate Generation** → Let's Encrypt with Cloudflare validation
5. **🆕 Nginx SSL Setup** → Configure HTTPS with automatic redirects
6. **Application Deployment** → Deploy services with SSL
7. **🆕 Certificate Auto-Renewal** → Setup cron job for renewal

## ⚡ Benefits of SSL Automation

- ✅ **Automatic HTTPS**: fkstrading.xyz will have SSL immediately
- ✅ **DNS Management**: A records updated automatically
- ✅ **Certificate Renewal**: Auto-renewal every 60 days
- ✅ **Security**: Encrypted traffic, proper domain setup
- ✅ **Professional**: Production-ready domain configuration

## 🔒 Security Best Practices

1. **API Token Scope**: Use minimal required permissions
2. **Token Rotation**: Rotate API tokens every 90 days
3. **IP Restrictions**: Limit token usage to GitHub Actions IPs if possible
4. **Monitoring**: Monitor Cloudflare audit logs for API usage
5. **Backup**: Keep multiple API tokens in case one is compromised

## 📝 Next Steps

1. **Create Cloudflare API token** with proper permissions
2. **Add all new secrets** to GitHub repository
3. **Update workflow** to include SSL automation steps
4. **Test deployment** with new domain configuration
5. **Verify SSL setup** and certificate auto-renewal

This will enable fully automated deployment with production-ready SSL certificates for fkstrading.xyz!
