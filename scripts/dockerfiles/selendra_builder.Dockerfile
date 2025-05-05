# Stage 1: Build Selendra binary
FROM docker.io/paritytech/ci-linux:production AS builder

WORKDIR /selendra
COPY . /selendra

# Remove --locked if you haven't generated Cargo.lock locally
RUN cargo build --release

# Stage 2: Create minimal runtime container
FROM ubuntu:20.04

LABEL description="Multistage Docker image for Selendra: a platform for web3" \
    io.parity.image.type="builder" \
    io.parity.image.authors="info@selendra.org" \
    io.parity.image.vendor="Selendra" \
    io.parity.image.description="Selendra: a platform for web3" \
    io.parity.image.source="https://github.com/selendra/selendra" \
    io.parity.image.documentation="https://github.com/selendra/selendra/"

ENV DEBIAN_FRONTEND=noninteractive

# Install only necessary runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        ca-certificates \
        libssl-dev \
        libclang-dev \
        libudev-dev \
        curl && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /selendra/target/release/selendra-node /usr/local/bin/selendra

# Create selendra user and setup directory
RUN useradd -m -u 1000 -U -s /bin/sh -d /selendra selendra && \
    mkdir -p /data /selendra/.local/share && \
    chown -R selendra:selendra /data /selendra

USER selendra

# Expose required ports
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/selendra"]
