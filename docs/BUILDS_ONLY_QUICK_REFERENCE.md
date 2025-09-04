# 🚀 Builds-Only Quick Reference

## 🎯 Problem Solved

You encountered the issue where running builds would trigger full deployment including infrastructure provisioning, which then failed and prevented Docker builds from running.

## ✅ Solutions Implemented

### **1. Enhanced Commit Message Detection**
The workflow now automatically detects builds-only mode from commit messages:

**Trigger Patterns:**
- `builds only` 
- `only builds`
- `build only` 
- `only build`

**Example commits that auto-trigger builds-only:**
```bash
git commit --allow-empty -m "force rebuild cpu builds only"
git commit --allow-empty -m "only build docker images"
git commit --allow-empty -m "build only - skip deployment"
```

### **2. GitHub Actions UI Method**
Use the workflow UI with these exact settings:

```
Deployment mode: builds-only          👈 Key setting!
Build Options: force-rebuild-cpu      👈 Force all CPU services
Enable CPU builds: ✓ true            
Enable GPU builds: ✗ false           👈 Skip GPU for faster builds
Skip code checks: ✓ true             👈 Skip for speed
Skip React tests: ✓ true             👈 Skip for speed
```

## 🔄 What This Does

**With builds-only mode:**
- ✅ Builds all Docker images (6 CPU services: api, worker, data, web, nginx, ninja-api)
- ✅ Pushes images to Docker Hub
- ⏭️ **Skips infrastructure provisioning** (no Linode server creation)
- ⏭️ **Skips deployment** (no server setup)
- ⏭️ **Skips code checks** (optional, for speed)

**Build time:** ~15-20 minutes instead of 40+ minutes

## 📊 CPU Services Built

1. **api** - Main API service (FastAPI/Python)
2. **worker** - Background task processor  
3. **data** - Data management service
4. **web** - React frontend application
5. **nginx** - Web server and reverse proxy
6. **ninja-api** - NinjaTrader integration API

## 🎯 Quick Commands

**Force build CPU services only:**
```bash
git commit --allow-empty -m "force rebuild cpu builds only"
git push
```

**Force build all services:**
```bash
git commit --allow-empty -m "force rebuild all builds only"
git push
```

## 🔍 How to Verify It's Working

Look for this in the workflow logs:

```
🔄 Auto-switching to builds-only mode based on commit message: 'force rebuild cpu builds only'
📝 This will skip infrastructure and deployment, only build Docker images
ℹ️ Skipping infrastructure - will reuse existing server or run builds-only
🔄 Force rebuild requested via commit message: 'force rebuild cpu builds only'
🔄 Builds-only mode detected via commit message: 'force rebuild cpu builds only'
```

## ✅ Expected Results

- **All 6 CPU services** should show "✅ CPU Service: [name] (will build)"
- **Build time**: ~15-20 minutes
- **All images pushed** to Docker Hub with `latest` and SHA tags
- **No infrastructure errors** (infrastructure steps are skipped)

---

**Commit that triggered this:** `3a6b867 - force rebuild cpu builds only - skip infrastructure`
