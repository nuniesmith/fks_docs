# Updated Two-Stage Deployment Guide

## Overview

The StackScript has been restructured to follow a clean two-stage deployment pattern as recommended:

### Stage 1: System Preparation (Pre-Reboot)
- Update packages and install software
- Run systemctl enable and start services
- Change hostname to "ninja"
- Create ninja user with proper permissions
- Enable but don't fully configure complex services
- **Then reboot**

### Stage 2: Service Configuration (Post-Reboot)
- Verify all services are running
- Setup Tailscale with `tailscale up --authkey --accept-routes`
- Clone repository using GitHub credentials
- Start Docker environment
- Configure monitoring and final setup
- **Ready for GitHub Actions CI/CD**

## Key Improvements

### 1. Clean Two-Stage Separation
```bash
# Stage 1: Basic system setup
stage_1_system_setup() {
    update_system_packages
    install_dotnet
    create_ninja_user
    setup_ssh_stage1      # Enable service only
    setup_docker_stage1   # Install and enable only
    set_hostname          # Set to "ninja"
    setup_firewall
    # REBOOT
}

# Stage 2: Advanced configuration
stage_2_post_reboot() {
    verify_services_post_reboot
    setup_tailscale_stage2   # With --accept-routes
    clone_repository         # To /home/ninja/ninja
    build_and_deploy_docker
    # Ready for GitHub Actions
}
```

### 2. Proper Hostname Configuration
- Hostname is set to "ninja" as requested
- Updated in `/etc/hostname` and `/etc/hosts`
- Applied during stage 1 before reboot

### 3. Tailscale Best Practices
```bash
# Recommended Tailscale setup
tailscale up --authkey="$TAILSCALE_AUTH_KEY" --accept-routes --timeout=30s
```
- Uses `--accept-routes` for subnet access
- Proper timeout handling
- Manual fallback instructions if auth key not provided

### 4. Repository Location
- Repository is cloned to `/home/ninja/ninja`
- Proper ownership: `ninja:ninja`
- Git configured with access token for future updates
- Ready for GitHub Actions workflow

### 5. GitHub Actions Readiness
- Git credentials stored securely
- Repository in standard location
- Docker environment fully operational
- Service management scripts available
- Configuration saved with `ready_for_github_actions: true`

## Stage 1 Operations

### System Updates
- OS-specific package updates
- Essential package installation
- .NET SDK installation
- Docker installation (enabled, not started)

### User Management
- Create `ninja` user with strong password
- Add to appropriate groups (`wheel` for Arch, `sudo` for Debian/Ubuntu)
- Configure sudo access

### Service Enablement
- Enable SSH service (don't configure user keys yet)
- Enable Docker service (don't start containers yet)
- Enable firewall with basic rules

### System Configuration
- Set hostname to "ninja"
- Configure basic security settings
- Save stage 1 configuration

## Stage 2 Operations

### Service Verification
- Verify SSH is running
- Start and verify Docker
- Install Docker Compose if needed
- Configure SSH keys for ninja user

### Tailscale Setup
- Install Tailscale
- Connect with provided auth key
- Use `--accept-routes` for subnet access
- Show connection status

### Repository Deployment
- Clone to `/home/ninja/ninja`
- Set proper ownership
- Configure Git with access token
- Make scripts executable

### Docker Environment
- Build all containers
- Start services with `docker-compose up -d`
- Verify all services are running
- Configure monitoring

### Final Configuration
- Create systemd service for management
- Set up monitoring and health checks
- Save final configuration
- Display comprehensive summary

## Usage Instructions

### For Linode StackScript
1. Create new Linode instance
2. Select the StackScript
3. Provide required UDF fields:
   - GitHub repository
   - GitHub token
   - SSH public key
   - Tailscale auth key (optional)
4. Deploy and wait for stage 1 completion
5. System will automatically reboot and run stage 2
6. Total deployment time: 10-15 minutes

### For GitHub Actions Integration
After deployment is complete:

1. **Repository is ready**: Located at `/home/ninja/ninja`
2. **Git is configured**: Uses provided GitHub token
3. **Docker environment is operational**: `docker-compose up -d`
4. **Services are managed**: systemd service `ninja-trading`

Example GitHub Actions workflow:
```yaml
name: Deploy to Ninja Server
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - name: Deploy to server
        uses: appleboy/ssh-action@v0.1.5
        with:
          host: ${{ secrets.HOST }}
          username: ninja
          key: ${{ secrets.SSH_KEY }}
          script: |
            cd /home/ninja/ninja
            git pull origin main
            docker-compose down
            docker-compose up -d --build
```

## Configuration Files

### Stage 1 Configuration
- Location: `/etc/ninja-trading/stage1-config.json`
- Contains: System setup status, user creation, service enablement

### Final Configuration
- Location: `/etc/ninja-trading/deployment-config.json`
- Contains: Complete deployment status, service URLs, paths, GitHub Actions readiness

### Environment Configuration
- Location: `/home/ninja/ninja/.env`
- Contains: Application-specific environment variables

## Troubleshooting

### If Stage 2 Doesn't Auto-Run
```bash
# Check if stage 2 marker exists
ls -la /etc/ninja-trading/stage2-required

# Check systemd service
systemctl status ninja-stage2.service

# Manual stage 2 run
sudo /usr/local/bin/ninja-stage2.sh
```

### Docker Issues
```bash
# Check Docker status
systemctl status docker

# Check for Arch Linux specific issues
journalctl -u docker.service -n 20

# Fallback to VFS storage if needed
# (Script handles this automatically)
```

### Tailscale Issues
```bash
# Check service status
systemctl status tailscaled

# Manual connection
sudo tailscale up --accept-routes

# Check IP assignment
tailscale ip -4
```

## Benefits of Two-Stage Approach

1. **Reliability**: Clean separation of system vs application setup
2. **Debugging**: Easy to identify which stage failed
3. **Consistency**: Reboot ensures all services start fresh
4. **CI/CD Ready**: Perfect foundation for GitHub Actions
5. **Maintainability**: Clear structure for future updates

The updated StackScript now provides a robust, reliable deployment that's ready for production use and GitHub Actions integration.
