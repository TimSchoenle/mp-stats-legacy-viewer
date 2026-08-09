# Configuration reference

Every key the platform reads, what it does, its default, and which binary consumes it.

Configuration is **file-first**: a TOML file describes a deployment, and the environment is one
override layer above it rather than the only way in. Both binaries load through the same
loader, so one file can describe the whole platform.

---

## 1. How configuration is loaded

Layered, lowest precedence first. The layering is
[`terrace-config`](https://github.com/TimSchoenle/terrace-config); which variable names *this*
deployment spells is [`crates/config/src/loader.rs`](../crates/config/src/loader.rs); the typed
blocks are the rest of [`crates/config`](../crates/config/src/lib.rs).

1. The `#[serde(default)]` value compiled into each field — the table below.
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
separator — the single most common way to get this wrong:

| TOML | Environment |
|---|---|
| `server.bind_addr` | `MP_STATS_SERVER__BIND_ADDR` |
| `server.data_dir` | `MP_STATS_SERVER__DATA_DIR` |
| `converter.cache.dir` | `MP_STATS_CONVERTER__CACHE__DIR` |

An **unknown** `MP_STATS_*` key is ignored, not rejected. That is what lets one file carry both
blocks — the server does not fail on `[converter]` — but it also means a typo fails silently,
as does a key that has been removed (see [§5](#5-removed-keys)).

Nothing here is required: with no file and no variables set, both binaries run on the defaults
below, which match a workspace checkout.

---

## 2. Where the file comes from

| Variable | Default | Meaning |
|---|---|---|
| `MP_STATS_CONFIG` | `./config.toml` | The TOML file, or a directory of `*.toml` fragments. |
| `MP_STATS_SECRETS_DIR` | unset | Directory of one-file-per-key values ([§4](#4-file-backed-layers)). |

Both are read to decide what the other layers *are*, so neither can itself be supplied by a
file; `terrace-config` reserves them and refuses an attempt rather than ignoring it.

The container image ships [`deploy/config.toml`](../deploy/config.toml) at `/config.toml` and
points `MP_STATS_CONFIG` at it, so `docker run` needs no arguments. Replace it with a bind
mount, point `MP_STATS_CONFIG` somewhere else, or override single keys with `MP_STATS_*`.

For a local checkout, copy [`config.example.toml`](../config.example.toml) to `config.toml`
(git-ignored) and edit.

---

## 3. Keys

### `[server]` — consumed by `mp-stats-server`

| Key | Default | Meaning |
|---|---|---|
| `server.bind_addr` | `0.0.0.0:8080` | Address the HTTP listener binds. Parsed as a socket address at load time, so a malformed value fails the boot rather than the bind. |
| `server.dist_dir` | `dist` | Built frontend. The server refuses to start unless it holds an `index.html`, which is both the SPA entry point and the fallback for unknown routes. |
| `server.data_dir` | `data` | The converter's output, served under `/data`. |
| `server.csp.enabled` | `true` | Attach a `Content-Security-Policy` to every document served. The policy itself is derived, not configured ([§6](#6-the-content-security-policy)). |
| `server.csp.cloudflare.script_nonce` | `false` | Reserve a per-response nonce in `script-src` and serve documents `Cache-Control: no-cache`, for the Cloudflare products that inject an inline script at the edge. Carries a deployment obligation ([§6](#6-the-content-security-policy)). |
| `server.csp.cloudflare.turnstile` | `false` | Admit `https://challenges.cloudflare.com` in `script-src` **and** `frame-src`, for a Turnstile widget rendered in a served page. |
| `server.csp.cloudflare.web_analytics` | `false` | Admit the Web Analytics beacon (`https://static.cloudflareinsights.com`) and the endpoint it reports to (`https://cloudflareinsights.com`). Manual snippet only. |

### `[converter]` — consumed by `mp-stats-converter`

| Key | Default | Meaning |
|---|---|---|
| `converter.input_dir` | `data` | Raw per-edition data dumps. Must exist. |
| `converter.output_dir` | `target/converted_data` | Optimized output. Must differ from `converter.input_dir`. |
| `converter.cache.enabled` | `true` | Reuse a previous run's output for editions whose input fingerprint is unchanged. `false` forces a full conversion. |
| `converter.cache.dir` | `target/converter_cache` | Where cached output and its fingerprints live. |

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
using [`csp-shell`](https://github.com/TimSchoenle/csp-shell); those hashes go into `script-src`
alongside the rest of a WebAssembly-SPA policy. There is deliberately no key that spells the
policy out, because a policy written in a config file drifts from the bundle the moment anyone
rebuilds the frontend, and the browser reports that drift by refusing the scripts and rendering
a blank page.

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
