//! Preferences for Bluesky application.
use crate::moderation::ModerationPrefs;
use serde::{Deserialize, Serialize};

/// Preferences for Bluesky application.
#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preferences {
    pub moderation_prefs: ModerationPrefs,
}