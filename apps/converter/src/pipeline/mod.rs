//! The four steps of a conversion, in the order [`crate::Converter::convert`] runs them.
//!
//! Each is a free function over paths rather than a method, so a step can be run on its own
//! against a fixture. What forces their order is documented at the crate root.

pub mod games;
pub mod leaderboards;
pub mod metadata;
pub mod players;

pub use games::process_game_metadata;
pub use leaderboards::process_java_leaderboards;
pub use metadata::{build_names_archive, process_dictionary_and_names};
pub use players::process_java_players;
