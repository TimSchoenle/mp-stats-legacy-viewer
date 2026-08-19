//! Renders the platform's configuration surface — for the documentation job, and for the image.
//!
//! Every table in `docs/CONFIGURATION.md`, the key table in the README, and the whole of
//! `config.example.toml` come out of here, so none of the three can drift from the structs in
//! `src/`: a pull request that adds a key and does not regenerate is a pull request whose
//! documentation job commits the missing row back onto the branch.
//!
//! Driven by [`.github/scripts/config-docs.sh`](../../../.github/scripts/config-docs.sh), which
//! is what CI runs and what to run locally:
//!
//! ```text
//! cargo run -p mp-stats-config --features config-schema --example config-schema -- \
//!     --format markdown --only server
//! ```
//!
//! It reads nothing from the environment — the loader is asked what it *would* read, never what
//! is set — so it produces the same answer on a runner as on a developer's machine.
//!
//! # The image outputs
//!
//! `contract`, `labels` and `dockerfile` describe the *image* rather than a page of
//! documentation, and the container build is what consumes them:
//!
//! ```text
//! config-schema -- --format contract   > /out/contract.json    # COPYed in, and attached to the digest
//! config-schema -- --format labels     > /out/contract.labels  # what CI checks the built image against
//! config-schema -- --format dockerfile                         # the LABEL block to paste
//! ```
//!
//! The document and the labels are produced by one run of this generator in one builder stage,
//! which is the only arrangement in which the two cannot disagree. `docs/config.contract.json`
//! is the committed copy the `Config Contract` job regenerates and diffs, so a renamed key shows
//! up in the pull request that renamed it rather than in the deployment that breaks on it.

use std::process::ExitCode;

use mp_stats_config::{ConverterConfig, ServerConfig, terrace};
use terrace_config::figment::util::nest;
use terrace_config::figment::value::Value;
use terrace_config::schema::{App, Column, Contract, DEFAULT_PATH, Describe, External, Schema};

fn main() -> ExitCode {
    let options = match Options::from_args() {
        Ok(options) => options,
        Err(message) => {
            eprintln!("config-schema: {message}");
            return ExitCode::FAILURE;
        }
    };

    match render(&options) {
        // `print!`, not `println!`: every rendering already ends with a newline, and the caller
        // is a shell redirect into a file whose bytes are compared against a committed copy. A
        // second newline would be a trailing blank line that the drift check reports forever.
        Ok(rendered) => {
            print!("{rendered}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("config-schema: {error}");
            ExitCode::FAILURE
        }
    }
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
/// never be wrong — "these are the keys this image reads" — about keys the image ignores. A
/// validator believing it would accept a rendered `[converter]` table that the server silently
/// drops, which is the exact defect the contract exists to catch.
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

fn render(options: &Options) -> Result<String, terrace_config::Error> {
    match options.format {
        // A subsystem table gets the keys alone: the loader variables belong once, above the
        // first of them, rather than repeated over every slice.
        Format::Markdown if !options.only.is_empty() => Ok(schema()
            .subset(&options.only)
            .to_markdown_keys(Column::DEFAULT)),
        Format::Markdown => Ok(schema().subset(&options.only).to_markdown()),
        Format::Loader => Ok(schema().subset(&options.only).to_markdown_loader()),
        Format::Toml => Ok(schema().subset(&options.only).to_toml_example()),
        // The three image renderings, each ending in exactly one newline, so the file a shell
        // redirect writes is one that `git diff` and a `while read` loop both handle.
        Format::Contract => Ok(format!("{}\n", contract(options)?.to_json()?)),
        Format::Labels => Ok(contract(options)?
            .labels(DEFAULT_PATH)
            .into_iter()
            .map(|(name, value)| format!("{name}={value}\n"))
            .collect()),
        // Already ends with a newline and carries no trailing backslash, so the block can be
        // diffed against the Dockerfile's without either side being trimmed first.
        Format::Dockerfile => Ok(contract(options)?.to_dockerfile_labels(DEFAULT_PATH)),
    }
}

/// The whole contract this image publishes: every configuration key, and everything else it reads.
///
/// The `external` half is the part no derive can reach. This image declares no variable of its
/// own there — the server is one static binary that reads the layered `MP_STATS_` namespace and
/// the loader's own path variables and nothing else — so what is left is the platform's, and
/// `Unknown::Reject` (the default) is what makes leaving it out a failure rather than a silence.
/// `HOSTNAME` comes from the container runtime and `KUBERNETES_*` from the API server; neither
/// has an owner in this image, which is the one case an ignore is for.
///
/// Not declared, and not declarable: the service-link variables Kubernetes injects for every
/// service in the namespace. This service's own are spelled `MP_STATS_LEGACY_VIEWER_*`, which
/// falls inside the loader's prefix, and an ignore pattern reaching into that namespace is
/// refused by `ContractBuilder::build` — exempting a real configuration key from the check that
/// owns it is worse than the collision it would paper over. A pod running this image needs
/// `enableServiceLinks: false`, which is the chart's to set and not something this document can
/// say.
fn contract(options: &Options) -> Result<Contract, terrace_config::Error> {
    // Spelled as the image tag spells it: `CARGO_PKG_VERSION` alone yields `0.16.1` where the
    // release is tagged `v0.16.1`, and this field exists to be compared against a tag.
    let mut app = App::new("mp-stats-legacy-viewer")
        .version(concat!("v", env!("CARGO_PKG_VERSION")))
        .source("https://github.com/TimSchoenle/mp-stats-legacy-viewer");

    // The two fields that legitimately differ between builds of one source tree, and the reason
    // they are flags rather than something read here: this generator reads nothing from its
    // environment, so a documentation job and a container build produce the same bytes. Passing
    // them makes that difference explicit and keeps `--format contract` reproducible when they
    // are omitted — which is what lets the committed copy be diffed at all.
    if let Some(revision) = &options.revision {
        app = app.revision(revision);
    }
    if let Some(created) = &options.created {
        app = app.created(created);
    }

    image_schema()
        .into_contract(app)
        .external(External::new().ignore("KUBERNETES_*").ignore("HOSTNAME"))
        .build()
}

/// What to emit, and how much of it.
struct Options {
    format: Format,
    /// The subtree to keep. Empty means the whole configuration.
    only: String,
    /// The commit this build is of, for `--format contract`.
    revision: Option<String>,
    /// When this build happened, RFC 3339, for `--format contract`.
    created: Option<String>,
}

/// Which rendering to emit. One per artefact the documentation job and the image build produce;
/// nothing here is available that nothing consumes.
#[derive(Clone, Copy)]
enum Format {
    /// GitHub-flavoured tables — the loader variables and the keys, or the keys alone under
    /// `--only`.
    Markdown,
    /// The loader-variable table alone, for the page that carries it above every key table.
    Loader,
    /// The commented file an operator copies to `config.toml`.
    Toml,
    /// The document the build embeds in the image and attaches to its digest.
    Contract,
    /// The image labels that make that document discoverable, one `NAME=value` per line.
    Labels,
    /// The same labels as the `LABEL` instruction to paste into the Dockerfile.
    Dockerfile,
}

impl Format {
    /// Whether this rendering describes a whole image rather than a slice of a configuration.
    ///
    /// A contract that quietly omitted the keys `--only` cut would be a contract asserting the
    /// image does not read them, which is the one claim in the document that must never be wrong.
    fn whole_image(self) -> bool {
        matches!(self, Self::Contract | Self::Labels | Self::Dockerfile)
    }
}

impl Options {
    fn from_args() -> Result<Self, String> {
        let mut options = Self {
            format: Format::Markdown,
            only: String::new(),
            revision: None,
            created: None,
        };
        let mut args = std::env::args().skip(1);

        while let Some(flag) = args.next() {
            match flag.as_str() {
                "--format" => {
                    options.format = match args.next().as_deref() {
                        Some("markdown" | "md") => Format::Markdown,
                        Some("loader") => Format::Loader,
                        Some("toml") => Format::Toml,
                        Some("contract") => Format::Contract,
                        Some("labels") => Format::Labels,
                        Some("dockerfile") => Format::Dockerfile,
                        Some(other) => return Err(format!("unknown format `{other}`; {USAGE}")),
                        None => return Err(format!("--format takes a value; {USAGE}")),
                    };
                }
                "--only" => {
                    options.only = args
                        .next()
                        .ok_or_else(|| format!("--only takes a key prefix; {USAGE}"))?;
                }
                "--revision" => {
                    options.revision = Some(
                        args.next()
                            .ok_or_else(|| format!("--revision takes a commit; {USAGE}"))?,
                    );
                }
                "--created" => {
                    options.created = Some(
                        args.next()
                            .ok_or_else(|| format!("--created takes a timestamp; {USAGE}"))?,
                    );
                }
                other => return Err(format!("unknown argument `{other}`; {USAGE}")),
            }
        }

        // Refused rather than silently ignored. A contract is a claim about what a whole image
        // reads, so one built from a slice would assert that the keys `--only` cut do not exist —
        // and a validator believing that rejects a chart which is setting them correctly.
        if options.format.whole_image() && !options.only.is_empty() {
            return Err(format!(
                "--only slices a configuration, and this format describes a whole image; a \
                 contract built from a slice would claim the image does not read the keys it \
                 cut. {USAGE}"
            ));
        }

        Ok(options)
    }
}

const USAGE: &str = "usage: config-schema \
                     [--format markdown|loader|toml|contract|labels|dockerfile] \
                     [--only <key-prefix>] [--revision <commit>] [--created <rfc3339>]";
