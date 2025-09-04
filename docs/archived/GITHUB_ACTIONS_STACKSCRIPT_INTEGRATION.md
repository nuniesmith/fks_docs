# GitHub Actions StackScript Integration

## Overview

The GitHub Actions workflow `deploy-dev.yml` has been updated to use the external StackScript located at `scripts/deployment/linode/linode-stackscript.sh` instead of an embedded inline script.

## Key Changes Made

### 1. StackScript Integration
- Updated the workflow to read the StackScript from the repository file instead of embedding it inline
- Changed StackScript name to `fks-trading-system-v3` to differentiate from the old embedded version
- Added support for Ubuntu 24.04 LTS as the default image (previously Ubuntu 22.04)

### 2. SSH Key Support
- Added SSH key UDF parameters to the external StackScript:
  - `ACTIONS_USER_SSH_PUB` - GitHub Actions SSH Public Key
  - `ACTIONS_JORDAN_SSH_PUB` - Jordan SSH Public Key
- Implemented `setup_ssh_for_user()` function in the StackScript to properly configure SSH keys
- Added fallback logic to handle both uppercase and lowercase UDF variable names (Linode converts to uppercase)

### 3. User Management Updates
- Updated all references from `github_actions` (underscore) to `actions_user` (hyphen) to match the StackScript
- This affects:
  - SSH configuration
  - User directory paths (`/home/actions_user/fks`)
  - Connection testing logic
  - Deployment scripts
  - Summary outputs

### 4. Extended Setup Time
- Increased StackScript completion wait time from 10 minutes to 15 minutes
- This accounts for the two-phase setup process in the external StackScript

### 5. Image and Distribution Support
- Default image changed to `linode/ubuntu24.04`
- StackScript supports both Ubuntu 24.04 LTS and Arch Linux
- Multi-distro capability for future flexibility

## UDF Variables Required

The StackScript expects these User Defined Fields (UDF):

| Variable | Description | Required |
|----------|-------------|----------|
| `jordan_password` | Password for jordan user | Yes |
| `fks_user_password` | Password for fks_user | Yes |
| `ACTIONS_USER_SSH_PUB` | GitHub Actions SSH Public Key | No |
| `ACTIONS_JORDAN_SSH_PUB` | Jordan SSH Public Key | No |
| `tailscale_auth_key` | Tailscale Auth Key | Yes |
| `docker_username` | Docker Hub Username | No |
| `docker_token` | Docker Hub Access Token | No |
| `netdata_claim_token` | Netdata Cloud Claim Token | No |
| `netdata_claim_room` | Netdata Cloud Room ID | No |
| `timezone` | Server Timezone | No (default: America/Toronto) |

## Benefits of External StackScript

1. **Maintainability**: Single source of truth for server setup logic
2. **Reusability**: StackScript can be used outside of GitHub Actions
3. **Testing**: Easier to test and debug server setup independently
4. **Version Control**: StackScript changes are tracked in git
5. **Multi-distro Support**: Supports multiple Linux distributions
6. **Two-phase Setup**: Handles complex setup scenarios with reboots

## User Account Structure

After StackScript completion, the following users are created:

- **`jordan`** - Admin user with sudo access and password authentication
- **`actions_user`** - CI/CD service account with SSH key authentication only
- **`fks_user`** - Application user with password authentication

## SSH Key Deployment

- GitHub Actions SSH key is deployed to both `actions_user` and `jordan` users
- `jordan` user has the GitHub Actions key as a fallback for deployment
- SSH keys are properly configured with correct permissions (600 for authorized_keys, 700 for .ssh)

## Compatibility

- Backward compatible with existing secrets and environment variables
- Maintains same deployment workflow behavior
- Supports both Tailscale and public IP connectivity

## Future Enhancements

- Consider adding support for additional Linux distributions
- Implement health checks during StackScript execution
- Add monitoring integration during server creation
- Support for custom Docker registry authentication
