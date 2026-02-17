// Feature menu structure for Eldiron

pub enum FeatureMenu {
    RPG,
    MMORPG,
    GameLogic,
    Custom(String),
}

pub struct MenuItem {
    pub name: String,
    pub submenus: Vec<MenuItem>,
}

impl MenuItem {
    pub fn new(name: &str) -> Self {
        MenuItem {
            name: name.to_string(),
            submenus: Vec::new(),
        }
    }

    pub fn with_submenus(name: &str, submenus: Vec<MenuItem>) -> Self {
        MenuItem {
            name: name.to_string(),
            submenus,
        }
    }
}

pub fn build_feature_menu() -> Vec<MenuItem> {
    vec![
        MenuItem::with_submenus("RPG Features", vec![
            MenuItem::with_submenus("Quests", vec![
                MenuItem::new("Main Quests"),
                MenuItem::new("Side Quests"),
                MenuItem::new("Quest Chains"),
                MenuItem::new("Quest Rewards"),
            ]),
            MenuItem::with_submenus("Inventory", vec![
                MenuItem::new("Items"),
                MenuItem::new("Equipment"),
                MenuItem::new("Key Items"),
                MenuItem::new("Consumables"),
                MenuItem::new("Crafting Materials"),
                MenuItem::new("Resources"),
            ]),
            MenuItem::with_submenus("Character Progression", vec![
                MenuItem::new("Leveling"),
                MenuItem::new("Skill Trees"),
                MenuItem::new("Classes"),
                MenuItem::new("Subclasses"),
                MenuItem::new("Stats & Attributes"),
                MenuItem::new("Perks"),
                MenuItem::new("Talents"),
            ]),
            MenuItem::with_submenus("Turn-based Combat", vec![
                MenuItem::new("Battle System"),
                MenuItem::new("Enemy Groups"),
                MenuItem::new("Boss Battles"),
                MenuItem::new("Status Effects"),
                MenuItem::new("Loot Tables"),
                MenuItem::new("Combat Log"),
            ]),
            MenuItem::with_submenus("World", vec![
                MenuItem::new("World Map"),
                MenuItem::new("Regions"),
                MenuItem::new("Towns"),
                MenuItem::new("Dungeons"),
                MenuItem::new("Biomes"),
                MenuItem::new("Weather"),
            ]),
        ]),
        MenuItem::with_submenus("MMORPG Features", vec![
            MenuItem::with_submenus("Guilds", vec![
                MenuItem::new("Guild Creation"),
                MenuItem::new("Guild Management"),
                MenuItem::new("Guild Quests"),
                MenuItem::new("Guild Wars"),
                MenuItem::new("Guild Bank"),
                MenuItem::new("Guild Chat"),
            ]),
            MenuItem::with_submenus("Trading", vec![
                MenuItem::new("Player Trading"),
                MenuItem::new("Auction House"),
                MenuItem::new("Marketplaces"),
                MenuItem::new("Bartering"),
            ]),
            MenuItem::with_submenus("PvP Arenas", vec![
                MenuItem::new("1v1 Duels"),
                MenuItem::new("Team Battles"),
                MenuItem::new("Ranked Matches"),
                MenuItem::new("Tournaments"),
            ]),
            MenuItem::with_submenus("World Events", vec![
                MenuItem::new("Seasonal Events"),
                MenuItem::new("Boss Raids"),
                MenuItem::new("Server-wide Quests"),
                MenuItem::new("Live Events"),
            ]),
            MenuItem::with_submenus("Social", vec![
                MenuItem::new("Friends List"),
                MenuItem::new("Chat"),
                MenuItem::new("Mail"),
                MenuItem::new("Groups"),
            ]),
        ]),
        MenuItem::with_submenus("Engine", vec![
            MenuItem::new("Engine Settings"),
            MenuItem::new("Graphics"),
            MenuItem::new("Audio"),
            MenuItem::new("Input"),
            MenuItem::new("Performance"),
            MenuItem::new("Plugins"),
        ]),
        MenuItem::with_submenus("World Tools", vec![
            MenuItem::new("Map Editor"),
            MenuItem::new("Tile Editor"),
            MenuItem::new("Object Placer"),
            MenuItem::new("Event Editor"),
            MenuItem::new("Cutscene Editor"),
        ]),
        MenuItem::with_submenus("Systems", vec![
            MenuItem::new("Save/Load System"),
            MenuItem::new("Quest System"),
            MenuItem::new("Dialogue System"),
            MenuItem::new("Combat System"),
            MenuItem::new("Inventory System"),
            MenuItem::new("AI System"),
        ]),
        MenuItem::with_submenus("Debug", vec![
            MenuItem::new("Console"),
            MenuItem::new("Profiler"),
            MenuItem::new("Log Viewer"),
            MenuItem::new("Hot Reload"),
            MenuItem::new("Test Runner"),
        ]),
        MenuItem::with_submenus("Game Logic", vec![
            MenuItem::with_submenus("Scripting", vec![
                MenuItem::new("Visual Scripting"),
                MenuItem::new("Lua Scripts"),
                MenuItem::new("Event Triggers"),
            ]),
            MenuItem::with_submenus("AI Behaviors", vec![
                MenuItem::new("Enemy AI"),
                MenuItem::new("NPC Schedules"),
                MenuItem::new("Pathfinding"),
            ]),
            MenuItem::with_submenus("Dialogue Trees", vec![
                MenuItem::new("Branching Dialogue"),
                MenuItem::new("Voice Acting"),
                MenuItem::new("Cinematic Events"),
            ]),
        ]),
    ]
}
