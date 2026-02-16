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
            MenuItem::new("Quests"),
            MenuItem::new("Inventory"),
            MenuItem::new("Character Progression"),
            MenuItem::new("Turn-based Combat"),
        ]),
        MenuItem::with_submenus("MMORPG Features", vec![
            MenuItem::new("Guilds"),
            MenuItem::new("Trading"),
            MenuItem::new("PvP Arenas"),
            MenuItem::new("World Events"),
        ]),
        MenuItem::with_submenus("Game Logic", vec![
            MenuItem::new("Scripting"),
            MenuItem::new("AI Behaviors"),
            MenuItem::new("Dialogue Trees"),
        ]),
    ]
}
