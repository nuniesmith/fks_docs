# FKS Development Services Setup

This guide will help you resolve the CORS errors, missing favicon, and WebSocket connection issues you're experiencing.

## Quick Fix Summary

The issues you're seeing are:
1. **CORS errors**: React app can't connect to API services on different ports
2. **Missing favicon**: 404 error for favicon.ico 
3. **WebSocket failures**: Services on ports 8002 and 8081 not running
4. **Service unavailability**: Python API and VS Code server not accessible

## Solution: Use WSL with Shell Scripts

### Prerequisites
1. **WSL (Windows Subsystem for Linux)** - Install if not already available
2. **Node.js** in WSL
3. **Python 3** in WSL

### Setup Instructions

1. **Open WSL terminal** (not PowerShell):
   ```bash
   wsl
   ```

2. **Navigate to your project directory**:
   ```bash
   cd /mnt/c/Users/Jordan/Documents/ninja
   ```

3. **Make scripts executable**:
   ```bash
   chmod +x *.sh
   ```

4. **Run the setup and test script**:
   ```bash
   ./setup-and-test.sh
   ```

### Starting Services

#### Option 1: Start All Services (Recommended for development)
```bash
./start-dev-servers.sh
```
This starts:
- Python API server (port 8002)
- VS Code proxy server (port 8081)  
- React development server (port 3000)

#### Option 2: Start Just API Services
```bash
./start-api-servers.sh
```
Then manually start React:
```bash
cd web
npm start
```

### Stopping Services

```bash
./stop-dev-servers.sh
```

### Service URLs
- **React App**: http://localhost:3000
- **Python API**: http://localhost:8002/healthz
- **VS Code Proxy**: http://localhost:8081/healthz

## What Was Fixed

### 1. Added Missing Favicon
- Created `web/public/favicon.svg`
- Updated `index.html` to reference it

### 2. Fixed CORS Issues
- Enhanced `web/server.js` with proper CORS configuration
- Created `python/main.py` FastAPI server with CORS middleware
- Created `vscode-proxy.js` proxy server for VS Code health checks

### 3. Added WebSocket Support
- Python FastAPI server includes WebSocket endpoint at `/ws`
- React components now handle WebSocket connection failures gracefully
- Auto-reconnection logic for WebSocket connections

### 4. Improved Error Handling
- Added timeouts for health check requests
- Better error logging and user feedback
- Graceful fallbacks when services are unavailable

## Files Created/Modified

### New Files:
- `python/main.py` - FastAPI server with CORS and WebSocket support
- `vscode-proxy.js` - VS Code server proxy
- `web/public/favicon.svg` - Application favicon
- `start-dev-servers.sh` - Complete development setup
- `start-api-servers.sh` - API services only
- `stop-dev-servers.sh` - Stop all services
- `setup-and-test.sh` - Setup and test environment
- `package.json` - Root package.json for dependencies

### Modified Files:
- `web/server.js` - Enhanced CORS configuration
- `web/public/index.html` - Added favicon reference
- `web/src/components/LGMMTradingAnalyzer.tsx` - Better WebSocket error handling
- `web/src/FKSProjectManager.tsx` - Improved health check timeouts
- `python/requirements.txt` - Added websockets and python-dotenv

## Troubleshooting

### Services Won't Start
```bash
# Check what's using the ports
lsof -i :3000 -i :8002 -i :8081

# Kill processes on specific ports
sudo kill -9 $(lsof -t -i:8002)
```

### WSL Issues
```bash
# Update WSL packages
sudo apt update && sudo apt upgrade

# Install missing dependencies
sudo apt install curl lsof nodejs npm python3 python3-pip
```

### Still Getting CORS Errors?
1. Make sure all services are running: `./start-dev-servers.sh`
2. Check service health: 
   - http://localhost:8002/healthz
   - http://localhost:8081/healthz
3. Restart all services: `./stop-dev-servers.sh && ./start-dev-servers.sh`

## Using Docker (Alternative)

If you prefer Docker, you can also use:
```bash
docker-compose up
```

This will start all services in containers with proper networking configured.
