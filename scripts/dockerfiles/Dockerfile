# Production Selendra RPC Node
FROM ubuntu:22.04

# Metadata
LABEL maintainer="KOOMPI <admin@koompi.org>"
LABEL description="Selendra RPC Node for Docker Swarm"
LABEL version="1.0.0"

# Set environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV BASE_PATH=/data/selendradb
ENV CHAIN=selendra
ENV NODE_NAME=selendra-rpc
ENV RPC_PORT=9933
ENV P2P_PORT=40333
ENV RPC_MAX_CONNECTIONS=1000

# Install required dependencies
RUN apt-get update && \
    apt-get install -y \
        ca-certificates \
        curl \
        libssl3 \
        && rm -rf /var/lib/apt/lists/* \
        && apt-get clean

# Create a non-root user for security
RUN useradd -m -u 1000 selendra && \
    mkdir -p /data && \
    chown -R selendra:selendra /data

# Set working directory
WORKDIR /app

# Copy the Selendra binary
COPY selendra-node /app/selendra-node

# Make binary executable and set ownership
RUN chmod +x /app/selendra-node && \
    chown selendra:selendra /app/selendra-node

# Switch to non-root user
USER selendra

# Create volume mount point
VOLUME ["/data"]

# Expose ports
EXPOSE ${RPC_PORT} ${P2P_PORT}

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f -H "Content-Type: application/json" \
        -d '{"id":1,"jsonrpc":"2.0","method":"system_health","params":[]}' \
        http://localhost:${RPC_PORT} || exit 1

# Entry point
ENTRYPOINT ["/app/selendra-node"]

# Default command with fixed parameters (environment variables don't work in CMD array format)
CMD ["--chain", "selendra", \
     "--base-path", "/data/selendradb", \
     "--name", "selendra-rpc", \
     "--rpc-port", "9933", \
     "--port", "40333", \
     "--no-mdns", \
     "--pool-limit", "1024", \
     "--db-cache", "2048", \
     "--max-runtime-instances", "8", \
     "--runtime-cache-size", "4", \
     "--rpc-external", \
     "--rpc-cors", "all", \
     "--rpc-max-connections", "1000"]
