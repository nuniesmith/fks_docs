# GitHub Secrets Configuration

This document outlines the GitHub secrets required for the Docker build workflow.

## Required Secrets

### Docker Hub Authentication

Add these secrets to your GitHub repository settings under **Settings > Secrets and variables > Actions**:

1. **`DOCKER_USERNAME`**
   - Description: Your Docker Hub username
   - Value: Your Docker Hub username (e.g., `nuniesmith`)

2. **`DOCKER_PASSWORD`**
   - Description: Your Docker Hub access token or password
   - Value: Docker Hub access token (recommended) or password
   - **Note**: It's recommended to use a Docker Hub access token instead of your password for security

## How to Set Up Secrets

### Step 1: Create Docker Hub Access Token (Recommended)

1. Log in to [Docker Hub](https://hub.docker.com/)
2. Go to **Account Settings > Security**
3. Click **New Access Token**
4. Give it a name (e.g., "GitHub Actions FKS")
5. Select permissions (Read, Write, Delete)
6. Copy the generated token

### Step 2: Add Secrets to GitHub Repository

1. Go to your GitHub repository
2. Navigate to **Settings > Secrets and variables > Actions**
3. Click **New repository secret**
4. Add the following secrets:
   - Name: `DOCKER_USERNAME`, Value: Your Docker Hub username
   - Name: `DOCKER_PASSWORD`, Value: Your Docker Hub access token

## Environment Variables

The workflow uses the following environment variables that are automatically configured:

- `REGISTRY`: Set to `docker.io` (Docker Hub)
- `NAMESPACE`: Set to `nuniesmith/fks` (can be changed in the workflow file)

## Docker Images

The workflow will create the following Docker images:

- `nuniesmith/fks:api`
- `nuniesmith/fks:data`
- `nuniesmith/fks:worker`
- `nuniesmith/fks:app`
- `nuniesmith/fks:web`
- `nuniesmith/fks:ninja-dev`
- `nuniesmith/fks:ninja-python`
- `nuniesmith/fks:ninja-build-api`

Each image will also be tagged with:
- Branch name + service name (e.g., `main-api`)
- SHA prefix + service name (e.g., `api-abc1234`)
- Latest tag for main branch (e.g., `api-latest`)

## Memory Optimization

The workflow is designed to be memory-efficient for GitHub's basic runners:

- Builds one service at a time (`max-parallel: 1`)
- Cleans up Docker resources after each build
- Removes unnecessary system files before building
- Uses inline caching to speed up subsequent builds
- Disables provenance to save space and time

## Manual Triggering

You can manually trigger builds for specific services:

1. Go to **Actions** tab in your repository
2. Click on "Build and Push Docker Images" workflow
3. Click **Run workflow**
4. Select the service you want to build or choose "all" for all services
5. Click **Run workflow**

## Troubleshooting

### Build Failures

If builds fail due to memory issues:

1. The workflow already includes memory optimization steps
2. Consider upgrading to larger GitHub runners if available
3. Check the build logs for specific error messages

### Authentication Issues

If you get authentication errors:

1. Verify `DOCKER_USERNAME` and `DOCKER_PASSWORD` secrets are correctly set
2. Make sure the Docker Hub access token has the correct permissions
3. Check that the token hasn't expired

### Service-Specific Issues

Each service has specific build arguments configured in the workflow. If a particular service fails:

1. Check the Dockerfile at `./deployment/docker/Dockerfile`
2. Verify the source code exists in the expected directories
3. Check the requirements files exist for Python services
