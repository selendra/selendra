# This is the build stage for Selendra. Here we create the binary in a temporary image.
FROM docker.io/paritytech/ci-linux:production as builder

WORKDIR /selendra
COPY . /selendra

RUN cargo build --locked --release

# This is the 2nd stage: a very small image where we copy the Selendra binary."
FROM docker.io/library/ubuntu:20.04

LABEL description="Multistage Docker image for Selendra: a platform for web3" \
	io.parity.image.type="builder" \
	io.parity.image.authors="info@selendra.org" \
	io.parity.image.vendor="Selendra" \
	io.parity.image.description="Selendra: a platform for web3" \
	io.parity.image.source="https://github.com/selendra/selendra/blob/${VCS_REF}/scripts/dockerfiles/selendra/selendra_builder.Dockerfile" \
	io.parity.image.documentation="https://github.com/selendra/selendra/"

COPY --from=builder /selendra/target/release/selendra /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /selendra selendra && \
	mkdir -p /data /selendra/.local/share && \
	chown -R selendra:selendra /data && \
	ln -s /data /selendra/.local/share/selendra && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
	/usr/local/bin/selendra --version

USER selendra

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/selendra"]
