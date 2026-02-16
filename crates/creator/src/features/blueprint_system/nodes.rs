//! Blueprint Node Definitions

#[derive(Debug, Clone)]
pub enum BlueprintNode {
    // Core logic
    Event(String),
    Function(String),
    Variable(String),
    Branch,
    Sequence,
    // RPG/MMORPG features
    Quest { name: String },
    Inventory,
    Dialogue { npc: String },
    Combat,
    Guild,
    Trade,
    // MMO-specific
    Party,
    PvP,
    WorldEvent(String),
    // Endgame & Encounter Systems
    Trial { name: String },
    Dungeon { name: String },
    Raid { name: String },
    Tower { name: String },
    GroupFinder,
    EventNode { name: String },
    WorldBoss { name: String },
    Boss { name: String },
    Mob { name: String },
    Nemesis { name: String },
    // Custom/Extensible
    Custom(String),
}

impl BlueprintNode {
    pub fn display_name(&self) -> String {
        match self {
            BlueprintNode::Event(name) => format!("Event: {}", name),
            BlueprintNode::Function(name) => format!("Function: {}", name),
            BlueprintNode::Variable(name) => format!("Variable: {}", name),
            BlueprintNode::Branch => "Branch".to_string(),
            BlueprintNode::Sequence => "Sequence".to_string(),
            BlueprintNode::Quest { name } => format!("Quest: {}", name),
            BlueprintNode::Inventory => "Inventory".to_string(),
            BlueprintNode::Dialogue { npc } => format!("Dialogue: {}", npc),
            BlueprintNode::Combat => "Combat".to_string(),
            BlueprintNode::Guild => "Guild".to_string(),
            BlueprintNode::Trade => "Trade".to_string(),
            BlueprintNode::Party => "Party".to_string(),
            BlueprintNode::PvP => "PvP".to_string(),
            BlueprintNode::WorldEvent(name) => format!("World Event: {}", name),
            BlueprintNode::Trial { name } => format!("Trial: {}", name),
            BlueprintNode::Dungeon { name } => format!("Dungeon: {}", name),
            BlueprintNode::Raid { name } => format!("Raid: {}", name),
            BlueprintNode::Tower { name } => format!("Tower: {}", name),
            BlueprintNode::GroupFinder => "Group Finder".to_string(),
            BlueprintNode::EventNode { name } => format!("Event: {}", name),
            BlueprintNode::WorldBoss { name } => format!("World Boss: {}", name),
            BlueprintNode::Boss { name } => format!("Boss: {}", name),
            BlueprintNode::Mob { name } => format!("Mob: {}", name),
            BlueprintNode::Nemesis { name } => format!("Nemesis: {}", name),
            BlueprintNode::Custom(name) => format!("Custom: {}", name),
        }
    }
}
