# syntax=docker/dockerfile:1.26@sha256:ecfaec9ed6d810b56388c508f4121597bfbba70d41a6dfeee4d8cad5f295fc32

# Global Build Args
ARG USER_ID=1001
ARG GROUP_ID=1001

# Every build stage is pinned to the *build* platform and cross-compiles to the
# requested target platform. No target-architecture code is ever executed during
# the build, so multi-arch images are produced without QEMU emulation.
FROM --platform=$BUILDPLATFORM rust:1.97-slim@sha256:8e8cf8f7fd54a2d23d5a743b3a03f56e26b6c774276c33fa0595111704ebb15c AS base
ARG BUILDARCH

# Besides the native toolchain, install the GNU cross toolchain for the
# architecture the build host cannot target natively. It is only used as a
# linker driver and for `strip`; the musl runtime itself is supplied by rustc.
RUN set -eux; \
    case "${BUILDARCH}" in \
      amd64) cross_toolchain=gcc-aarch64-linux-gnu ;; \
      arm64) cross_toolchain=gcc-x86-64-linux-gnu ;; \
      *) echo "unsupported build architecture: ${BUILDARCH}" >&2; exit 1 ;; \
    esac; \
    apt-get update; \
    apt-get install -y \
      pkg-config \
      libssl-dev \
      wget \
      tar \
      curl \
      musl-tools \
      upx \
      "${cross_toolchain}"; \
    rm -rf /var/lib/apt/lists/*

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Add rust targets
RUN rustup target add \
    x86_64-unknown-linux-musl \
    aarch64-unknown-linux-musl \
    wasm32-unknown-unknown

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-gnu-gcc \
    CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=aarch64-linux-gnu-gcc

# Single source of truth for the Docker architecture -> Rust target mapping.
# The binutils prefix of a triple is derived from its architecture component,
# e.g. `aarch64-unknown-linux-musl` -> `aarch64-linux-gnu-strip`.
COPY <<'EOF' /usr/local/bin/rust-target
#!/bin/sh
set -eu
case "$1" in
  amd64) echo x86_64-unknown-linux-musl ;;
  arm64) echo aarch64-unknown-linux-musl ;;
  *) echo "unsupported architecture: $1" >&2; exit 1 ;;
esac
EOF
RUN chmod +x /usr/local/bin/rust-target

WORKDIR /app

FROM base AS chef
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Dependencies for the build host's own architecture. Everything that has to
# *run* during the build (currently the converter) is compiled from here so it
# executes natively, and its output stays independent of the target platform.
FROM chef AS host_cacher
ARG BUILDARCH
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target "$(rust-target "${BUILDARCH}")" --recipe-path recipe.json

# Build the converter binary once in a dedicated stage so that the (expensive)
# data conversion below is only re-run when the converter source or the input
# data actually changes - not on every unrelated frontend/server edit.
FROM host_cacher AS converter_builder
ARG BUILDARCH
COPY . .
RUN set -eux; \
    target="$(rust-target "${BUILDARCH}")"; \
    cargo build --release --target "${target}" -p mp-stats-converter; \
    cp "target/${target}/release/converter" /usr/local/bin/converter

FROM converter_builder AS data-optimizer
ARG DATA_INPUT_DIRECTORY=data

# The converter is deterministic with respect to its input, so we keep a
# persistent build cache and let it skip re-processing editions whose input is
# byte-for-byte unchanged. This prevents useless re-calculation of the data
# artifacts across image rebuilds.
#
# The converter takes no arguments: every path comes from the layered
# configuration (see `docs/CONFIGURATION.md`). Here it is the environment layer
# rather than a TOML file, because these three paths are build-stage scaffolding
# - `/app/data` is a bind mount and `/app/.converter_cache` a cache mount, both
# of which exist only for the duration of this RUN.
RUN --mount=type=bind,source=${DATA_INPUT_DIRECTORY},target=/app/data \
    --mount=type=cache,id=converter-cache,target=/app/.converter_cache,sharing=locked \
    MP_STATS_CONVERTER__INPUT_DIR=/app/data \
    MP_STATS_CONVERTER__OUTPUT_DIR=/app/data-dist \
    MP_STATS_CONVERTER__CACHE__DIR=/app/.converter_cache \
    converter

# Dependencies for the target architecture. Layered on top of `host_cacher` so
# that a native build (target == build architecture) reuses those artifacts
# instead of compiling the same dependency set twice.
FROM host_cacher AS backend_cacher
ARG TARGETARCH
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target "$(rust-target "${TARGETARCH}")" --recipe-path recipe.json

FROM backend_cacher AS backend_builder
ARG TARGETARCH
COPY . .
RUN set -eux; \
    target="$(rust-target "${TARGETARCH}")"; \
    cargo build --release --target "${target}" -p mp-stats-server; \
    "${target%%-*}-linux-gnu-strip" --strip-all "target/${target}/release/server"; \
    upx --best --lzma "target/${target}/release/server"; \
    cp "target/${target}/release/server" /server

FROM chef AS frontend_base
RUN apt-get update && apt-get install -y nodejs npm
RUN cargo binstall trunk

# The frontend compiles to wasm and is therefore identical for every target
# platform; keeping it free of `TARGETARCH` lets multi-arch builds share it.
FROM frontend_base AS frontend
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target wasm32-unknown-unknown --recipe-path recipe.json

COPY . .
WORKDIR /app/apps/frontend
RUN npm install
RUN trunk build --release

# Only architecture-independent files (accounts, certificates, timezone data)
# are taken from this stage, so it can stay on the build platform.
FROM --platform=$BUILDPLATFORM alpine:3.24@sha256:28bd5fe8b56d1bd048e5babf5b10710ebe0bae67db86916198a6eec434943f8b AS env
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

COPY --from=backend_builder /server /server
COPY --from=frontend /app/apps/frontend/dist /dist
COPY --from=data-optimizer /app/data-dist /dist/data

# The image describes itself: the layout inside it is a property of the image,
# not something an operator should have to re-state on every `docker run`. It
# stays overridable - a bind mount over /config.toml replaces it, a different
# MP_STATS_CONFIG points elsewhere, and MP_STATS_SERVER__* wins over both.
COPY deploy/config.toml /config.toml
ENV MP_STATS_CONFIG=/config.toml

EXPOSE 8080
USER ${USER_ID}:${GROUP_ID}
ENTRYPOINT ["/server"]
