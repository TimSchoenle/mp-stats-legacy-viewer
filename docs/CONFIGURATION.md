<!--
Generated from .github/templates/CONFIGURATION.md.hbs - edit that file, not docs/CONFIGURATION.md.
The prose is here; every table below is a partial written by .github/scripts/config-docs.sh
straight out of the structs in crates/config, so no key, default or environment spelling in this
page is maintained by hand.
-->
# Configuration reference

Every key the platform reads, what it does, its default, and which binary consumes it.

Configuration is **file-first**: a TOML file describes a deployment, and the environment is one
override layer above it rather than the only way in. Both binaries load through the same
loader, so one file can describe the whole platform.

---

## 1. How configuration is loaded

Layered, lowest precedence first. The layering is
[`terrace-config`](https://github.com/TimSchoenle/terrace-config), pinned at
`v0.6.0`; which variable names *this* deployment spells is
[`crates/config/src/loader.rs`](../crates/config/src/loader.rs); the typed blocks are the rest of
[`crates/config`](../crates/config/src/lib.rs).

1. The `#[serde(default)]` value compiled into each field — the tables below.
2. TOML at `$MP_STATS_CONFIG`, defaulting to `./config.toml` and **silently skipped if
   absent**. If it names a *directory*, every `*.toml` directly inside it is merged in
   file-name order, later winning — a `ConfigMap` mounted as a set of fragments.
3. Environment variables prefixed `MP_STATS_`.
4. Files in `$MP_STATS_SECRETS_DIR`, one per key, named after the key ([§4](#4-file-backed-layers)).
5. `MP_STATS_<KEY>_FILE=/path`, which reads `<KEY>` from that path ([§4](#4-file-backed-layers)).

**Layers 3, 4 and 5 are mutually exclusive per key.** A key supplied by two of them fails the
boot, naming the key — it is not resolved by precedence. What that prevents is a half-migrated
deployment in which a stale environment variable shadows a mounted file that has since been
changed: the service keeps working, on the old value, and the discrepancy surfaces during an
incident rather than during a deploy.

**Nesting is `__` (two underscores).** A single underscore is part of a field name, not a
separator — the single most common way to get this wrong. The `Environment` column of every
table in [§3](#3-keys) carries the exact spelling, derived rather than written down:
`server.bind_addr` is `MP_STATS_SERVER__BIND_ADDR`, and `converter.cache.dir` is
`MP_STATS_CONVERTER__CACHE__DIR`.

An **unknown** `MP_STATS_*` key is ignored, not rejected. That is what lets one file carry both
blocks — the server does not fail on `[converter]` — but it also means a typo fails silently,
as does a key that has been removed (see [§5](#5-removed-keys)). `MP_STATS_EXPLAIN=1`
([§2](#2-where-the-file-comes-from)) is how to see what was actually read.

Nothing here is required: with no file and no variables set, both binaries run on the defaults
below, which match a workspace checkout.

---

## 2. Where the file comes from

| Variable | Role | Default | Purpose |
|---|---|---|---|
| `MP_STATS_CONFIG` | config | `config.toml` | Names the TOML layer: a file, or a directory whose `*.toml` files are all merged in name order. |
| `MP_STATS_SECRETS_DIR` | secrets dir | — | Names a directory of key-named files — a mounted Kubernetes `Secret` volume. Each file supplies the key its name spells. |
| `MP_STATS_EXPLAIN` | reserved | — | Read directly from the environment before the layered config exists, so no file may supply it. |

`MP_STATS_CONFIG` and `MP_STATS_SECRETS_DIR` are read to decide what the other layers *are*, so
neither can itself be supplied by a file; `terrace-config` reserves them and refuses an attempt
rather than ignoring it. `MP_STATS_EXPLAIN` is reserved for the same reason — it decides whether
the layers are described, which is answered before they exist.

Set `MP_STATS_EXPLAIN` to `1` or `true` and either binary writes a report to stderr at boot:
every layer, the file or variable behind it, and the key each one supplied, with anything
shadowed listed under the value that won. It holds no configuration *value*, so it is safe in a
log a secrets-carrying deployment writes. It is printed even when the boot fails, which is the
case it exists for.

The container image ships [`deploy/config.toml`](../deploy/config.toml) at `/config.toml` and
points `MP_STATS_CONFIG` at it, so `docker run` needs no arguments. Replace it with a bind
mount, point `MP_STATS_CONFIG` somewhere else, or override single keys with `MP_STATS_*`.

For a local checkout, copy [`config.example.toml`](../config.example.toml) to `config.toml`
(git-ignored) and edit.

---

## 3. Keys

### `[server]` — consumed by `mp-stats-server`

| TOML | Type | Environment | Default | Flags | Purpose |
|---|---|---|---|---|---|
| `server.bind_addr` | `SocketAddr` | `MP_STATS_SERVER__BIND_ADDR` | `0.0.0.0:8080` | — | Address the HTTP listener binds. |
| `server.dist_dir` | `PathBuf` | `MP_STATS_SERVER__DIST_DIR` | `dist` | — | Directory holding the built frontend. |
| `server.data_dir` | `PathBuf` | `MP_STATS_SERVER__DATA_DIR` | `data` | — | Directory holding the converter's output, mounted at `/data`. |
| `server.csp.enabled` | `bool` | `MP_STATS_SERVER__CSP__ENABLED` | `true` | — | Send the header at all. |
| `server.csp.cloudflare.script_nonce` | `bool` | `MP_STATS_SERVER__CSP__CLOUDFLARE__SCRIPT_NONCE` | `false` | — | Reserve a per-response nonce in `script-src`. |
| `server.csp.cloudflare.turnstile` | `bool` | `MP_STATS_SERVER__CSP__CLOUDFLARE__TURNSTILE` | `false` | — | Admit `https://challenges.cloudflare.com` in `script-src` **and** `frame-src`. |
| `server.csp.cloudflare.web_analytics` | `bool` | `MP_STATS_SERVER__CSP__CLOUDFLARE__WEB_ANALYTICS` | `false` | — | Admit Cloudflare Web Analytics: the beacon script, and the endpoint it reports to. |

### `[converter]` — consumed by `mp-stats-converter`

| TOML | Type | Environment | Default | Flags | Purpose |
|---|---|---|---|---|---|
| `converter.input_dir` | `PathBuf` | `MP_STATS_CONVERTER__INPUT_DIR` | `data` | — | Directory holding the raw per-edition data dumps. Must exist; the converter refuses to start otherwise. |
| `converter.output_dir` | `PathBuf` | `MP_STATS_CONVERTER__OUTPUT_DIR` | `target/converted_data` | — | Directory the optimized output is written to. Must differ from the input directory. |
| `converter.cache.enabled` | `bool` | `MP_STATS_CONVERTER__CACHE__ENABLED` | `true` | — | Restore from and store into the cache directory. |
| `converter.cache.dir` | `PathBuf` | `MP_STATS_CONVERTER__CACHE__DIR` | `target/converter_cache` | — | Where cached output and its input fingerprints live. |

Each key is also readable from a file: `MP_STATS_<KEY>_FILE` naming a path, or a file named
after the key in the secrets directory ([§4](#4-file-backed-layers)) — `server.data_dir` is
`server__data_dir` there.

---

## 4. File-backed layers

Neither binary reads a secret today — no database, no credentials, no tokens. The two
file-backed layers are still available, and are the reason this loader was adopted rather than
a plain TOML parse:

* **Secrets directory.** Every file in `$MP_STATS_SECRETS_DIR` supplies the key its *name*
  spells, in the same `__`-nested, case-folded spelling minus the prefix:
  `/run/secrets/server__data_dir` supplies `server.data_dir`. A `.` in a file name is refused
  rather than treated as a separator. The provider follows the `..data` symlink a Kubernetes
  projected volume uses and skips the dot-prefixed entries, so it works against a real mounted
  `Secret`.
* **`_FILE` indirection.** `MP_STATS_SERVER__DATA_DIR_FILE=/path` reads the value from that
  path — the Docker convention.

---

## 5. Removed keys

These were read directly from the environment before the migration to layered configuration.
They are **no longer read at all**; a deployment still setting one is silently running on the
default.

| Removed | Replacement |
|---|---|
| `CONVERTER_CACHE_DIR` | `converter.cache.dir` / `MP_STATS_CONVERTER__CACHE__DIR` |
| `CONVERTER_NO_CACHE` | `converter.cache.enabled = false` / `MP_STATS_CONVERTER__CACHE__ENABLED=false` |

The server's `--dir` and `--data-dir` command-line flags are gone with them: `server.dist_dir`
and `server.data_dir` replace them, and the binary now takes no arguments. The converter's two
positional arguments are likewise replaced by `converter.input_dir` and
`converter.output_dir`.

---

## 6. The `Content-Security-Policy`

The header the server sends is **derived, not configured**. At startup the server reads the
`index.html` it is about to serve and computes a `'sha256-…'` for every inline `<script>` in it,
using [`csp-shell`](https://github.com/TimSchoenle/csp-shell), pinned at `csp-shell-v0.2.0`;
those hashes go into `script-src` alongside the rest of a WebAssembly-SPA policy. There is
deliberately no key that spells the policy out, because a policy written in a config file drifts
from the bundle the moment anyone rebuilds the frontend, and the browser reports that drift by
refusing the scripts and rendering a blank page.

Two consequences worth knowing:

* **An unreadable or unscannable shell fails the boot**, before the listener binds. The shell
  boots WASM from an inline script, so a policy missing that hash is a blank page — the same
  posture as the `index.html` existence check.
* **The header lands on documents only.** A `Content-Security-Policy` on a font or a `/data`
  blob is inert, and the `Cache-Control` below would be actively wrong there.

What the keys in `[server.csp.cloudflare]` decide is the part that genuinely differs between
deployments: which Cloudflare products this one runs. Each is off by default.

### `script_nonce` and its deployment obligation

Cloudflare's bot products — Bot Fight Mode, JavaScript Detections, the challenge platform —
inject an inline `<script>` into the served HTML **at the edge**, after the shell was hashed. No
hash can cover it, `script-src` refuses it, and the detection silently never runs: bot management
appears enabled and does nothing. Cloudflare's documented answer is to read the nonce out of the
`Content-Security-Policy` response header and copy it onto what it injects, so nothing has to be
stamped into the shell.

Setting `script_nonce = true` mints 128 CSPRNG bits per response and serves every document
`Cache-Control: no-cache`. That header is an obligation, not a suggestion — a nonce served from a
cache is shared by every reader of that entry, which admits exactly the inline script the nonce
exists to constrain.

The second half of the obligation cannot be met from inside the process and belongs in the
deployment checklist:

> **No Cloudflare Cache Rule may cache the shell.** A "Cache Everything" rule overrides the origin
> `Cache-Control`, satisfying the obligation at the origin and violating it at the edge.

---

## 7. Examples

A full local override:

```toml
# config.toml
[server]
bind_addr = "127.0.0.1:3000"
dist_dir = "apps/frontend/dist"
data_dir = "target/converted_data"

[converter]
input_dir = "data-test"
```

The same, one key at a time, without a file:

```bash
MP_STATS_SERVER__BIND_ADDR=127.0.0.1:3000 cargo run -p mp-stats-server
```

A directory of fragments, merged in file-name order — a base plus an environment overlay:

```bash
MP_STATS_CONFIG=/etc/mp-stats/conf.d cargo run -p mp-stats-server
# /etc/mp-stats/conf.d/10-base.toml
# /etc/mp-stats/conf.d/20-production.toml
```

Which of those a running deployment actually read:

```bash
MP_STATS_EXPLAIN=1 cargo run -p mp-stats-server
```

---

## 8. The contract the image publishes

Everything above is for a human. The same surface is published in a form a deployment pipeline
reads, so that a chart rendering a `config.toml` full of keys this binary stopped reading fails
its own CI instead of starting a pod that quietly runs on a compiled default — `serde` ignores an
unknown key by design, which is what makes that failure invisible everywhere else.

The document is [`docs/config.contract.json`](config.contract.json), generated by
[`.github/scripts/config-docs.sh`](../.github/scripts/config-docs.sh) from the same types these
tables come from:

```bash
bash .github/scripts/config-docs.sh
```

It is regenerated and committed by the `Docs` workflow on every pull request, exactly like
`config.example.toml`, and verified wherever that workflow cannot commit — so a renamed key
arrives as a diff in the pull request that renamed it. It covers the `[server]` block alone: the
runtime image runs `/server` and nothing else, and claiming it reads `[converter]` would tell a
validator to accept a table the server drops.

The copy a deployment actually trusts is never this one. It is the document the image build
generates and attaches to its own digest, which is why this file carries neither the commit nor
the build time — only `app.version`, which moves with `Cargo.toml` and is a source change like
any other.

Each released image carries the same document three ways:

| Carrier | What reads it |
|---|---|
| `/config/contract.json` inside the image | an exported tarball, an air-gapped mirror, an in-cluster reader — no registry needed |
| an OCI referrer on the pushed digest, `application/vnd.terrace.config-schema.v1+json` | [`TimSchoenle/helm-charts`](https://github.com/TimSchoenle/helm-charts), which pins that digest |
| the `dev.terrace.config.*` image labels | anything, to discover the other two without pulling a layer |

The three labels — the envelope version, the in-image path, and the loader's `MP_STATS_` prefix —
are constants, so the `Dockerfile` writes them out by hand. Two checks are what make that safe:
the `Config Contract` job diffs the block against `--format dockerfile` and fails on a difference,
and the Docker job checks the labels **of both built images** against the generator's own output,
because a label a source diff can see is not the same thing as a label the image carries.

A pod running this image needs `enableServiceLinks: false`. Kubernetes injects a
`MP_STATS_LEGACY_VIEWER_*` variable per service in the namespace, those names fall inside the
loader's own prefix, and a contract may not exempt its own namespace from the check that owns it.
