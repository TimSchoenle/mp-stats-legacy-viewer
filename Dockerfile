# syntax=docker/dockerfile:1.23@sha256:2780b5c3bab67f1f76c781860de469442999ed1a0d7992a5efdf2cffc0e3d769

# Global Build Args
ARG USER_ID=1001
ARG GROUP_ID=1001

FROM rust:1.95-slim@sha256:c03ea1587a8e4474ae1a3f4a377cbb35ad53d2eb5c27f0bdf1ca8986025e322f AS base
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    wget \
    tar \
    curl \
    musl-tools \
    upx \
    && rm -rf /var/lib/apt/lists/*

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

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
RUN cargo binstall trunk

FROM frontend_base AS frontend
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

COPY . .
WORKDIR /app/apps/frontend
RUN npm install
RUN trunk build --release

FROM alpine:3.23@sha256:5b10f432ef3da1b8d4c7eb6c487f2f5a8f096bc91145e68878dd4a5019afde11 AS env
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