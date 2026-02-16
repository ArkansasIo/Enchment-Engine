use rustapi::game_logic::{
    TownGeneratorSettings, generate_starter_rpg_mmorpg_config, generate_town_map,
};

#[test]
fn town_payload_roundtrip_json() {
    let settings = TownGeneratorSettings {
        seed: 999,
        town_name: "Roundtrip".to_string(),
        size: 800,
        rings: 3,
        districts_per_ring: 6,
        has_river: true,
        has_walls: false,
    };
    let data = generate_town_map(&settings);
    let json = serde_json::to_string(&data).unwrap();
    let back: rustapi::game_logic::TownMapData = serde_json::from_str(&json).unwrap();
    assert_eq!(back.seed, data.seed);
    assert_eq!(back.districts.len(), data.districts.len());
}

#[test]
fn mmorpg_payload_can_be_persisted_in_toml() {
    let cfg = generate_starter_rpg_mmorpg_config(42, "PersistWorld".to_string());
    let data = serde_json::to_string_pretty(&cfg).unwrap();

    let mut root = toml::Table::new();
    let mut mmo = toml::Table::new();
    mmo.insert("seed".to_string(), toml::Value::Integer(42));
    mmo.insert("data".to_string(), toml::Value::String(data.clone()));
    root.insert("mmorpg_systems".to_string(), toml::Value::Table(mmo));

    let text = toml::to_string_pretty(&root).unwrap();
    let parsed: toml::Value = toml::from_str(&text).unwrap();
    let data_str = parsed
        .get("mmorpg_systems")
        .and_then(toml::Value::as_table)
        .and_then(|t| t.get("data"))
        .and_then(toml::Value::as_str)
        .unwrap();

    let back: rustapi::game_logic::StarterRpgMmorpgConfig =
        serde_json::from_str(data_str).unwrap();
    assert_eq!(back.seed, cfg.seed);
    assert_eq!(back.default_classes.len(), cfg.default_classes.len());
}
