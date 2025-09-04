# Auto-Update Setup Documentation

## Overview
This setup provides automatic monitoring and updating of the FKS repository from GitHub, with automatic application restart when updates are detected.

## Files Created

### 1. `auto_update.sh`
- **Purpose**: Main script that checks for GitHub updates and restarts the application
- **Location**: `/home/jordan/fks/auto_update.sh`
- **Schedule**: Runs every 5 minutes via cron
- **Functionality**:
  - Fetches latest changes from GitHub
  - Compares local and remote commits
  - Pulls updates if available
  - Restarts application using `./start.sh`
  - Logs all activity to `/home/jordan/fks/logs/auto_update.log`

### 2. `check_auto_update.sh`
- **Purpose**: Status monitoring script
- **Location**: `/home/jordan/fks/check_auto_update.sh`
- **Usage**: Run manually to check system status
- **Shows**:
  - Cron service status
  - Current script execution status
  - Last log entries
  - Repository status
  - Application status

## Cron Job Configuration
```
*/5 * * * * /home/jordan/fks/auto_update.sh
```
This runs the auto-update script every 5 minutes.

## Log Files
- **Main log**: `/home/jordan/fks/logs/auto_update.log`
- **Contains**: Timestamps, git operations, success/error messages

## Security Features
- **Lock file**: Prevents multiple instances running simultaneously
- **SSH key authentication**: Uses existing SSH keys for GitHub access
- **Process management**: Safely kills and restarts application processes

## Manual Operations

### Check Status
```bash
cd /home/jordan/fks
./check_auto_update.sh
```

### View Recent Logs
```bash
tail -f /home/jordan/fks/logs/auto_update.log
```

### Manual Update
```bash
cd /home/jordan/fks
./auto_update.sh
```

### Manage Cron Job
```bash
# View current cron jobs
crontab -l

# Edit cron jobs
crontab -e

# Remove all cron jobs
crontab -r
```

## Service Management
```bash
# Check cron service status
systemctl status cronie

# Start cron service
sudo systemctl start cronie

# Enable cron service (start on boot)
sudo systemctl enable cronie
```

## Troubleshooting

### Common Issues
1. **Cron not running**: Check `systemctl status cronie`
2. **Script not executing**: Check file permissions and paths
3. **Git access issues**: Verify SSH keys are properly configured
4. **Lock file issues**: Remove `/tmp/auto_update.lock` if stuck

### Debugging
- Check logs: `cat /home/jordan/fks/logs/auto_update.log`
- Test manually: `./auto_update.sh`
- Run status check: `./check_auto_update.sh`

## Notes
- The script only pulls updates from the `main` branch
- Application restart is handled automatically when updates are found
- The system uses SSH key authentication (already configured)
- Logs are rotated automatically by the system
