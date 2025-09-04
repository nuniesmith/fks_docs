# SSH User Separation Strategy

## Overview
The FKS deployment system now uses a robust 4-user SSH setup with proper fallback mechanisms for maximum deployment reliability.

## User Roles

### 1. `root` User
- **Purpose**: Emergency access and deployment fallback
- **Access**: Password authentication with `FKS_DEV_ROOT_PASSWORD`
- **Used for**: Initial server setup, emergency recovery
- **SSH Key**: `ACTIONS_USER_SSH_PUB` (same as all users)

### 2. `jordan` User  
- **Purpose**: System administration and manual operations
- **Access**: SSH key authentication (primary)
- **Permissions**: `sudo` access via `wheel` group
- **SSH Key**: `ACTIONS_USER_SSH_PUB`

### 3. `actions_user` User
- **Purpose**: GitHub Actions connection and repository management
- **Access**: SSH key authentication (primary method for GitHub Actions)
- **Permissions**: `sudo` access via `wheel` group, `docker` group
- **SSH Key**: `ACTIONS_USER_SSH_PUB` (for server access)
- **Repository Key**: Generated Ed25519 key for GitHub repository access

### 4. `fks_user` User
- **Purpose**: Service account for running Docker containers
- **Access**: SSH key authentication (limited use)
- **Permissions**: `docker` group only
- **SSH Key**: `ACTIONS_USER_SSH_PUB`

## SSH Connection Strategy

### GitHub Actions Connection Flow
1. **Primary**: `actions_user@public_ip` (with SSH key)
2. **Fallback**: `root@public_ip` (with password)
3. **Warning**: If actions_user fails, logs warning and continues with root

### Initial Server Setup
- **Always**: `root@public_ip` with `FKS_DEV_ROOT_PASSWORD`
- **Reason**: Ensures consistent initial access regardless of SSH key state

## SSH Key Management

### Server Access Key
- **Secret**: `ACTIONS_USER_SSH_PUB` 
- **Purpose**: GitHub Actions → Server connection
- **Users**: All 4 users get this key in their `authorized_keys`

### Repository Access Key
- **Generated**: On server during Stage 1 setup
- **Purpose**: Server → GitHub repository access
- **Location**: `/home/actions_user/.ssh/id_ed25519`
- **Setup**: Must be added as deploy key to GitHub repository

## Key Benefits

1. **Deployment Reliability**: Multiple fallback mechanisms ensure deployment always works
2. **Security**: Proper user separation with minimal privileges
3. **Simplicity**: Single SSH key for all server access
4. **Flexibility**: Different users for different purposes
5. **Recovery**: Root access always available for emergency situations

## Security Configuration

### SSH Daemon Settings
```
PermitRootLogin yes          # For deployment reliability
PasswordAuthentication yes   # For fallback access
PubkeyAuthentication yes     # Primary authentication method
AllowUsers root jordan actions_user fks_user
```

### User Permissions
- `root`: Full system access
- `jordan`: Sudo access (wheel group)
- `actions_user`: Sudo + docker access
- `fks_user`: Docker access only

## Workflow Integration

### Stage 1: Initial Setup
- Connects as `root@public_ip` with password
- Creates all 4 users with proper SSH keys
- Configures SSH daemon for security + deployment needs
- Generates repository access key for `actions_user`

### Stage 2: Post-Reboot Connection
- Tries `actions_user@public_ip` with SSH key (preferred)
- Falls back to `root@public_ip` with password
- Logs warnings if fallback is used

### Deployment Phase
- Uses `actions_user@public_ip` for repository operations
- Can escalate to `sudo` for system operations
- Switches to `fks_user` for Docker container operations

## Troubleshooting

### If actions_user SSH fails:
1. Check if `ACTIONS_USER_SSH_PUB` is correctly set in secrets
2. Verify SSH key was deployed during Stage 1
3. Workflow will automatically fallback to root access

### If root fallback fails:
1. Check `FKS_DEV_ROOT_PASSWORD` secret
2. Verify server is responsive and SSH service is running
3. Check firewall/networking configuration

### Repository access issues:
1. Ensure deploy key is added to GitHub repository
2. Check `/home/actions_user/.ssh/id_ed25519` exists on server
3. Verify key permissions and ownership

## Future Enhancements

1. **Post-deployment hardening**: Disable root password login after successful deployment
2. **Key rotation**: Automated SSH key rotation for enhanced security
3. **Audit logging**: Enhanced logging of SSH access and user activities
4. **Multi-key support**: Support for different SSH keys per user if needed
