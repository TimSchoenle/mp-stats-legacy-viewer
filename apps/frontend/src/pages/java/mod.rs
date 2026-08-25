//! The four pages that take an edition, which despite the module name serve Bedrock as well as
//! Java.

pub use self::game::GameView;
pub use self::landing::EditionLanding;
pub use self::player::PlayerView;
pub use leaderboard::LeaderboardView;

pub mod leaderboard;

pub mod game;

pub mod player;

pub mod landing;
