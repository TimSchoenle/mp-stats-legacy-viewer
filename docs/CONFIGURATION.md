<!--
Generated from .github/templates/CONFIGURATION.md.hbs - edit that file, not docs/CONFIGURATION.md.
The prose is here; every table below is a partial written by `just regenerate` straight out of
the structs in crates/config, so no key, default or environment spelling in this page is
maintained by hand.
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
`v0.9.1`; which variable names *this* deployment spells is
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

### `[telemetry]` — consumed by `mp-stats-server`

Outside `[server]` because none of it is a property of the HTTP listener: a log filter and an
error reporter belong to the process. [§7](#7-logging-and-error-reporting) is what these keys
mean in practice.

| TOML | Type | Environment | Default | Flags | Purpose |
|---|---|---|---|---|---|
| `telemetry.log_filter` | `String` | `MP_STATS_TELEMETRY__LOG_FILTER` | `info` | — | `RUST_LOG`-style filter deciding which records are emitted at all, for example `info,mp_stats_server=debug`. |
| `telemetry.json_logs` | `bool` | `MP_STATS_TELEMETRY__JSON_LOGS` | `false` | — | Emit one JSON object per record instead of human-readable lines. |
| `telemetry.sentry.enabled` | `bool` | `MP_STATS_TELEMETRY__SENTRY__ENABLED` | `false` | — | Initialise the Sentry client. `false` installs no client, no panic hook, no `tracing` layer and no HTTP middleware, so every other key here is inert and nothing leaves the process. |
| `telemetry.sentry.dsn` | `SecretString` | `MP_STATS_TELEMETRY__SENTRY__DSN` | unset | secret | Ingest URL, `https://<key>@<host>/<project>`. |
| `telemetry.sentry.environment` | `String` | `MP_STATS_TELEMETRY__SENTRY__ENVIRONMENT` | unset | — | Environment tag on every event. |
| `telemetry.sentry.release` | `String` | `MP_STATS_TELEMETRY__SENTRY__RELEASE` | unset | — | Release tag on every event. |
| `telemetry.sentry.server_name` | `String` | `MP_STATS_TELEMETRY__SENTRY__SERVER_NAME` | unset | — | Host tag on every event. |
| `telemetry.sentry.sample_rate` | `f32` | `MP_STATS_TELEMETRY__SENTRY__SAMPLE_RATE` | `1` | — | Fraction of captured events actually sent, `0.0`–`1.0`. |
| `telemetry.sentry.traces_sample_rate` | `f32` | `MP_STATS_TELEMETRY__SENTRY__TRACES_SAMPLE_RATE` | `0` | — | Fraction of traces this process **starts** that are recorded, `0.0`–`1.0`. |
| `telemetry.sentry.capture_level` | `SentryLevel`: `off` \| `error` \| `warn` \| `info` \| `debug` \| `trace` | `MP_STATS_TELEMETRY__SENTRY__CAPTURE_LEVEL` | `error` | — | Least severe `tracing` level reported as a Sentry **issue**. |
| `telemetry.sentry.breadcrumb_level` | `SentryLevel`: `off` \| `error` \| `warn` \| `info` \| `debug` \| `trace` | `MP_STATS_TELEMETRY__SENTRY__BREADCRUMB_LEVEL` | `info` | — | Least severe `tracing` level kept as a **breadcrumb** — the trail attached to the next issue. |
| `telemetry.sentry.max_breadcrumbs` | `usize` | `MP_STATS_TELEMETRY__SENTRY__MAX_BREADCRUMBS` | `100` | — | How many breadcrumbs one event carries. |
| `telemetry.sentry.attach_stacktraces` | `bool` | `MP_STATS_TELEMETRY__SENTRY__ATTACH_STACKTRACES` | `true` | — | Attach a stack trace to events that carry none of their own. |
| `telemetry.sentry.send_default_pii` | `bool` | `MP_STATS_TELEMETRY__SENTRY__SEND_DEFAULT_PII` | `false` | — | Send personally identifying data with every event: the client IP, the full request header set (`Cookie` included) and the resolved user. |
| `telemetry.sentry.http_transactions` | `bool` | `MP_STATS_TELEMETRY__SENTRY__HTTP_TRANSACTIONS` | `true` | — | Record request spans: one Sentry transaction per request, named by the *matched route* rather than the URI, so a `/data` path does not become its own transaction name. |
| `telemetry.sentry.span_attributes` | `bool` | `MP_STATS_TELEMETRY__SENTRY__SPAN_ATTRIBUTES` | `false` | — | Copy `tracing` span fields onto the Sentry span as attributes. |
| `telemetry.sentry.shutdown_timeout_secs` | `u64` | `MP_STATS_TELEMETRY__SENTRY__SHUTDOWN_TIMEOUT_SECS` | `2` | — | How long process exit waits for queued events to drain. |
| `telemetry.sentry.debug` | `bool` | `MP_STATS_TELEMETRY__SENTRY__DEBUG` | `false` | — | Print the SDK's own diagnostics to stderr. For proving a DSN works, not for running. |

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

The platform reads exactly one secret: `telemetry.sentry.dsn`, whose URL embeds a bearer
credential for a Sentry project's ingest endpoint. It is held as a `SecretString`, so the block
it sits in does not print it when it is logged, and it belongs in one of the two layers below
rather than in a `config.toml` that is usually committed. No database, no tokens, nothing else.

* **Secrets directory.** Every file in `$MP_STATS_SECRETS_DIR` supplies the key its *name*
  spells, in the same `__`-nested, case-folded spelling minus the prefix:
  `/run/secrets/telemetry__sentry__dsn` supplies `telemetry.sentry.dsn`. A `.` in a file name is
  refused rather than treated as a separator. The provider follows the `..data` symlink a
  Kubernetes projected volume uses and skips the dot-prefixed entries, so it works against a real
  mounted `Secret`. A trailing newline is not part of the value.
* **`_FILE` indirection.** `MP_STATS_TELEMETRY__SENTRY__DSN_FILE=/path` reads the value from that
  path — the Docker convention.

Both work for every key, not only the secret one; `MP_STATS_SERVER__DATA_DIR_FILE` is as valid as
the example above.

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

## 7. Logging and error reporting

Everything the server says about itself goes through `tracing`. `[telemetry]` decides what it
writes and, optionally, who else receives it.

### The log stream

`telemetry.log_filter` is a `RUST_LOG`-style directive list and defaults to `info`. Set
`telemetry.json_logs = true` for one JSON object per record, which is what a cluster shipping
logs to a collector wants; left off, records are human-readable lines, which is what `docker run`
on a terminal wants.

`RUST_LOG` **outranks the key** when it is set. It is the one place in this configuration where a
bare environment variable wins over the layered loader, and it is deliberate: it is the variable
every Rust operator already reaches for while a container is misbehaving. A `RUST_LOG` that does
not parse fails the boot rather than being silently ignored — the alternative is an operator
reading the configured filter's output and concluding their directive did nothing.

### Sentry

Off by default and off in the image's own `config.toml`. A DSN is an egress destination for
whatever a log line happens to carry, so switching it on is a decision made once per deployment:

```toml
[telemetry.sentry]
enabled = true
traces_sample_rate = 0.1
```

The DSN itself belongs in one of the file-backed layers ([§4](#4-file-backed-layers)) rather than
in this file, which is usually committed:

```bash
MP_STATS_TELEMETRY__SENTRY__DSN_FILE=/run/secrets/sentry-dsn
```

`enabled = true` **without a usable DSN fails the boot**, and so does a DSN that does not parse or
a sample rate outside `0.0`–`1.0`. The alternative is a server that starts, serves, and reports
into nothing — a state that is indistinguishable from a service with nothing to report, and that
is noticed during the incident it was meant to surface. The error names the key and never quotes
the DSN back, which embeds a credential.

What a bound client attaches to:

* **Records.** `capture_level` (default `error`) is the least severe level reported as an issue;
  `breadcrumb_level` (default `info`) the least severe kept as the trail attached to the next
  issue. Both sit *under* `log_filter`, which is the surprise worth knowing: tightening the log
  filter to `warn` removes every `info` breadcrumb as well.
* **Panics.** The SDK's hook. Worth having precisely because `axum` absorbs a panicking handler by
  dropping the connection, which leaves nothing behind in an image nobody is attached to.
* **Requests.** One hub per request, so breadcrumbs from concurrently served requests do not land
  on each other, and — while `http_transactions` is on — one transaction per *matched route*, so
  the whole of `/data` is one transaction name rather than one per blob. Whether a started
  transaction is kept is `traces_sample_rate`, which is `0.0` by default: a static file server
  starts no trace of its own, and still continues one it is handed.

`send_default_pii` is off and worth leaving off. On, events carry the client IP and the full
request header set, none of which a crash report needs in order to be actionable.

### Two switches, not one

The keys above are compiled in under the `mp-stats-server` crate's `sentry` feature, which is on
by default and is what the published image is built with. A binary built with
`--no-default-features` — worth roughly two megabytes of transport and TLS — still reads the block
and **refuses to start while `telemetry.sentry.enabled` is set**, for the same reason a missing
DSN does.

One known limit of the release image: it is stripped and UPX-compressed, so the frames in a
reported stack trace are addresses rather than symbols unless debug files for that build are
uploaded to the Sentry project separately. The issue, its breadcrumbs, its tags and its request
metadata are unaffected.

### Shutdown

The server stops on `SIGTERM` (and Ctrl-C), finishes what is in flight, and then flushes whatever
Sentry has queued, bounded by `shutdown_timeout_secs`. A `SIGKILL` takes the queue with it —
which is what makes the orchestrator's grace period the real bound on how much of a crashing
replica's last minute survives.

---

## 8. Examples

A full local override:

```toml
# config.toml
[server]
bind_addr = "127.0.0.1:3000"
dist_dir = "apps/frontend/dist"
data_dir = "target/converted_data"

[telemetry]
log_filter = "info,mp_stats_server=debug"

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

## 9. The contract the image publishes

Everything above is for a human. The same surface is published in a form a deployment pipeline
reads, so that a chart rendering a `config.toml` full of keys this binary stopped reading fails
its own CI instead of starting a pod that quietly runs on a compiled default — `serde` ignores an
unknown key by design, which is what makes that failure invisible everywhere else.

The document is [`docs/config.contract.json`](config.contract.json), generated by
[`just regenerate`](../justfile) from the same types these tables come from:

```bash
just regenerate
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
