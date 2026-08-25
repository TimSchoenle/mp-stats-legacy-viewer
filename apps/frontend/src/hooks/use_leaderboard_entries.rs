//! The leaderboard page's data: one page of one board, current or archived.

use crate::Api;
use crate::models::LeaderboardEntry;
use dioxus::prelude::*;
use mp_stats_core::HistoricalSnapshot;
use mp_stats_core::models::PlatformEdition;

/// What a leaderboard page has to render from.
#[derive(Clone, PartialEq)]
pub struct UseLeaderboardEntriesResult {
    /// The rows, in rank order. Empty while loading, and also for a page past the end of the
    /// board, which is how the table draws its own empty state.
    pub entries: Vec<LeaderboardEntry>,
    /// A fetch is in flight.
    pub loading: bool,
    /// Why the page could not be loaded. A page that simply does not exist is not an error here.
    pub error: Option<String>,
}

/// Fetches one page of a board, from the current standings or from an archived snapshot.
///
/// `page` is 1-based and becomes the zero-based chunk number. Nothing is fetched until `snapshot`
/// is known, since the game metadata that names the snapshots arrives first; until then this
/// reports itself as loading, which is what it is.
///
/// Every argument is a dependency, so a board tab, a page button and the snapshot timeline all
/// refetch through the same path.
#[must_use]
pub fn use_leaderboard_entries(
    edition: PlatformEdition,
    game: String,
    board: String,
    stat: String,
    page: u32,
    snapshot: Option<HistoricalSnapshot>,
    is_latest_snapshot: bool,
) -> UseLeaderboardEntriesResult {
    let api = use_context::<Api>();

    let resource = use_resource(use_reactive!(
        |edition, game, board, stat, page, snapshot, is_latest_snapshot| {
            let api = api.clone();

            async move {
                // The outer `None` is "not asked yet" rather than "asked and got nothing": the
                // snapshot rides in on the game metadata, and until that lands there is no file
                // name to fetch.
                let snapshot = snapshot?;

                // The site's page 1 is the tree's chunk 0.
                let chunk = page.saturating_sub(1);

                let fetched = if is_latest_snapshot {
                    api.fetch_leaderboard(&edition, &board, &game, &stat, chunk)
                        .await
                } else {
                    api.fetch_history_leaderboard(
                        &edition,
                        &board,
                        &game,
                        &stat,
                        &snapshot.snapshot_id,
                        chunk,
                    )
                    .await
                };

                Some(match fetched {
                    Ok(entries) => Ok(entries),
                    // A page past the end of the board is a missing file, and a missing file is the
                    // ordinary way a board ends. It draws the table's empty state rather than a
                    // failure banner.
                    Err(error) if error.to_string().contains("404") => Ok(Vec::new()),
                    Err(error) => Err(format!("Failed to fetch chunk: {error}")),
                })
            }
        }
    ));

    let pending = matches!(resource.state().cloned(), UseResourceState::Pending);
    let fetched = resource.value().cloned().flatten();

    UseLeaderboardEntriesResult {
        entries: match &fetched {
            Some(Ok(entries)) => entries.clone(),
            Some(Err(_)) | None => Vec::new(),
        },
        // Also loading while the resource has resolved to "no snapshot yet": the page has no rows
        // and no reason to say so, which is the same thing a fetch in flight means.
        loading: pending || fetched.is_none(),
        error: match fetched {
            Some(Err(error)) => Some(error),
            Some(Ok(_)) | None => None,
        },
    }
}
