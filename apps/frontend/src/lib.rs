// Re-export core types so that `crate::models` and `crate::data` paths continue to work
// within the frontend crate (pages, components, etc.)
pub use mp_stats_core as data;
pub use mp_stats_core::models;

pub mod components;
pub mod pages;
pub mod route;
pub use route::Route;
pub mod api;
pub mod app;
pub use api::Api;
