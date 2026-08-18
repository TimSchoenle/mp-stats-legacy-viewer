//! Renders the platform's configuration surface for the documentation job.
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

use std::process::ExitCode;

use mp_stats_config::{ConverterConfig, ServerConfig, terrace};
use terrace_config::figment::util::nest;
use terrace_config::figment::value::Value;
use terrace_config::schema::{Column, Schema};

fn main() -> ExitCode {
    let options = match Options::from_args() {
        Ok(options) => options,
        Err(message) => {
            eprintln!("config-schema: {message}");
            return ExitCode::FAILURE;
        }
    };

    // `print!`, not `println!`: every rendering already ends with a newline, and the caller is a
    // shell redirect into a file whose bytes are compared against a committed copy. A second
    // newline would be a trailing blank line that the drift check reports forever.
    print!("{}", render(&options));
    ExitCode::SUCCESS
}

/// The whole configuration surface, with the defaults an unconfigured process would see.
///
/// Built from the two blocks this crate owns rather than from an aggregate struct: the server
/// binary deserialises `[server]` and the converter `[converter]`, and neither knows about the
/// other's block. `merge` unions them into the one document that describes the file both read,
/// which an aggregate declared for the generator alone could silently stop matching.
///
/// `schema_at` is what keeps the paths real. `schema::<ServerConfig>()` would describe
/// `csp.cloudflare.turnstile` — a key that appears in no configuration file anywhere.
fn schema() -> Schema {
    let terrace = terrace();

    terrace
        .schema_at::<ServerConfig>("server")
        .with_defaults_from_value(&defaults("server", &ServerConfig::default()))
        .merge(
            terrace
                .schema_at::<ConverterConfig>("converter")
                .with_defaults_from_value(&defaults("converter", &ConverterConfig::default())),
        )
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

fn render(options: &Options) -> String {
    let schema = schema().subset(&options.only);

    match options.format {
        // A subsystem table gets the keys alone: the loader variables belong once, above the
        // first of them, rather than repeated over every slice.
        Format::Markdown if !options.only.is_empty() => schema.to_markdown_keys(Column::DEFAULT),
        Format::Markdown => schema.to_markdown(),
        Format::Loader => schema.to_markdown_loader(),
        Format::Toml => schema.to_toml_example(),
    }
}

/// What to emit, and how much of it.
struct Options {
    format: Format,
    /// The subtree to keep. Empty means the whole configuration.
    only: String,
}

/// Which rendering to emit. One per artefact the documentation job produces; nothing here is
/// available that nothing consumes.
enum Format {
    /// GitHub-flavoured tables — the loader variables and the keys, or the keys alone under
    /// `--only`.
    Markdown,
    /// The loader-variable table alone, for the page that carries it above every key table.
    Loader,
    /// The commented file an operator copies to `config.toml`.
    Toml,
}

impl Options {
    fn from_args() -> Result<Self, String> {
        let mut options = Self {
            format: Format::Markdown,
            only: String::new(),
        };
        let mut args = std::env::args().skip(1);

        while let Some(flag) = args.next() {
            match flag.as_str() {
                "--format" => {
                    options.format = match args.next().as_deref() {
                        Some("markdown" | "md") => Format::Markdown,
                        Some("loader") => Format::Loader,
                        Some("toml") => Format::Toml,
                        Some(other) => return Err(format!("unknown format `{other}`; {USAGE}")),
                        None => return Err(format!("--format takes a value; {USAGE}")),
                    };
                }
                "--only" => {
                    options.only = args
                        .next()
                        .ok_or_else(|| format!("--only takes a key prefix; {USAGE}"))?;
                }
                other => return Err(format!("unknown argument `{other}`; {USAGE}")),
            }
        }

        Ok(options)
    }
}

const USAGE: &str = "usage: config-schema [--format markdown|loader|toml] [--only <key-prefix>]";
