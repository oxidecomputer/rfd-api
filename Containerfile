# syntax=docker/dockerfile:1
# Build stage
FROM docker.io/rust:1-trixie AS builder

ARG DIESEL_VER=v2.3.5

WORKDIR /app

# Install build dependencies for diesel/postgres
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config


# Copy workspace files
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Copy all workspace members
COPY parse-rfd ./parse-rfd
COPY rfd-api ./rfd-api
COPY rfd-cli ./rfd-cli
COPY rfd-data ./rfd-data
COPY rfd-github ./rfd-github
COPY rfd-installer ./rfd-installer
COPY rfd-model ./rfd-model
COPY rfd-processor ./rfd-processor
COPY rfd-sdk ./rfd-sdk
COPY trace-request ./trace-request
COPY xtask ./xtask

ENV CARGO_HOME=/data/cargo

# Build all target binaries in release mode
RUN  cargo build --release \
    --package rfd-api \
    --package rfd-processor \
    --package rfd-cli \
    --package rfd-installer

# Download diesel tool for migrations
WORKDIR /tmp
RUN curl -L --output-dir /tmp -O "https://github.com/diesel-rs/diesel/releases/download/${DIESEL_VER}/diesel_cli-x86_64-unknown-linux-gnu.tar.xz" \
    && curl -L --output-dir /tmp -O "https://github.com/diesel-rs/diesel/releases/download/${DIESEL_VER}/diesel_cli-x86_64-unknown-linux-gnu.tar.xz.sha256" \
    && sha256sum diesel_cli-x86_64-unknown-linux-gnu.tar.xz.sha256 && tar --strip-components=1 -xJvf diesel_cli-x86_64-unknown-linux-gnu.tar.xz

# Runtime stage
FROM docker.io/debian:trixie-slim

# Install runtime dependencies and tini
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    tini \
    && rm -rf /var/lib/apt/lists/*

RUN useradd --create-home --user-group rfd \
    && mkdir /home/rfd/db

# Copy binaries from builder
COPY --from=builder /app/target/release/rfd-api /usr/local/bin/
COPY --from=builder /app/target/release/rfd-processor /usr/local/bin/
COPY --from=builder /app/target/release/rfd-cli /usr/local/bin/
COPY --from=builder /app/target/release/rfd-installer /usr/local/bin/

# Database migrations for diesel
COPY --from=builder /tmp/diesel /usr/local/bin/
# COPY --from=builder /app/rfd-model/diesel.toml /home/rfd/db/
COPY --from=builder /app/rfd-model/migrations/ /home/rfd/db/migrations/

# Create non-root user
USER rfd
WORKDIR /home/rfd

# Use tini as entrypoint for proper signal handling
ENTRYPOINT ["/usr/bin/tini", "--"]

# Default to running rfd-api, can be overridden with CMD
CMD ["rfd-api"]
