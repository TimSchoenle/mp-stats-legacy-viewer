# Local tooling. `just` with no arguments lists what there is.
#
# Everything a contributor has to run by hand lives here rather than in a script under
# `.github/scripts/`, so that the command a README quotes, the command CI runs and the command a
# developer types are one string. Recipes that only wrap `cargo` are here for the same reason:
# the flags are the part people get wrong.
#
#     https://github.com/casey/just
#
# There is deliberately no recipe that *checks* the generated artefacts. Checking is
# `TimSchoenle/actions/actions/rust/config-contract`, which does it in three places this file
# cannot reach — against the Dockerfile, against the committed document, and against the labels a
# built image actually carries. A second implementation here would be a second opinion, and the
# whole point of the shared action is that there is only one.

# The generator, and where its output belongs. These five lines are the only per-repository part
# of this file.
example := "config-schema"
features := "config-schema"
package := "mp-stats-config"
contract := "docs/config.contract.json"
dockerfile := "Dockerfile"

# The rest of what this repository generates: the Handlebars partials the two documentation
# templates inject, and the file an operator copies to `config.toml`. The tables are partials
# rather than template variables because a Markdown table is multi-line text with pipes in it,
# and threading that through a strict-JSON workflow input means escaping it in a shell script.
partials := "target/config-docs"
toml_example := "config.example.toml"

# Read for the two pinned dependency tags the documentation quotes, which is the one thing in the
# templates that is neither prose nor generated from the configuration types.
manifest := "Cargo.toml"

# The markers `--format dockerfile` emits around the LABEL block. Defined by terrace-config, not
# by this repository: cutting the region by line count reads correctly right up until a fourth
# label is added, and then compares two of three lines and passes.
begin := "# terrace-config:labels:begin"
end := "# terrace-config:labels:end"

[private]
default:
    @just --list --unsorted

[doc('Rewrite everything generated from crates/config')]
regenerate: contract-json dockerfile-labels tables toml-example

[doc('Print one rendering: json|markdown|markdown-loader|markdown-keys|toml|json-schema|contract|labels|dockerfile')]
[group('generate')]
render format only="":
    #!/usr/bin/env bash
    set -euo pipefail
    args=(run --quiet --example "{{ example }}")
    [ -n "{{ package }}" ] && args+=(-p "{{ package }}")
    [ -n "{{ features }}" ] && args+=(--features "{{ features }}")
    only=()
    [ -n "{{ only }}" ] && only=(--only "{{ only }}")
    cargo "${args[@]}" -- --format "{{ format }}" "${only[@]}"

# Rendered without `--version`, `--revision` or `--created`, so it is byte-reproducible across
# rebuilds and releases: the committed copy describes the configuration surface, and the copy
# inside an image additionally names the build it came from.

[doc('Rewrite the committed contract document')]
[group('generate')]
contract-json:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "$(dirname "{{ contract }}")"
    just render contract > "{{ contract }}"
    echo "wrote {{ contract }}"

# The file is rebuilt around the markers rather than substituted in place: `sed` cannot replace a
# multi-line block portably, and `--format dockerfile` emits both markers along with the block
# between them.

[doc('Rewrite the LABEL region in the Dockerfile')]
[group('generate')]
dockerfile-labels:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! grep -qF '{{ begin }}' "{{ dockerfile }}" || ! grep -qF '{{ end }}' "{{ dockerfile }}"; then
        echo "error: {{ dockerfile }} carries no '{{ begin }}' … '{{ end }}' region, so the" >&2
        echo "       generated LABEL block has nowhere to go. Paste 'just render dockerfile'" >&2
        echo "       into it once, markers included." >&2
        exit 1
    fi
    block="$(mktemp)"
    rewritten="$(mktemp)"
    trap 'rm -f "$block" "$rewritten"' EXIT
    just render dockerfile > "$block"
    {
        sed -n "1,/^{{ begin }}\$/p" "{{ dockerfile }}" | sed '$d'
        cat "$block"
        sed -n "/^{{ end }}\$/,\$p" "{{ dockerfile }}" | sed '1d'
    } > "$rewritten"
    mv "$rewritten" "{{ dockerfile }}"
    echo "wrote the LABEL region in {{ dockerfile }}"

# Addressed from a template by path, so the directory level is part of a partial's name:
# `target/config-docs/config/server-keys.hbs` is referenced as `config/server-keys`.
#
# The loader's own variables are rendered once, for the page that carries them above the first
# key table, rather than repeated over every subsystem — which is why the two subsystem tables
# are `markdown --only`, the rendering that drops the loader table and keeps the keys.

[doc('Rewrite the Markdown table partials the documentation templates inject')]
[group('generate')]
tables:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p "{{ partials }}/config"
    just render markdown-loader > "{{ partials }}/config/loader-variables.hbs"
    just render markdown server > "{{ partials }}/config/server-keys.hbs"
    just render markdown converter > "{{ partials }}/config/converter-keys.hbs"
    echo "wrote the table partials in {{ partials }}/config"

[doc('Rewrite the example configuration an operator copies to config.toml')]
[group('generate')]
toml-example:
    #!/usr/bin/env bash
    set -euo pipefail
    just render toml > "{{ toml_example }}"
    echo "wrote {{ toml_example }}"

# Deliberately POSIX tools only, no `jq`: it is not present in a default Git for Windows shell,
# and a command that only runs on the CI runner is a command nobody checks their edit against.
#
# The tag is read out of the workspace dependency's inline table and refused unless it is an
# alphabet `printf` can be trusted with, which is what keeps this free of a JSON encoder.

[doc('Print the scalar variables the documentation templates read, as JSON')]
[group('generate')]
docs-variables:
    #!/usr/bin/env bash
    set -euo pipefail
    tag() {
        local crate="$1" value
        value="$(sed -n "s/^${crate} = {.*tag = \"\([^\"]*\)\".*/\1/p" "{{ manifest }}" | head -n1)"
        if [ -z "${value}" ]; then
            echo "docs-variables: no git tag for '${crate}' in {{ manifest }}" >&2
            return 1
        fi
        if ! printf '%s' "${value}" | grep -Eq '^[0-9A-Za-z][0-9A-Za-z.+-]*$'; then
            echo "docs-variables: '${crate}' tag '${value}' is not a version tag" >&2
            return 1
        fi
        printf '%s' "${value}"
    }
    printf '{"terrace_config_tag":"%s","csp_shell_tag":"%s"}\n' "$(tag terrace-config)" "$(tag csp-shell)"

[doc('Format, lint and test — what a pull request is going to run anyway')]
[group('check')]
verify: fmt lint test

[group('check')]
fmt:
    cargo fmt --all

# `--workspace`, because the crate this file is mostly about is a member rather than the root
# package and the root package alone compiles almost nothing. No `-D warnings`: the frontend
# carries warnings that predate this file, and the gate that a pull request has to clear is
# `TimSchoenle/actions/actions/rust/cargo-check`, not a second opinion here.

[group('check')]
lint:
    cargo clippy --workspace --all-features --all-targets

[group('check')]
test:
    cargo test --workspace --all-features
