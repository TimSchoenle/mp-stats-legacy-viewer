# syntax=docker/dockerfile:1.26@sha256:ecfaec9ed6d810b56388c508f4121597bfbba70d41a6dfeee4d8cad5f295fc32

# Global Build Args
ARG USER_ID=1001
ARG GROUP_ID=1001

# Every build stage is pinned to the *build* platform and cross-compiles to the
# requested target platform. No target-architecture code is ever executed during
# the build, so multi-arch images are produced without QEMU emulation.
FROM --platform=$BUILDPLATFORM rust:1.98-slim@sha256:cc0448b41c3b7b7fea44f5dc50eacba729a56db365b65b7bd5e8a82d5b3db078 AS base
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

# The configuration contract: the document a deployment pipeline reads to check
# that what it renders is what this image loads - every key in every spelling,
# the same keys as a JSON Schema, and the variables outside the `MP_STATS_`
# namespace this container tolerates.
#
# On `host_cacher` and pinned to the build platform, not on `backend_builder`:
# the document describes types, not machine code, so it is identical for every
# target architecture and building it once is what keeps the two platforms of a
# multi-arch image carrying the same bytes. Nothing the `config-schema` feature
# links - `serde_json`, `syn`, the derive - reaches the binary the runtime stage
# copies.
#
# Both files come out of one invocation pair in one stage, which is the only
# arrangement in which the labels and the document cannot disagree: two runs at
# different times can, and the labels are what a consumer finds the document by.
FROM host_cacher AS contract-builder
ARG BUILDARCH
COPY . .
RUN set -eux; \
    target="$(rust-target "${BUILDARCH}")"; \
    mkdir -p /out; \
    cargo run -q --locked --release --target "${target}" \
      -p mp-stats-config --features config-schema --example config-schema \
      -- --format contract > /out/contract.json; \
    cargo run -q --locked --release --target "${target}" \
      -p mp-stats-config --features config-schema --example config-schema \
      -- --format labels > /out/contract.labels

# The two files alone, so `--output type=local` hands the host `contract.json`
# and `contract.labels` rather than exporting the whole Rust toolchain image the
# stage above is built on.
FROM scratch AS contract-export
COPY --from=contract-builder /out /out

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

# The offline copy of the contract: what makes the image self-describing with no
# registry at all - an exported tarball, an air-gapped mirror, an initContainer
# reading it in-cluster. The canonical copy is the OCI referrer attached to the
# pushed digest; this one costs a few kilobytes and needs nothing to fetch it.
COPY --from=contract-builder /out/contract.json /config/contract.json

# How anything finds that document without pulling a layer. All three values are
# constants for this service - the envelope version, where the file was COPYed,
# and the loader's prefix - so the block is written out rather than interpolated:
# a `LABEL` key cannot be interpolated at all, and feeding `--label` from the
# generator would mean running it a second time on the host, where the builder
# stage that produced the document is out of reach.
#
# Hand-carried means it can be wrong in ways a source diff cannot see - a line
# dropped on a branch nobody diffed, a base image contributing its own - so the
# check is on the built image instead, against `contract.labels` from the same
# generator run. `--format dockerfile` emits exactly this block, and the `Config
# Contract` job diffs the two so the copy here cannot drift from the document it
# points at.

# The markers are terrace-config's own, and cutting the region at them is
# what the shared check does. The line-count cut this repository used -
# `grep -A2 '^LABEL dev\.terrace\.config'` - reads correctly right up until
# a fourth label is added, and then compares two of three lines and passes.
# terrace-config:labels:begin
LABEL dev.terrace.config.contract.version="1" \
      dev.terrace.config.contract.path="/config/contract.json" \
      dev.terrace.config.prefix="MP_STATS_"
# terrace-config:labels:end

EXPOSE 8080
USER ${USER_ID}:${GROUP_ID}
ENTRYPOINT ["/server"]
