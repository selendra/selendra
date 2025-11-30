# syntax=docker/dockerfile:1

FROM ubuntu:22.04 AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Download pre-built binaries
RUN curl -L https://github.com/selendra/selendra/releases/download/2.0.6/selendra-testnet -o /tmp/selendra-node && \
    chmod +x /tmp/selendra-node

RUN curl -L https://github.com/selendra/selendra/releases/download/2.0.6/chain-bootstrapper -o /tmp/chain-bootstrapper && \
    chmod +x /tmp/chain-bootstrapper

# Runtime stage
FROM ubuntu:22.04 AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    jq \
    curl \
    procps \
    net-tools \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 selendra

# Copy downloaded binaries
COPY --from=builder /tmp/selendra-node /usr/local/bin/
COPY --from=builder /tmp/chain-bootstrapper /usr/local/bin/

# Copy scripts
COPY --chmod=0755 run.node.sh /usr/local/bin/
COPY common.sh /selendra/

# Set working directory
WORKDIR /selendra

# Create data directory
RUN mkdir -p /selendra/run-nodes-local && chown selendra:selendra /selendra/run-nodes-local

# Switch to non-root user for runtime
USER selendra

# Expose ports
EXPOSE 9944-9954 30333-30353 9615-9625

# Run the script directly
CMD ["/usr/local/bin/run.node.sh", "--validators", "4", "--rpc-nodes", "1"]
