//! Game dev workflow test: macro and keybind integration.

#[cfg(test)]
mod tests {
    use engine::keybinds::Keybinds;
    use engine::macros::MacroSystem;

    #[test]
    fn test_macro_and_keybind_integration() {
        let mut kb = Keybinds::new();
        kb.bind("QuickSave", "F5");
        let mut ms = MacroSystem::new();
        ms.add_macro("QuickSave", vec!["Save".to_string(), "Notify".to_string()]);
        assert_eq!(kb.get_key("QuickSave"), Some("F5"));
        ms.run_macro("QuickSave");
    }
}
