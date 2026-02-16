//! Game control tests: RPG/MMORPG keyboard layouts and input mapping.

#[cfg(test)]
mod tests {
    use engine::keybinds::Keybinds;

    #[test]
    fn test_rpg_keyboard_layout() {
        let mut kb = Keybinds::new();
        kb.bind("MoveUp", "W");
        kb.bind("MoveDown", "S");
        kb.bind("MoveLeft", "A");
        kb.bind("MoveRight", "D");
        kb.bind("Attack", "Space");
        kb.bind("Inventory", "I");
        assert_eq!(kb.get_key("MoveUp"), Some("W"));
        assert_eq!(kb.get_key("Attack"), Some("Space"));
    }

    #[test]
    fn test_mmorpg_keyboard_layout() {
        let mut kb = Keybinds::new();
        kb.bind("MoveUp", "W");
        kb.bind("MoveDown", "S");
        kb.bind("MoveLeft", "A");
        kb.bind("MoveRight", "D");
        kb.bind("Skill1", "1");
        kb.bind("Skill2", "2");
        kb.bind("Skill3", "3");
        kb.bind("Chat", "Enter");
        assert_eq!(kb.get_key("Skill2"), Some("2"));
        assert_eq!(kb.get_key("Chat"), Some("Enter"));
    }
}
