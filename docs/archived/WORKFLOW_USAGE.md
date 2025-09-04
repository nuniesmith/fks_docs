# GitHub Actions Workflow Usage Guide

## 🚀 Quick Start

### 1. **Setup GitHub Secrets**
Go to your repository → Settings → Secrets and variables → Actions

Add these secrets:
```
DOCKER_USERNAME      # Your Docker Hub username
DOCKER_TOKEN         # Your Docker Hub token/password
LINODE_CLI_TOKEN     # Your Linode API token
FKS_DEV_ROOT_PASSWORD # Password for new servers
```

### 2. **Test Builds First**
- Go to Actions tab in your repository
- Click "Run workflow" on the deployment workflow
- Select `test-builds` mode
- Click "Run workflow"

### 3. **Full Deployment**
- Go to Actions tab in your repository
- Click "Run workflow" on the deployment workflow
- Select `full-deploy` mode
- Choose your environment (development/production)
- Click "Run workflow"

## 📋 **Deployment Modes**

| Mode | Description | What it does |
|------|-------------|--------------|
| `test-builds` | Test Docker builds only | Builds all images, tests build process |
| `full-deploy` | Complete deployment | Infrastructure + Build + Deploy |
| `builds-only` | Build and push images | Just Docker builds, no deployment |
| `deploy-only` | Deploy existing images | Use existing images, just deploy |
| `infra-only` | Setup infrastructure | Create/setup servers only |

## 🎛️ **Common Usage Patterns**

### **First Time Setup:**
```
1. Run: test-builds (verify builds work)
2. Run: full-deploy (create infrastructure + deploy)
```

### **Code Updates:**
```
1. Run: builds-only (build new images)
2. Run: deploy-only (deploy updated images)
```

### **Quick Deploy:**
```
1. Run: full-deploy (everything in one go)
```

### **Infrastructure Only:**
```
1. Run: infra-only (just setup servers)
```

## 🔧 **Troubleshooting**

### **Build Failures:**
- Check Docker build logs in Actions
- Ensure all required files exist
- Verify build arguments are correct

### **Deployment Failures:**
- Check server SSH connectivity
- Verify GitHub secrets are correct
- Check server disk space and Docker status

### **Health Check Failures:**
- Check service logs: `docker-compose logs`
- Verify ports are not blocked
- Check service configuration

## 📊 **Monitoring Deployment**

### **During Deployment:**
1. Watch GitHub Actions logs in real-time
2. Check each job status
3. Monitor deployment summary

### **After Deployment:**
1. SSH to server: `ssh root@YOUR_SERVER_IP`
2. Check services: `docker-compose ps`
3. View logs: `docker-compose logs`
4. Test endpoints manually

## 🛡️ **Security Best Practices**

### **Secrets Management:**
- Never commit secrets to repository
- Use GitHub Secrets for sensitive data
- Rotate tokens regularly

### **Server Security:**
- Use strong passwords
- Enable firewall
- Keep system updated
- Monitor access logs

## 🎯 **Tips for Success**

1. **Always test builds first** before full deployment
2. **Monitor logs** during deployment
3. **Check health status** after deployment
4. **Keep backups** of working configurations
5. **Use staging environment** for testing

## 📱 **Quick Commands**

### **Local Development:**
```bash
# Test builds locally
./test-docker-builds.sh

# Validate workflow
./validate-workflow.sh

# Start local development
docker-compose up -d
```

### **Server Management:**
```bash
# SSH to server
ssh root@YOUR_SERVER_IP

# Check services
docker-compose ps

# View logs
docker-compose logs -f

# Restart services
docker-compose restart
```

## 🆘 **Getting Help**

If you encounter issues:
1. Check GitHub Actions logs
2. Review server logs via SSH
3. Verify all secrets are set correctly
4. Test Docker builds locally first
5. Check network connectivity to server

---

## 🎉 **You're Ready!**

Your GitHub Actions workflow is now configured and ready to deploy your FKS trading system. Start with `test-builds` mode to verify everything works, then proceed with `full-deploy` for complete deployment.

Happy deploying! 🚀
