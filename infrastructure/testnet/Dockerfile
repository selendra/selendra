# syntax=docker/dockerfile:1

###############################################
# Builder Stage (Arch Linux)
###############################################
FROM archlinux:latest AS builder

# Install dependencies (curl, jq)
RUN pacman -Sy --noconfirm curl jq

# Download pre-built binaries
RUN curl -L https://github.com/selendra/selendra/releases/download/2.0.6/selendra-testnet -o /tmp/selendra-node \
    && chmod +x /tmp/selendra-node

RUN curl -L https://github.com/selendra/selendra/releases/download/2.0.6/chain-bootstrapper -o /tmp/chain-bootstrapper \
    && chmod +x /tmp/chain-bootstrapper


###############################################
# Runtime Stage (Arch Linux)
###############################################
FROM archlinux:latest AS runtime

# Install runtime utilities
RUN pacman -Sy --noconfirm \
    curl \
    jq \
    procps-ng \
    net-tools \
    && pacman -Scc --noconfirm

# Create non-root user
RUN useradd -m -u 1000 selendra

# Copy built binaries
COPY --from=builder /tmp/selendra-node /usr/local/bin/
COPY --from=builder /tmp/chain-bootstrapper /usr/local/bin/

# Copy scripts
COPY --chmod=0755 run.node.sh /usr/local/bin/
COPY common.sh /selendra/

# Working directory
WORKDIR /selendra

# Create data directory
RUN mkdir -p /selendra/run-nodes-local \
    && chown selendra:selendra /selendra/run-nodes-local

# Switch to user
USER selendra

# Expose ports
EXPOSE 9944-9954 30333-30353 9615-9625

# Run command
CMD /usr/local/bin/run.node.sh --validators ${VALIDATORS:-4} --rpc-nodes ${RPC_NODES:-1} \
    ${DONT_BOOTSTRAP:+--dont-bootstrap} \
    ${DONT_DELETE_DB:+--dont-delete-db} \
    ${DONT_REMOVE_ABFT_BACKUPS:+--dont-remove-abft-backups}
