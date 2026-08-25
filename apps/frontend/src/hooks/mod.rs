//! One hook per page, each turning a fetch into the loading, loaded and failed states a component
//! renders.
//!
//! Components do not fetch. That keeps every network call in [`crate::api`] and every component a
//! pure function of what it was handed.
//!
//! Each of the three fetching hooks is a [`use_resource`](dioxus::prelude::use_resource) over a
//! [`use_reactive!`](macro@dioxus::prelude::use_reactive) dependency list. That list is the part
//! worth reading: a component's arguments are plain values rather than signals, so a resource that
//! does not name them goes on showing the first page it was given after the reader navigates to
//! the second.

pub mod use_game_leaderboards;
pub use use_game_leaderboards::use_game_leaderboards;

pub mod use_leaderboard_entries;
pub use use_leaderboard_entries::use_leaderboard_entries;

pub mod use_player_profile;
pub use use_player_profile::use_player_profile;

pub mod use_theme;

pub use use_theme::use_theme;
