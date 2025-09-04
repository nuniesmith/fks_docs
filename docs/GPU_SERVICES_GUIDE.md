# FKS GPU Services Guide

This guide explains how to use the GPU-based training and transformer services with Docker Compose profiles.

## Overview

The FKS system now includes two GPU-accelerated services:

1. **Training Service** - Long-running ML model training for cloud-based API
2. **Transformer Service** - AI-powered market analysis and insights

These services are designed to run on your home GPU hardware while the main FKS services run in the cloud.

## Architecture

```
┌─────────────────┐    ┌─────────────────┐
│   Cloud (CPU)   │    │   Home (GPU)    │
│                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │    API      │ │    │ │  Training   │ │
│ │   Service   │ │◄──►│ │  Service    │ │
│ └─────────────┘ │    │ └─────────────┘ │
│                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │    Data     │ │    │ │ Transformer │ │
│ │   Service   │ │◄──►│ │   Service   │ │
│ └─────────────┘ │    │ └─────────────┘ │
│                 │    │                 │
│ ┌─────────────┐ │    │ ┌─────────────┐ │
│ │   Worker    │ │    │ │  Database   │ │
│ │   Service   │ │    │ │  (Optional) │ │
│ └─────────────┘ │    │ └─────────────┘ │
└─────────────────┘    └─────────────────┘
```

## Prerequisites

### Hardware Requirements
- NVIDIA GPU with CUDA support (GTX 1060/RTX 2060 or better)
- At least 8GB GPU memory (16GB+ recommended for transformer service)
- 16GB+ system RAM
- Docker with nvidia-container-toolkit installed

### Software Requirements
- Docker Engine 20.10+
- Docker Compose v2
- NVIDIA Docker runtime
- CUDA 12.8+ drivers

## Installation

### 1. Install NVIDIA Container Toolkit

```bash
# Ubuntu/Debian
curl -fsSL https://nvidia.github.io/libnvidia-container/gpgkey | sudo gpg --dearmor -o /usr/share/keyrings/nvidia-container-toolkit-keyring.gpg
curl -s -L https://nvidia.github.io/libnvidia-container/stable/deb/nvidia-container-toolkit.list | \
  sed 's#deb https://#deb [signed-by=/usr/share/keyrings/nvidia-container-toolkit-keyring.gpg] https://#g' | \
  sudo tee /etc/apt/sources.list.d/nvidia-container-toolkit.list

sudo apt-get update
sudo apt-get install -y nvidia-container-toolkit
sudo systemctl restart docker
```

### 2. Verify GPU Access

```bash
# Test NVIDIA runtime
docker run --rm --gpus all nvidia/cuda:12.8-base-ubuntu24.04 nvidia-smi
```

## Usage

### Environment Files

Three environment files are provided:

1. **`.env.development`** - CPU services only (development)
2. **`.env.cloud`** - CPU services for cloud deployment  
3. **`.env.gpu`** - GPU services for home deployment

### Docker Compose Profiles

The system uses Docker Compose profiles to separate services:

- **Default profile**: CPU services (api, data, worker, app, etc.)
- **GPU profile**: GPU services (training, transformer)
- **ML profile**: Both GPU services
- **Home profile**: All services including GPU

### Deployment Scenarios

#### 1. Home Development (All Services)

Run everything locally including GPU services:

```bash
# Use GPU environment
cp .env.gpu .env

# Start all services including GPU
docker-compose --profile gpu up -d

# Or use the home profile
docker-compose --profile home up -d
```

#### 2. Cloud Deployment (CPU Only)

Deploy only CPU services to cloud:

```bash
# Use cloud environment  
cp .env.cloud .env

# Start only CPU services (default profile)
docker-compose up -d
```

#### 3. Split Deployment (Recommended)

**On Cloud Server:**
```bash
# Deploy CPU services
cp .env.cloud .env
docker-compose up -d api data worker app web nginx
```

**On Home GPU Machine:**
```bash
# Deploy GPU services
cp .env.gpu .env
docker-compose --profile gpu up -d training transformer

# Or build locally
docker-compose -f docker-compose.yml -f docker-compose.gpu.yml --profile gpu up -d
```

#### 4. Training Service Only

```bash
docker-compose --profile training up -d training
```

#### 5. Transformer Service Only

```bash
docker-compose --profile transformer up -d transformer
```

## Service Details

### Training Service

**Purpose**: Long-running ML model training triggered by cloud API

**Ports**:
- `8088`: Main API endpoint
- `6006`: TensorBoard interface
- `8889`: Jupyter notebooks

**Key Features**:
- LSTM, Transformer, and CNN model support
- Automatic retraining based on performance thresholds
- Remote job queue integration with cloud API
- TensorBoard monitoring
- Model checkpointing and versioning

**API Endpoints**:
```bash
# Health check
curl http://localhost:8088/health

# Start training job
curl -X POST http://localhost:8088/api/v1/train \
  -H "Content-Type: application/json" \
  -d '{"model_type": "lstm", "epochs": 50}'

# Check training status
curl http://localhost:8088/api/v1/status

# Get trained models
curl http://localhost:8088/api/v1/models
```

### Transformer Service

**Purpose**: AI-powered market analysis and insights generation

**Ports**:
- `8089`: Main API endpoint
- `7860`: Gradio web interface
- `8890`: Jupyter notebooks

**Key Features**:
- Natural language processing for market analysis
- Sentiment analysis of news and social media
- Automated report generation
- Interactive web interface via Gradio
- Support for multiple transformer models

**API Endpoints**:
```bash
# Health check
curl http://localhost:8089/health

# Analyze market sentiment
curl -X POST http://localhost:8089/api/v1/analyze \
  -H "Content-Type: application/json" \
  -d '{"text": "Market news text here", "analysis_type": "sentiment"}'

# Generate market summary
curl -X POST http://localhost:8089/api/v1/summary \
  -d '{"timeframe": "daily", "symbols": ["AAPL", "TSLA"]}'

# Web interface
open http://localhost:7860
```

## Configuration

### Remote API Connection

Both services can connect to your cloud API for job coordination:

```bash
# In .env.gpu
CLOUD_API_HOST=api.fkstrading.xyz
CLOUD_API_PORT=8000
CLOUD_API_TOKEN=your_api_token_here
```

### GPU Memory Management

Configure GPU memory settings:

```bash
# Training service
TRAINING_GPU_COUNT=1
PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:512

# Transformer service  
TRANSFORMER_GPU_COUNT=1
PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:1024
```

### Model Configuration

Customize models in the service config files:

- `config/services/training.yaml`
- `config/services/transformer.yaml`

## Monitoring

### TensorBoard (Training)

```bash
# Access TensorBoard
open http://localhost:6006
```

### Gradio Interface (Transformer)

```bash
# Access AI interface
open http://localhost:7860
```

### Jupyter Notebooks

```bash
# Training notebooks
open http://localhost:8889

# Transformer notebooks  
open http://localhost:8890
```

### Logs

```bash
# View service logs
docker-compose logs -f training
docker-compose logs -f transformer

# View specific log files
tail -f logs/training.log
tail -f logs/transformer.log
```

## Troubleshooting

### GPU Not Detected

```bash
# Check NVIDIA drivers
nvidia-smi

# Check Docker GPU support
docker run --rm --gpus all nvidia/cuda:12.8-base-ubuntu24.04 nvidia-smi

# Check container GPU access
docker-compose exec training nvidia-smi
```

### Out of Memory Errors

```bash
# Reduce batch sizes in .env.gpu
TRAINING_BATCH_SIZE=32
TRANSFORMER_BATCH_SIZE=4

# Enable memory growth
PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:256
```

### Service Connection Issues

```bash
# Check network connectivity
docker-compose exec training ping api.fkstrading.xyz

# Check service health
curl http://localhost:8088/health
curl http://localhost:8089/health
```

## Performance Optimization

### Training Service

1. **Batch Size**: Increase based on GPU memory
2. **Mixed Precision**: Enable for faster training
3. **Data Loading**: Use multiple workers
4. **Checkpointing**: Save frequently to prevent loss

### Transformer Service

1. **Model Size**: Choose appropriate model for your GPU
2. **Quantization**: Use int8 quantization for inference
3. **Caching**: Enable model and result caching
4. **Batching**: Process multiple requests together

## Integration with Cloud API

The GPU services are designed to work with your cloud-based FKS API:

1. **Training Jobs**: Cloud API queues training requests
2. **Model Deployment**: Trained models uploaded to cloud
3. **Analysis Tasks**: Transformer processes analysis requests
4. **Results Delivery**: Insights sent back to cloud API

Example cloud integration:

```python
# Cloud API triggers training
import requests

response = requests.post('http://gpu-server:8088/api/v1/train', json={
    'model_type': 'lstm',
    'data_source': 'daily_bars',
    'symbols': ['AAPL', 'GOOGL', 'MSFT'],
    'epochs': 100
})

# GPU service trains model and uploads results
# Cloud API receives trained model for inference
```

## Cost Optimization

### Cloud vs Home Deployment

**Cloud GPU Costs** (AWS p3.2xlarge):
- ~$3.06/hour = $2,203/month for 24/7 training

**Home GPU Costs**:
- RTX 4090: ~$1,600 one-time + electricity
- Pays for itself in ~1 month vs cloud GPU

### Resource Management

1. **Auto-scaling**: Stop services when not training
2. **Scheduled Training**: Run intensive jobs during off-peak hours
3. **Model Optimization**: Use smaller models when possible
4. **Shared Resources**: Run multiple services on same GPU

## Security Considerations

1. **API Authentication**: Use tokens for cloud communication
2. **Network Security**: VPN or secure tunnels for cloud connection
3. **Data Privacy**: Keep sensitive data on home servers
4. **Model Protection**: Encrypt model artifacts
5. **Access Control**: Limit service access to authorized users

## Next Steps

1. Set up your GPU environment using this guide
2. Test services with sample data
3. Configure cloud API integration
4. Set up monitoring and alerting
5. Optimize for your specific use case

For support, check the logs or create an issue in the repository.
