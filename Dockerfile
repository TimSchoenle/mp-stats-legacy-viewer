# syntax=docker/dockerfile:1.21@sha256:27f9262d43452075f3c410287a2c43f5ef1bf7ec2bb06e8c9eeb1b8d453087bc

# Global Build Args
ARG USER_ID=1001
ARG GROUP_ID=1001

FROM rust:1.93-slim@sha256:9663b80a1621253d30b146454f903de48f0af925c967be48c84745537cd35d8b AS base
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    tar \
    curl \
    musl-tools \
    upx \
    && rm -rf /var/lib/apt/lists/*

# Add rust targets
RUN rustup target add x86_64-unknown-linux-musl
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

FROM base AS chef
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend_cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

FROM backend_cacher AS backend_builder
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl -p mp-stats-server

RUN strip --strip-all /app/target/x86_64-unknown-linux-musl/release/server && \
    upx --best --lzma /app/target/x86_64-unknown-linux-musl/release/server

FROM backend_cacher AS data-optimizer
ARG DATA_INPUT_DIRECTORY=data
COPY . .

RUN --mount=type=bind,source=${DATA_INPUT_DIRECTORY},target=/app/data \
    cargo run --release --target x86_64-unknown-linux-musl -p mp-stats-converter -- /app/data /app/data-dist

FROM chef AS frontend_base
RUN apt-get update && apt-get install -y nodejs npm
RUN cargo install trunk

FROM frontend_base AS frontend
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

COPY . .
WORKDIR /app/apps/frontend
RUN npm install
RUN trunk build --release

FROM alpine:3.23@sha256:25109184c71bdad752c8312a8623239686a9a2071e8825f20acb8f2198c3f659 AS env
ARG USER_ID

# mailcap is used for content type (MIME type) detection
# tzdata is used for timezones info
RUN apk add --no-cache \
    ca-certificates \
    mailcap \
    tzdata && \
    update-ca-certificates && \
    adduser \
        --disabled-password \
        --gecos "" \
        --home "/nonexistent" \
        --shell "/sbin/nologin" \
        --no-create-home \
        --uid "${USER_ID}" \
        "appuser"

FROM scratch AS runtime

ARG USER_ID
ARG GROUP_ID

COPY --from=env /etc/passwd /etc/passwd
COPY --from=env /etc/group /etc/group
COPY --from=env /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=env /usr/share/zoneinfo /usr/share/zoneinfo

COPY --from=backend_builder /app/target/x86_64-unknown-linux-musl/release/server /server
COPY --from=frontend /app/apps/frontend/dist /dist
COPY --from=data-optimizer /app/data-dist /dist/data

EXPOSE 8080
USER ${USER_ID}:${GROUP_ID}
ENTRYPOINT ["/server", "--dir", "/dist", "--data-dir", "/dist/data"]