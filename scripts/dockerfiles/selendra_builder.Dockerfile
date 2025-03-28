# This is the build stage for Selendra. Here we create the binary in a temporary image.
FROM rust:1.79-slim AS builder

ARG VCS_REF=unknown

WORKDIR /selendra
COPY . .

# Install required dependencies and build
RUN apt-get update && \
    apt-get install -y build-essential clang libclang-dev pkg-config libssl-dev && \
    cargo build --locked --release && \
    ls -la /selendra/target/release/

# This is the 2nd stage: a very small image where we copy the Selendra binary."
FROM docker.io/library/ubuntu:22.04

# Install minimal runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

LABEL description="Multistage Docker image for Selendra: an L1 Network for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="info@selendra.org" \
	io.parity.image.vendor="Selendra" \
	io.parity.image.description="Selendra: an L1 Network for web3" \
	io.parity.image.source="https://github.com/selendra/selendra/blob/${VCS_REF}/scripts/dockerfiles/selendra_builder.Dockerfile" \
	io.parity.image.documentation="https://github.com/selendra/selendra/"

# Copy the compiled binaries from the builder stage
COPY --from=builder /selendra/target/release/selendra* /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /selendra selendra && \
	mkdir -p /data /selendra/.local/share && \
	chown -R selendra:selendra /data && \
	ln -s /data /selendra/.local/share/selendra && \
# unclutter and minimize the attack surface23

	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/selendra --version || ls -la /usr/local/bin/

USER selendra

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/selendra"]
