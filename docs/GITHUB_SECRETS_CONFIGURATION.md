# FKS Trading Systems - GitHub Secrets Configuration

## 🔐 Required Secrets for Deployment

This document outlines how each of your configured GitHub secrets is used in the automated deployment pipeline.

## 🏗️ Infrastructure Secrets

### Linode Configuration
| Secret | Usage | Required |
|--------|-------|----------|
| `LINODE_CLI_TOKEN` | Linode API access for server creation and management | ✅ Yes |
| `FKS_DEV_ROOT_PASSWORD` | Root password for newly created Linode servers | ✅ Yes |

### User Management
| Secret | Usage | Required |
|--------|-------|----------|
| `JORDAN_PASSWORD` | Password for 'jordan' user account on server | ✅ Yes |
| `FKS_USER_PASSWORD` | Password for 'fks_user' service account | ✅ Yes |

## 🐳 Container Registry

### Docker Hub
| Secret | Usage | Required |
|--------|-------|----------|
| `DOCKER_USERNAME` | Docker Hub username for image push/pull | ✅ Yes |
| `DOCKER_TOKEN` | Docker Hub access token for authentication | ✅ Yes |

## 🔐 SSH Access Keys

All SSH public keys are automatically configured on the server for secure access:

| Secret | Purpose | Auto-Configured |
|--------|---------|-----------------|
| `ACTIONS_USER_SSH_PUB` | GitHub Actions runner SSH access | ✅ Yes |
| `DESKTOP_SSH_PUB` | Desktop computer SSH access | ✅ Yes |
| `FREDDY_SSH_PUB` | Freddy's computer SSH access | ✅ Yes |
| `MACBOOK_SSH_PUB` | MacBook SSH access | ✅ Yes |
| `ORYX_SSH_PUB` | Oryx computer SSH access | ✅ Yes |
| `SULLIVAN_SSH_PUB` | Sullivan's computer SSH access | ✅ Yes |

## 🌐 Network & Security

### VPN Access
| Secret | Usage | Required |
|--------|-------|----------|
| `TAILSCALE_AUTH_KEY` | Automatic VPN setup for secure server access | ✅ Yes |

### Domain Management
| Secret | Usage | Required |
|--------|-------|----------|
| `DOMAIN_NAME` | Domain configuration for services | 🔶 Optional |
| `CLOUDFLARE_API_TOKEN` | DNS management (future use) | 🔶 Optional |
| `CLOUDFLARE_ZONE_ID` | DNS zone management (future use) | 🔶 Optional |
| `SSL_STAGING` | SSL certificate configuration | 🔶 Optional |

## 📊 Monitoring & Alerts

### System Monitoring
| Secret | Usage | Required |
|--------|-------|----------|
| `NETDATA_CLAIM_TOKEN` | Netdata Cloud monitoring setup | 🔶 Optional |
| `NETDATA_CLAIM_ROOM` | Netdata Cloud room assignment | 🔶 Optional |

### Notifications
| Secret | Usage | Required |
|--------|-------|----------|
| `DISCORD_WEBHOOK_SERVERS` | Server alerts and notifications | 🔶 Optional |

## ⚖️ Application Configuration

### Administration
| Secret | Usage | Required |
|--------|-------|----------|
| `ADMIN_EMAIL` | System administrator email | 🔶 Optional |
| `JWT_SECRET_KEY` | JSON Web Token signing key | 🔶 Optional |

### Trading Platform
| Secret | Usage | Required |
|--------|-------|----------|
| `RITHMIC_USERNAME` | Rithmic trading platform username | 🔶 Optional |
| `RITHMIC_PASSWORD` | Rithmic trading platform password | 🔶 Optional |

## 🚀 Deployment Workflow Usage

### Server Creation Phase
```yaml
# Uses these secrets to create fks-dev server in Toronto:
- LINODE_CLI_TOKEN          # API access
- FKS_DEV_ROOT_PASSWORD      # Server root password  
- JORDAN_PASSWORD           # User setup
- FKS_USER_PASSWORD         # Service user setup
- TAILSCALE_AUTH_KEY        # VPN setup
- All SSH public keys       # SSH access setup
- NETDATA_CLAIM_TOKEN       # Monitoring setup
- NETDATA_CLAIM_ROOM        # Monitoring room
```

### Application Deployment Phase
```yaml
# Uses these secrets for application deployment:
- JORDAN_PASSWORD           # SSH authentication
- DOCKER_USERNAME           # Container registry
- DOCKER_TOKEN              # Container authentication  
- JWT_SECRET_KEY            # Application security
- RITHMIC_USERNAME          # Trading platform
- RITHMIC_PASSWORD          # Trading platform
- DOMAIN_NAME               # Service configuration
- ADMIN_EMAIL               # Admin configuration
```

### Environment Configuration
The deployment automatically creates a `.env` file on the server with:
```bash
DOCKER_USERNAME=${DOCKER_USERNAME}
APP_ENV=development
DOMAIN_NAME=${DOMAIN_NAME}
ADMIN_EMAIL=${ADMIN_EMAIL}
JWT_SECRET_KEY=${JWT_SECRET_KEY}
RITHMIC_USERNAME=${RITHMIC_USERNAME}
RITHMIC_PASSWORD=${RITHMIC_PASSWORD}
TZ=America/Toronto
```

## 🔍 Secret Validation

The workflow automatically validates that all required secrets are configured:

### ✅ Required Secrets Check
- Linode access (API token, root password)
- User passwords (jordan, fks_user)  
- Docker registry (username, token)
- Tailscale VPN (auth key)

### 📋 Optional Secrets Report
- Reports which optional secrets are configured
- Shows what features will be available based on configured secrets

## 🛡️ Security Best Practices

### Secret Management
- ✅ All secrets stored in GitHub encrypted secrets
- ✅ Secrets never logged or exposed in workflow output
- ✅ SSH key-based authentication preferred where possible
- ✅ VPN (Tailscale) for secure server access

### Access Control
- ✅ Multiple SSH keys configured for team access
- ✅ Separate service account (fks_user) for application
- ✅ Password authentication as fallback
- ✅ Automatic firewall configuration

## 🔄 Updating Secrets

To update any secret:

1. Go to GitHub repository → Settings → Secrets and variables → Actions
2. Find the secret name from the table above
3. Click "Update" and enter the new value
4. The next deployment will automatically use the updated value

## 📞 Support

If you need to:
- Add new secrets → Update this documentation and the workflow
- Remove secrets → Ensure they're not required in the workflow first
- Rotate secrets → Update in GitHub settings, then redeploy

---

**Note:** This configuration provides a secure, automated deployment pipeline with comprehensive secret management for the FKS Trading Systems.
