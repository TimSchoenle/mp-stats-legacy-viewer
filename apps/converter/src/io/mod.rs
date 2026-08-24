//! Reading the dumps, writing the tree, and the cache that lets a run skip both.
//!
//! Nothing here knows what a leaderboard is. The record layouts are `pipeline`'s.

pub mod cache;
pub mod reader;
pub mod writer;

pub use cache::ConversionCache;
pub use reader::*;
pub use writer::*;
