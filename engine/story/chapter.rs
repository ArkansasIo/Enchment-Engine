//! Chapter: contains episodes

#[derive(Debug, Clone, Default)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub description: String,
    pub episodes: Vec<Episode>,
}

use super::episode::Episode;
