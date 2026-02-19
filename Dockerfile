FROM rust:1.93-slim@sha256:9663b80a1621253d30b146454f903de48f0af925c967be48c84745537cd35d8b AS base

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    tar \
    curl \
    && rm -rf /var/lib/apt/lists/*

FROM base AS frontend_base
WORKDIR /app

RUN apt-get update && apt-get install -y nodejs npm
RUN cd apps/frontend & npm init -y && npm install tailwindcss @tailwindcss/cli

# Install Trunk for WASM builds
RUN cargo install --locked trunk

# Add WASM build target
RUN rustup target add wasm32-unknown-unknown

FROM frontend_base AS frontend

COPY . .

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

FROM debian:bookworm-slim@sha256:98f4b71de414932439ac6ac690d7060df1f27161073c5036a7553723881bffbe

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
