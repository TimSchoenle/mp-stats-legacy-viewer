//! The Yew client, which is where the querying happens.
//!
//! There is no service to ask. A page assembles itself out of files fetched from the converted
//! tree under `/data`, decompressed and decoded in the browser, so a route resolves to a set of
//! file paths rather than to a request. That tree is documented at the root of [`mp_stats_core`],
//! and [`mp_stats_core::routes`] is where every path into it comes from.
//!
//! The shape of this crate follows from that. [`api`] is the only module that fetches anything,
//! and it holds a short-lived cache in front of the network so that paging back and forth over a
//! board does not refetch what it just had. [`hooks`] turns a fetch into the loading, loaded and
//! failed states a component can render, which is why no component fetches. [`components`] and
//! [`pages`] are the markup, [`route`] is the URL, and [`util`] is formatting.

// Re-export core types so that `crate::models` and `crate::data` paths continue to work
// within the frontend crate (pages, components, etc.)
pub use mp_stats_core as data;
pub use mp_stats_core::models;

pub mod components;
pub mod hooks;
pub mod pages;
pub mod route;
pub use route::Route;
pub mod api;
pub mod app;
pub mod util;

pub use api::Api;
