FROM rust:1.85-slim AS base

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    tar \
    curl \
    && rm -rf /var/lib/apt/lists/*

FROM base AS frontend
WORKDIR /app

# Install Node.js 20.x (for Tailwind CSS)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Trunk for WASM builds
RUN wget -qO- https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-unknown-linux-gnu.tar.gz \
    | tar -xzf- -C /usr/local/bin/

# Add WASM build target
RUN rustup target add wasm32-unknown-unknown

COPY . .

# Install npm dependencies
RUN cd apps/frontend && npm install

# Build Tailwind CSS
RUN cd apps/frontend && npm run build:css

# Build WASM/SPA frontend via Trunk
RUN cd apps/frontend && trunk build --release

FROM base AS data-optimizer
WORKDIR /app

COPY . .

# Convert data to optimized binary format (Postcard + Zlib)
RUN --mount=type=bind,source=data,target=/app/data \
    cargo run --release -p mp-stats-converter -- /app/data /app/data-dist

FROM base AS builder
WORKDIR /app

COPY . .

# Build the server (default glibc target)
RUN cargo build --release -p mp-stats-server

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the server binary
COPY --from=builder /app/target/release/server /server

# Copy static frontend assets
COPY --from=frontend /app/apps/frontend/dist /dist

# Copy optimized data
COPY --from=data-optimizer /app/data-dist /dist/data

EXPOSE 8080
ENTRYPOINT ["/server", "--dir", "/dist", "--data-dir", "/dist/data"]
