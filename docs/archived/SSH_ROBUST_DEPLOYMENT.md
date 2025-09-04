# Robust SSH Deployment System

## Overview

The GitHub Actions deployment workflow now includes a comprehensive SSH connection system with multiple fallback mechanisms to ensure reliable deployment even when initial SSH attempts fail.

## Connection Flow

### 1. Primary Connection (Tailscale DNS)

- Attempts connection via Tailscale DNS hostname
- Uses jordan user with SSH key authentication
- 3 retry attempts with 15-second delays

### 2. Fallback Connection (Public IP)

- Falls back to public IP if Tailscale fails
- Uses jordan user with SSH key authentication  
- 2 retry attempts with 20-second delays

### 3. Root Access Recovery

- If jordan user SSH fails completely, attempts root login
- Uses `FKS_DEV_ROOT_PASSWORD` for authentication
- Automatically configures jordan user SSH access
- Sets up authorized_keys and SSHD configuration

### 4. Manual Key Addition

- Final fallback extracts public key from private key
- Manually adds to jordan's authorized_keys via root
- Includes multiple verification attempts

## Required GitHub Secrets

| Secret | Purpose | Required |
|--------|---------|----------|
| `LINODE_CLI_TOKEN` | Linode API access | ✅ Always |
| `ACTIONS_ROOT_PRIVATE_KEY` | SSH authentication | ✅ Always |
| `FKS_DEV_ROOT_PASSWORD` | Root access fallback | ✅ Recommended |
| `JORDAN_PASSWORD` | Jordan user password | ✅ Recommended |

## Automated SSH Setup Features

### Key Setup and Validation

- Validates SSH private key format (OpenSSH vs RFC4716)
- Automatically converts base64-encoded keys
- Extracts public key for authorized_keys setup
- Sets proper file permissions (600 for keys, 700 for .ssh)

### Server Configuration

- Creates jordan user if it doesn't exist
- Adds jordan to sudo group
- Configures SSHD for key authentication
- Enables both key and password authentication
- Reloads SSH service after configuration

### Error Handling and Diagnostics

- Detailed connection attempt logging
- Clear error messages with troubleshooting steps
- Fallback connection information
- Manual setup instructions if all automation fails

## Troubleshooting

### Common Issues

1. **New server not ready**
   - Wait 10-15 minutes for full initialization
   - Check Linode console for server status

2. **SSH key issues**
   - Verify ACTIONS_ROOT_PRIVATE_KEY secret is correctly set
   - Ensure key format is proper OpenSSH
   - Check that key wasn't corrupted during copy/paste

3. **Root access required**
   - Ensure FKS_DEV_ROOT_PASSWORD is set
   - Verify password works via Linode console

### Manual Recovery

If all automated attempts fail:

```bash
# Connect as root
ssh root@<server-public-ip>

# Set up jordan user
useradd -m -s /bin/bash jordan
echo 'jordan:your-password' | chpasswd
usermod -aG sudo jordan

# Set up SSH
mkdir -p /home/jordan/.ssh
chmod 700 /home/jordan/.ssh
echo 'your-public-key' > /home/jordan/.ssh/authorized_keys
chmod 600 /home/jordan/.ssh/authorized_keys
chown -R jordan:jordan /home/jordan/.ssh

# Configure SSHD
sed -i 's/#PubkeyAuthentication yes/PubkeyAuthentication yes/' /etc/ssh/sshd_config
systemctl reload sshd
```

## Security Features

- Uses secure key-based authentication by default
- Enables password authentication as fallback
- Maintains proper file permissions
- Uses non-root user for deployments
- Includes timeout and connection limits

## Future Enhancements

Consider implementing:

- Dedicated `github_actions` user instead of `jordan`
- Key rotation mechanism
- Connection health monitoring
- Automated server hardening after setup

## Testing the Setup

To verify the robust SSH system:

1. Run the deployment workflow
2. Monitor the "Test SSH connection and setup user access" step
3. Check that it progresses through fallback mechanisms if needed
4. Verify successful deployment completion

The system is designed to be self-healing and should handle most common SSH connection scenarios automatically.
