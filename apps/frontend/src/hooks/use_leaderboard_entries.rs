use crate::Api;
use crate::models::LeaderboardEntry;
use mp_stats_core::HistoricalSnapshot;
use mp_stats_core::models::PlatformEdition;
use yew::platform::spawn_local;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct UseLeaderboardEntriesResult {
    pub entries: Vec<LeaderboardEntry>,
    pub loading: bool,
    pub error: Option<String>,
}

#[hook]
pub fn use_leaderboard_entries(
    edition: PlatformEdition,
    game: String,
    board: String,
    stat: String,
    page: u32,
    snapshot: Option<HistoricalSnapshot>,
    is_latest_snapshot: bool,
) -> UseLeaderboardEntriesResult {
    let entries = use_state(|| Vec::<LeaderboardEntry>::new());
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let context = use_context::<Api>().expect("no api context found");

    {
        let entries = entries.clone();
        let loading = loading.clone();
        let error = error.clone();
        let edition = edition.clone();

        use_effect_with(
            (
                snapshot.clone(),
                page,
                board.clone(),
                stat.clone(),
                is_latest_snapshot,
            ),
            move |(snap, page_captured, b, s, is_latest)| {
                error.set(None);

                if let Some(snapshot_data) = snap.as_ref() {
                    let page_idx = page_captured.saturating_sub(1); // 0-based chunk
                    let provider = context.clone();
                    let snapshot_data = snapshot_data.clone();
                    let b = b.clone();
                    let s = s.clone();
                    let is_latest_captured = *is_latest;

                    loading.set(true);
                    spawn_local(async move {
                        let result = if is_latest_captured {
                            provider
                                .fetch_leaderboard(&edition, &b, &game, &s, page_idx)
                                .await
                        } else {
                            provider
                                .fetch_history_leaderboard(
                                    &edition,
                                    &b,
                                    &game,
                                    &s,
                                    &snapshot_data.snapshot_id,
                                    page_idx,
                                )
                                .await
                        };

                        match result {
                            Ok(data) => {
                                entries.set(data);
                                loading.set(false);
                            }
                            Err(e) => {
                                loading.set(false);
                                if e.to_string().contains("404") {
                                    entries.set(vec![]);
                                } else {
                                    error.set(Some(format!("Failed to fetch chunk: {}", e)));
                                }
                            }
                        }
                    });
                }
                || ()
            },
        );
    }

    UseLeaderboardEntriesResult {
        entries: (*entries).clone(),
        loading: *loading,
        error: (*error).clone(),
    }
}
