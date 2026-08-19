//! Renders the platform's configuration surface — for the documentation job, and for the image.
//!
//! Every table in `docs/CONFIGURATION.md`, the key table in the README, and the whole of
//! `config.example.toml` come out of here, so none of the three can drift from the structs in
//! `src/`: a pull request that adds a key and does not regenerate is a pull request whose
//! documentation job commits the missing row back onto the branch.
//!
//! Driven by `just regenerate`, which is what CI runs and what to run locally. One rendering on
//! its own is `just render <format>`, or by hand:
//!
//! ```text
//! cargo run -p mp-stats-config --features config-schema --example config-schema -- \
//!     --format markdown --only server
//! ```
//!
//! It reads nothing from the environment — the loader is asked what it *would* read, never what
//! is set — so it produces the same answer on a runner as on a developer's machine.
//!
//! # What is left here
//!
//! The `--format` vocabulary, the argument parsing, the dispatch across the renderings and the
//! usage message are [`Cli`](terrace_config::schema::cli::Cli). They were the same two hundred
//! lines in every repository that had a generator, which is how three of them ended up
//! disagreeing about how to cut a `LABEL` block back out of a Dockerfile.
//!
//! What is genuinely this platform's own is below: the two blocks the schema is assembled from,
//! the narrower surface the *image* publishes, the app identity, and the external variables no
//! derive can find.
//!
//! # The image outputs
//!
//! `contract`, `labels` and `dockerfile` describe the *image* rather than a page of
//! documentation, and the container build is what consumes them:
//!
//! ```text
//! config-schema -- --format contract   > /out/contract.json    # COPYed in, and attached to the digest
//! config-schema -- --format labels     > /out/contract.labels  # what CI checks the built image against
//! config-schema -- --format dockerfile                         # the marked LABEL region
//! ```
//!
//! The document and the labels are produced by one run of this generator in one builder stage,
//! which is the only arrangement in which the two cannot disagree. `docs/config.contract.json` is
//! the committed copy `just regenerate` writes and the documentation workflow commits, so a
//! renamed key shows up in the pull request that renamed it rather than in the deployment that
//! breaks on it.

use std::process::ExitCode;

use mp_stats_config::{ConverterConfig, ServerConfig, terrace};
use terrace_config::figment::util::nest;
use terrace_config::figment::value::Value;
use terrace_config::schema::cli::{Cli, Request};
use terrace_config::schema::{App, Describe, External, JsonSchema, Schema};

/// The `$id` the generated JSON Schema carries.
const SCHEMA_ID: &str = "https://github.com/TimSchoenle/mp-stats-legacy-viewer/config.schema.json";

fn main() -> ExitCode {
    let request = match Request::from_env() {
        Ok(request) => request,
        Err(error) => {
            eprintln!("config-schema: {error}");
            return ExitCode::FAILURE;
        }
    };

    // The one thing this generator cannot hand `Cli` a single schema for. A page of
    // documentation describes the configuration *file*, which both binaries read; a contract
    // describes the *image*, whose entry point is `/server` and which reads one of the two
    // blocks. Rendering a contract from the whole surface would publish the claim that this
    // image reads `[converter]`, and a validator believing it would accept a rendered
    // `[converter]` table that the server silently drops — the exact defect the contract exists
    // to catch.
    let schema = if request.format().whole_image() {
        image_schema()
    } else {
        schema()
    };

    match Cli::new(app())
        .json_schema(
            JsonSchema::new()
                .title("mp-stats-legacy-viewer configuration")
                .id(SCHEMA_ID),
        )
        .contract_with(&|builder| builder.external(external()))
        .render(&request, schema)
    {
        // Exactly one trailing newline, whichever rendering this was. Every caller is a shell
        // redirect into a file whose bytes are compared against a committed copy: the Markdown
        // and TOML renderings already end in one and a second would be a trailing blank line the
        // drift check reports forever, while the contract, the labels and the `LABEL` region end
        // without one and a file that does not end in a newline is one that `git diff` and a
        // `while read` loop both complain about.
        Ok(rendered) => {
            if rendered.ends_with('\n') {
                print!("{rendered}");
            } else {
                println!("{rendered}");
            }
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("config-schema: {error}");
            ExitCode::FAILURE
        }
    }
}

/// What this build is, for the contract that names it.
///
/// `--version`, `--revision` and `--created` override what is set here, and are the three things
/// that legitimately differ between builds of one source tree. Omitted — which is how the
/// committed copy is rendered — `--format contract` is byte-reproducible and can be diffed in
/// review.
fn app() -> App {
    // Spelled as the image tag spells it: `CARGO_PKG_VERSION` alone yields `0.16.1` where the
    // release is tagged `v0.16.1`, and this field exists to be compared against a tag.
    App::new("mp-stats-legacy-viewer")
        .version(concat!("v", env!("CARGO_PKG_VERSION")))
        .source("https://github.com/TimSchoenle/mp-stats-legacy-viewer")
}

/// What this image reads that no derive can reach.
///
/// This image declares no variable of its own — the server is one static binary that reads the
/// layered `MP_STATS_` namespace and the loader's own path variables and nothing else — so what
/// is left is the platform's, and `Unknown::Reject` (the default) is what makes leaving one out a
/// failure rather than a silence. `HOSTNAME` comes from the container runtime and `KUBERNETES_*`
/// from the API server; neither has an owner in this image, which is the one case an ignore is
/// for.
///
/// Not declared, and not declarable: the service-link variables Kubernetes injects for every
/// service in the namespace. This service's own are spelled `MP_STATS_LEGACY_VIEWER_*`, which
/// falls inside the loader's prefix, and an ignore pattern reaching into that namespace is
/// refused by `ContractBuilder::build` — exempting a real configuration key from the check that
/// owns it is worse than the collision it would paper over. A pod running this image needs
/// `enableServiceLinks: false`, which is the chart's to set and not something this document can
/// say.
fn external() -> External {
    External::new().ignore("KUBERNETES_*").ignore("HOSTNAME")
}

/// The whole configuration surface, with the defaults an unconfigured process would see.
///
/// Built from the two blocks this crate owns rather than from an aggregate struct: the server
/// binary deserialises `[server]` and the converter `[converter]`, and neither knows about the
/// other's block. `merge` unions them into the one document that describes the file both read,
/// which an aggregate declared for the generator alone could silently stop matching.
fn schema() -> Schema {
    block::<ServerConfig>("server").merge(block::<ConverterConfig>("converter"))
}

/// The surface of the *image*, which is the server block and nothing else.
///
/// The runtime stage is a `scratch` image whose entry point is `/server`, and that binary
/// deserialises one block. `[converter]` is read by a binary that runs during the build and is
/// never copied out of it, so publishing it here would make this document's one claim that must
/// never be wrong — "these are the keys this image reads" — about keys the image ignores.
fn image_schema() -> Schema {
    block::<ServerConfig>("server")
}

/// One block's schema, rooted at the key path the block sits at, carrying its observed defaults.
///
/// `schema_at` is what keeps the paths real. `schema::<ServerConfig>()` would describe
/// `csp.cloudflare.turnstile` — a key that appears in no configuration file anywhere.
fn block<T: Default + serde::Serialize + Describe>(root: &str) -> Schema {
    terrace()
        .schema_at::<T>(root)
        .with_defaults_from_value(&defaults(root, &T::default()))
}

/// One block's defaults, nested under the key path the block sits at.
///
/// The schema looks each default up by full path, so a `[server]` block serialised on its own
/// would answer to `bind_addr` while the schema asks for `server.bind_addr` — and every default
/// would silently come back empty.
fn defaults<T: serde::Serialize>(root: &str, block: &T) -> Value {
    nest(
        root,
        Value::serialize(block).expect("a block of paths and flags serialises"),
    )
}
