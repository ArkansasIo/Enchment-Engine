//! Act: contains chapters

#[derive(Debug, Clone, Default)]
pub struct Act {
    pub id: String,
    pub title: String,
    pub description: String,
    pub chapters: Vec<Chapter>,
}

use super::chapter::Chapter;
