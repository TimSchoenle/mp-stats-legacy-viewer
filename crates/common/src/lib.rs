//! The container every file in the converted tree is wrapped in, and the arithmetic that decides
//! which file a lookup lands in.
//!
//! One crate rather than two halves because the frontend links it as well: the browser
//! decompresses through [`compression::uncompress_lzma`] exactly what the converter compressed
//! through [`compression::write_lzma_bin`], so the two cannot drift into disagreeing about the
//! container. Everything here compiles for `wasm32-unknown-unknown`, which is the constraint that
//! keeps the record types in `mp-stats-core` and the paths to them out of this crate entirely.
//!
//! The file-backed halves of [`compression`] are the converter's alone. `std::fs` exists on
//! `wasm32-unknown-unknown` and fails at every call, so calling one from the client compiles and
//! then does not work.

pub mod compression;
pub mod error;
pub mod formats;
pub mod shard;

pub use error::{DataError, Result};
