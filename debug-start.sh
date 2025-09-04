#!/bin/bash
echo "Starting FKS Docs Service..."
echo "Binary path: $(which fks_docs)"
echo "Binary exists: $(test -f /usr/local/bin/fks_docs && echo 'YES' || echo 'NO')"
echo "Binary permissions: $(ls -la /usr/local/bin/fks_docs)"

# Try to run the binary
echo "Attempting to start service..."
exec /usr/local/bin/fks_docs --listen 0.0.0.0:8040
