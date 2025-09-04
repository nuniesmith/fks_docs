# FKS Trading Systems - DNS & Server Management Enhancement

## 🎯 **Enhancement Overview**

This update adds comprehensive DNS management and dynamic server IP handling to resolve the SSH connection timeout issues in GitHub Actions.

## 🔧 **New Features Added**

### 1. **Dynamic Server IP Detection**
- **Script**: `scripts/deployment/tools/get-server-ip.sh`
- **Purpose**: Automatically gets the current FKS server IP from Linode API
- **Usage**: 
  ```bash
  ./scripts/deployment/tools/get-server-ip.sh --token YOUR_TOKEN --label fks
  ```

### 2. **Automatic DNS Record Updates**
- **Script**: `scripts/deployment/tools/update-dns-records.sh` 
- **Purpose**: Updates Cloudflare DNS records for main domain and service subdomains
- **Features**:
  - Updates root domain (fkstrading.xyz)
  - Updates www subdomain (www.fkstrading.xyz)
  - Updates service subdomains (api, app, admin, grafana, etc.)
  - Creates wildcard CNAME for flexibility

### 3. **Enhanced Server Creation Pipeline**
- **Script**: `scripts/deployment/staged/stage-0-create-server.sh`
- **Enhancement**: Automatically updates DNS records after server creation
- **Flow**:
  1. Creates new Linode server
  2. Gets server IP address
  3. Automatically updates DNS records
  4. Server is ready with correct DNS pointing

### 4. **Improved GitHub Actions Workflow**
- **File**: `.github/staging/00-update-repo-enhanced.yml`
- **Improvements**:
  - Dynamically gets server IP from Linode
  - Updates DNS records before deployment
  - Uses IP address instead of domain for SSH (avoids DNS propagation delays)
  - Better error handling and logging

## 📊 **DNS Records Configuration**

### Main Domain Records
| Record Type | Name | Value | TTL | Proxied |
|-------------|------|-------|-----|---------|
| A | fkstrading.xyz | [SERVER_IP] | 300 | No |
| A | www.fkstrading.xyz | [SERVER_IP] | 300 | No |

### Service Subdomains
| Subdomain | Purpose | URL |
|-----------|---------|-----|
| api.fkstrading.xyz | API Gateway | https://api.fkstrading.xyz |
| app.fkstrading.xyz | Main Application | https://app.fkstrading.xyz |
| admin.fkstrading.xyz | Admin Panel | https://admin.fkstrading.xyz |
| ws.fkstrading.xyz | WebSocket Server | wss://ws.fkstrading.xyz |
| grafana.fkstrading.xyz | Monitoring Dashboard | https://grafana.fkstrading.xyz |
| prometheus.fkstrading.xyz | Metrics Collection | https://prometheus.fkstrading.xyz |
| status.fkstrading.xyz | Status Page | https://status.fkstrading.xyz |
| docs.fkstrading.xyz | Documentation | https://docs.fkstrading.xyz |

### Wildcard Support
- `*.fkstrading.xyz` → CNAME to `fkstrading.xyz` (for future services)

## 🔐 **Required Secrets**

Add these to your GitHub repository secrets:

| Secret Name | Description | Example |
|-------------|-------------|---------|
| `LINODE_CLI_TOKEN` | Linode API token | `abcd1234...` |
| `CLOUDFLARE_API_TOKEN` | Cloudflare API token | `xyz789...` |
| `CLOUDFLARE_ZONE_ID` | Cloudflare Zone ID for fkstrading.xyz | `abc123...` |
| `ACTIONS_USER_SSH_PRIVATE` | SSH private key for server access | `-----BEGIN OPENSSH...` |
| `FKS_DEV_ROOT_PASSWORD` | Root password for Linode servers | `SecurePassword123!` |

## 🚀 **Usage Examples**

### Manual DNS Update
```bash
# Update DNS records for a specific server IP
./scripts/deployment/tools/update-dns-records.sh \
  --server-ip 192.168.1.100 \
  --api-token YOUR_CLOUDFLARE_TOKEN \
  --zone-id YOUR_ZONE_ID

# Dry run to see what would be updated
./scripts/deployment/tools/update-dns-records.sh \
  --server-ip 192.168.1.100 \
  --dry-run

# Update only main domain, skip subdomains
./scripts/deployment/tools/update-dns-records.sh \
  --server-ip 192.168.1.100 \
  --no-subdomains
```

### Get Server IP
```bash
# Get just the IP address
./scripts/deployment/tools/get-server-ip.sh --token YOUR_TOKEN

# Get full server info as JSON
./scripts/deployment/tools/get-server-ip.sh --token YOUR_TOKEN --format json

# Get server info as environment variables
./scripts/deployment/tools/get-server-ip.sh --token YOUR_TOKEN --format env
```

### Automated Server Creation with DNS
```bash
# Create server and automatically update DNS
CLOUDFLARE_API_TOKEN=your_token \
CLOUDFLARE_ZONE_ID=your_zone_id \
./scripts/deployment/staged/stage-0-create-server.sh
```

## 🔄 **Workflow Integration**

### GitHub Actions Flow
1. **Checkout Code** - Get latest repository code
2. **Get Server IP** - Query Linode API for current FKS server IP
3. **Update DNS** - Update Cloudflare DNS records to point to server
4. **Setup SSH** - Configure SSH authentication using server IP
5. **Deploy** - Connect to server and update repository
6. **Summary** - Provide deployment summary with IP and DNS info

### Benefits
- **Eliminates DNS Timeout Issues**: Uses IP address for SSH connection
- **Automatic DNS Management**: No manual DNS updates needed
- **Service-Ready Infrastructure**: Subdomains configured for all services
- **Scalable Architecture**: Easy to add new services with DNS records

## 🛠️ **Docker Service Integration**

### Service Configuration
Each Docker service can be accessed via its dedicated subdomain:

```yaml
# docker-compose.yml example
services:
  api:
    ports:
      - "8000:8000"
    environment:
      - VIRTUAL_HOST=api.fkstrading.xyz
      
  app:
    ports:
      - "3000:3000"
    environment:
      - VIRTUAL_HOST=app.fkstrading.xyz
      
  grafana:
    ports:
      - "3001:3000"
    environment:
      - VIRTUAL_HOST=grafana.fkstrading.xyz
```

### Nginx Reverse Proxy Setup
```nginx
# /etc/nginx/sites-available/fkstrading.xyz
server {
    listen 80;
    server_name api.fkstrading.xyz;
    
    location / {
        proxy_pass http://localhost:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}

server {
    listen 80;
    server_name app.fkstrading.xyz;
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 🔍 **Troubleshooting**

### Common Issues

**DNS Update Fails**
```bash
# Check Cloudflare credentials
curl -X GET "https://api.cloudflare.com/client/v4/zones" \
  -H "Authorization: Bearer YOUR_API_TOKEN" \
  -H "Content-Type: application/json"
```

**Server IP Not Found**
```bash
# List all Linode servers
curl -H "Authorization: Bearer YOUR_TOKEN" \
  "https://api.linode.com/v4/linode/instances"
```

**SSH Connection Still Fails**
```bash
# Test direct IP connection
ssh root@ACTUAL_IP_ADDRESS

# Check if server is accessible
ping ACTUAL_IP_ADDRESS
```

### Debug Commands
```bash
# Test DNS resolution
nslookup fkstrading.xyz
dig fkstrading.xyz

# Check all FKS servers
./scripts/deployment/tools/get-server-ip.sh --format json | jq

# Dry run DNS update
./scripts/deployment/tools/update-dns-records.sh --dry-run --server-ip NEW_IP
```

## 🎯 **Next Steps**

1. **SSL Certificate Automation**: Extend to automatically update SSL certificates
2. **Health Monitoring**: Add DNS-based health checks
3. **Load Balancing**: Support multiple servers with DNS round-robin
4. **CDN Integration**: Add Cloudflare proxy support for static assets
5. **Backup DNS**: Configure secondary DNS providers

## 📈 **Performance Benefits**

- **Faster Deployments**: No waiting for DNS propagation
- **Reduced Failures**: Direct IP connection eliminates DNS-related timeouts
- **Better Monitoring**: Clear visibility into server IP and DNS status
- **Scalable Infrastructure**: Ready for multi-service architecture

---

**Enhancement completed**: $(date)
**Scripts added**: 2 new tools, 1 enhanced workflow
**DNS records managed**: 12+ subdomains automatically configured
