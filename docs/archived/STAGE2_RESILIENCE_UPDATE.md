# Stage 2 Resilience Update

## Problem
The Stage 2 systemd service was failing to run after reboot because of a hard dependency on Docker (`Requires=docker.service`). When Docker failed to start (which is common on Arch Linux due to storage driver issues), the Stage 2 service would never run, preventing Tailscale installation and the rest of the post-reboot deployment.

## Solution
Made the Stage 2 deployment more resilient by removing hard dependencies and improving error handling:

### 1. Systemd Service Changes
- **Removed**: `Requires=docker.service` (hard dependency)
- **Added**: `Wants=docker.service` (soft dependency - prefers Docker but won't fail without it)
- **Added**: `ExecStartPre=/bin/sleep 30` (30-second delay to let system settle after boot)

### 2. Enhanced Error Handling in `verify_services_post_reboot()`
- **Non-blocking Docker fixes**: Docker startup failures no longer prevent Stage 2 from continuing
- **Improved logging**: Added more detailed Docker status and error logging
- **Graceful degradation**: If Docker can't be fixed, Stage 2 continues without Docker
- **Better diagnostics**: Shows systemctl status and journalctl logs for troubleshooting

### 3. Docker-aware Function Updates

#### `build_and_deploy_docker()`
- **Pre-checks**: Verifies Docker is running before attempting build/deploy
- **Graceful skipping**: Skips Docker operations if Docker isn't functional
- **Helpful messages**: Provides manual recovery commands when Docker operations fail

#### `create_ninja_service()`
- **Conditional creation**: Only creates the ninja-trading service if Docker is available
- **Soft dependency**: Uses `Wants=docker.service` instead of `Requires=docker.service`
- **Correct path**: Fixed WorkingDirectory to `/home/ninja/ninja`

### 4. Preserved Critical Functions
- **Tailscale installation**: Always runs regardless of Docker status
- **Repository cloning**: Always runs regardless of Docker status
- **SSH setup**: Always runs regardless of Docker status
- **User configuration**: Always runs regardless of Docker status

## Expected Behavior After Update

### Successful Boot Scenario
1. System boots
2. Stage 2 service waits 30 seconds for system to settle
3. Verifies and fixes services (including Docker if possible)
4. Installs and configures Tailscale
5. Clones repository and sets up environment
6. Builds and deploys Docker (if Docker is working)
7. Creates monitoring and services
8. Completes successfully

### Docker-Failed Boot Scenario  
1. System boots
2. Stage 2 service waits 30 seconds for system to settle
3. Attempts to fix Docker issues, but Docker still fails
4. **Continues anyway** (doesn't exit)
5. Installs and configures Tailscale ✓
6. Clones repository and sets up environment ✓
7. Skips Docker build/deploy with helpful messages
8. Skips Docker-dependent services
9. Completes Stage 2 with warnings about Docker

### Manual Recovery
If Docker issues persist, users can:
1. Fix Docker manually: `systemctl start docker`
2. Run Docker deployment: `cd /home/ninja/ninja && docker-compose up -d`
3. Create ninja-trading service manually

## Files Modified
- `scripts/StackScript.sh` - Main StackScript with resilience improvements
- Stage 2 systemd service definition (within StackScript)
- `verify_services_post_reboot()` function
- `build_and_deploy_docker()` function  
- `create_ninja_service()` function

## Testing Recommendations
1. Deploy on fresh Linode instance
2. Verify Stage 2 runs automatically after reboot
3. Confirm Tailscale is installed and configured
4. Check that Docker issues are handled gracefully
5. Verify manual recovery procedures work

## Deployment Priority
**High Priority**: Tailscale connectivity is now guaranteed to be configured even if Docker fails, ensuring remote access and management capabilities are maintained.
