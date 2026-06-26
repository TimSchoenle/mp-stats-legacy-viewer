use crate::Route;
use crate::components::error_message::ErrorMessage;
use crate::hooks::{use_player_profile, use_theme};
use crate::util::score_formatter::create_score_formatter;
use mp_stats_core::models::PlatformEdition;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct PlayerProps {
    pub edition: PlatformEdition,
    pub uuid: String,
}

#[function_component(PlayerView)]
pub fn player_view(props: &PlayerProps) -> Html {
    let profile_req = use_player_profile(props.edition.clone(), props.uuid.clone());
    let theme_color = use_theme();

    html!()
}
