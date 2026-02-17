//! Episode: contains quests and side quests

#[derive(Debug, Clone, Default)]
pub struct Episode {
    pub id: String,
    pub title: String,
    pub description: String,
    pub quests: Vec<Quest>,
    pub side_quests: Vec<SideQuest>,
}

use super::quest::Quest;
use super::side_quest::SideQuest;
