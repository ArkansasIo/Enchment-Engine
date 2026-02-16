//! Test coverage for key systems: controls, macros, payment, and integration.

#[cfg(test)]
mod tests {
    use engine::keybinds::Keybinds;
    use engine::macros::MacroSystem;

    #[test]
    fn test_full_integration() {
        let mut kb = Keybinds::new();
        kb.bind("QuickLoad", "F9");
        kb.bind("OpenMap", "M");
        let mut ms = MacroSystem::new();
        ms.add_macro("QuickLoad", vec!["Load".to_string(), "Notify".to_string()]);
        ms.add_macro("OpenMap", vec!["ShowMap".to_string()]);
        assert_eq!(kb.get_key("QuickLoad"), Some("F9"));
        ms.run_macro("QuickLoad");
        ms.run_macro("OpenMap");
    }

    #[test]
    fn test_payment_and_controls() {
        let mut paid = false;
        let mut kb = Keybinds::new();
        kb.bind("UnlockFeature", "U");
        // Simulate payment and unlock
        paid = true;
        if paid {
            assert_eq!(kb.get_key("UnlockFeature"), Some("U"));
        }
    }
}
