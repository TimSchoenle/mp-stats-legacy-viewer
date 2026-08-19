#!/usr/bin/env bash
#
# Regenerates everything the configuration types are the source of truth for: the Markdown tables
# the documentation templates inject, `config.example.toml`, and the configuration contract the
# image publishes.
#
# Run it yourself after changing a key, or let the documentation workflow run it - it renders the
# templates against whatever this produces and commits the result back to the branch:
#
#     bash .github/scripts/config-docs.sh [partials-directory]
#
# The tables are written as Handlebars partials rather than passed as template variables. A
# Markdown table is multi-line text with pipes in it, and threading that through a strict-JSON
# workflow input means escaping it in a shell script; a partial is the same bytes on disk, read
# by the renderer directly.
set -euo pipefail

partials="${1:-target/config-docs}"

# Addressed from a template by path, so the `config/` level is part of the partial's name:
# `target/config-docs/config/server-keys.hbs` is `{{> config/server-keys }}`.
mkdir -p "${partials}/config"

generate() {
    cargo run --quiet -p mp-stats-config --features config-schema --example config-schema -- "$@"
}

generate --format loader >"${partials}/config/loader-variables.hbs"
generate --format markdown --only server >"${partials}/config/server-keys.hbs"
generate --format markdown --only converter >"${partials}/config/converter-keys.hbs"

# Not a partial: nothing renders it, and an operator copies it to `config.toml` as it stands.
generate --format toml >config.example.toml

# Nor is this one: it is read by a machine, not by a person. The committed copy is what makes a
# configuration change reviewable - a removed key shows up in the pull request that removed it,
# next to the deployment it is about to break - while the copy a deployment trusts is the one the
# image build generates and attaches to its own digest.
#
# `--revision` and `--created` are deliberately not passed. They move between builds of one source
# tree, so the committed copy carries neither; `app.version` is here because it moves with
# `Cargo.toml`, which is a source change like any other and belongs in the diff.
generate --format contract >docs/config.contract.json
