use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;
use std::sync::{LazyLock, RwLock};
use std::{collections::HashMap, fmt::Write as _};

pub mod rpg_mmorpg;
pub use rpg_mmorpg::*;

static TOWN_GEN_CACHE: LazyLock<RwLock<HashMap<String, TownMapData>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WardType {
    Administration,
    Castle,
    Cathedral,
    Market,
    Merchant,
    Craftsmen,
    Common,
    Military,
    Slum,
    Park,
    Farm,
    Gate,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TownGeneratorSettings {
    pub seed: u64,
    pub town_name: String,
    pub size: u32,
    pub rings: u32,
    pub districts_per_ring: u32,
    pub has_river: bool,
    pub has_walls: bool,
}

impl Default for TownGeneratorSettings {
    fn default() -> Self {
        Self {
            seed: 1,
            town_name: "New Procedural Town".to_string(),
            size: 1024,
            rings: 4,
            districts_per_ring: 8,
            has_river: true,
            has_walls: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DistrictNode {
    pub id: usize,
    pub ward: WardType,
    pub center: (f32, f32),
    pub radius: f32,
    pub population: u32,
    pub wealth: f32,
    pub danger: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Road {
    pub from: usize,
    pub to: usize,
    pub primary: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Landmark {
    pub name: String,
    pub district_id: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TownMapData {
    pub seed: u64,
    pub town_name: String,
    pub size: u32,
    pub has_river: bool,
    pub has_walls: bool,
    pub districts: Vec<DistrictNode>,
    pub roads: Vec<Road>,
    pub landmarks: Vec<Landmark>,
    pub gates: Vec<usize>,
}

pub fn generate_town_map(settings: &TownGeneratorSettings) -> TownMapData {
    let key = town_settings_cache_key(settings);
    if let Ok(cache) = TOWN_GEN_CACHE.read()
        && let Some(hit) = cache.get(&key)
    {
        return hit.clone();
    }

    let mut rng = StdRng::seed_from_u64(settings.seed);

    let mut districts = Vec::new();
    let mut id = 0usize;
    let center = settings.size as f32 * 0.5;
    let ring_step = center / (settings.rings.max(1) as f32 + 1.0);

    for ring in 0..settings.rings {
        let ring_radius = ring_step * (ring as f32 + 1.0);
        let count = settings.districts_per_ring + ring * 2;

        for i in 0..count {
            let angle = (i as f32 / count as f32) * TAU + rng.random_range(-0.12f32..0.12f32);
            let radial_jitter = rng.random_range(-ring_step * 0.25..ring_step * 0.25);
            let r = (ring_radius + radial_jitter).max(ring_step * 0.5);

            let x = center + angle.cos() * r;
            let y = center + angle.sin() * r;

            let ward = choose_ward_for_ring(ring, settings.rings, &mut rng);
            let population = estimate_population(&ward, ring, settings.rings, &mut rng);
            let wealth = estimate_wealth(&ward, &mut rng);
            let danger = estimate_danger(&ward, &mut rng);

            districts.push(DistrictNode {
                id,
                ward,
                center: (x, y),
                radius: (ring_step * 0.55).max(24.0),
                population,
                wealth,
                danger,
            });
            id += 1;
        }
    }

    let roads = build_road_graph(&districts, &mut rng);
    let gates = compute_gates(&districts, settings.has_walls);
    let landmarks = compute_landmarks(&districts);

    let result = TownMapData {
        seed: settings.seed,
        town_name: settings.town_name.clone(),
        size: settings.size,
        has_river: settings.has_river,
        has_walls: settings.has_walls,
        districts,
        roads,
        landmarks,
        gates,
    };

    if let Ok(mut cache) = TOWN_GEN_CACHE.write() {
        if cache.len() > 128 {
            cache.clear();
        }
        cache.insert(key, result.clone());
    }

    result
}

fn town_settings_cache_key(settings: &TownGeneratorSettings) -> String {
    let mut s = String::new();
    let _ = write!(
        s,
        "{}:{}:{}:{}:{}:{}:{}",
        settings.seed,
        settings.town_name,
        settings.size,
        settings.rings,
        settings.districts_per_ring,
        settings.has_river,
        settings.has_walls
    );
    s
}

fn choose_ward_for_ring(ring: u32, rings: u32, rng: &mut StdRng) -> WardType {
    if ring == 0 {
        return match rng.random_range(0..4) {
            0 => WardType::Castle,
            1 => WardType::Administration,
            2 => WardType::Market,
            _ => WardType::Cathedral,
        };
    }

    if ring + 1 >= rings {
        return match rng.random_range(0..5) {
            0 => WardType::Farm,
            1 => WardType::Gate,
            2 => WardType::Slum,
            3 => WardType::Common,
            _ => WardType::Park,
        };
    }

    match rng.random_range(0..8) {
        0 => WardType::Market,
        1 => WardType::Merchant,
        2 => WardType::Craftsmen,
        3 => WardType::Common,
        4 => WardType::Military,
        5 => WardType::Park,
        6 => WardType::Cathedral,
        _ => WardType::Slum,
    }
}

fn estimate_population(ward: &WardType, ring: u32, rings: u32, rng: &mut StdRng) -> u32 {
    let density = match ward {
        WardType::Castle => 350,
        WardType::Administration => 700,
        WardType::Cathedral => 500,
        WardType::Market => 1200,
        WardType::Merchant => 900,
        WardType::Craftsmen => 1100,
        WardType::Common => 1400,
        WardType::Military => 800,
        WardType::Slum => 1800,
        WardType::Park => 120,
        WardType::Farm => 220,
        WardType::Gate => 450,
    };

    let ring_factor = 1.0 + (ring as f32 / (rings.max(1) as f32)) * 0.6;
    (density as f32 * ring_factor + rng.random_range(-80.0..140.0)).max(50.0) as u32
}

fn estimate_wealth(ward: &WardType, rng: &mut StdRng) -> f32 {
    let base = match ward {
        WardType::Castle => 0.95,
        WardType::Administration => 0.85,
        WardType::Cathedral => 0.75,
        WardType::Market => 0.78,
        WardType::Merchant => 0.83,
        WardType::Craftsmen => 0.62,
        WardType::Common => 0.45,
        WardType::Military => 0.58,
        WardType::Slum => 0.18,
        WardType::Park => 0.70,
        WardType::Farm => 0.35,
        WardType::Gate => 0.42,
    };
    (base + rng.random_range(-0.08f32..0.08f32)).clamp(0.0f32, 1.0f32)
}

fn estimate_danger(ward: &WardType, rng: &mut StdRng) -> f32 {
    let base = match ward {
        WardType::Castle => 0.10,
        WardType::Administration => 0.12,
        WardType::Cathedral => 0.15,
        WardType::Market => 0.35,
        WardType::Merchant => 0.22,
        WardType::Craftsmen => 0.28,
        WardType::Common => 0.40,
        WardType::Military => 0.20,
        WardType::Slum => 0.82,
        WardType::Park => 0.12,
        WardType::Farm => 0.24,
        WardType::Gate => 0.45,
    };
    (base + rng.random_range(-0.07f32..0.07f32)).clamp(0.0f32, 1.0f32)
}

fn build_road_graph(districts: &[DistrictNode], rng: &mut StdRng) -> Vec<Road> {
    let mut roads = Vec::new();
    if districts.is_empty() {
        return roads;
    }

    for district in districts {
        let mut nearest: Vec<(usize, f32)> = districts
            .iter()
            .filter(|other| other.id != district.id)
            .map(|other| {
                let dx = other.center.0 - district.center.0;
                let dy = other.center.1 - district.center.1;
                (other.id, dx * dx + dy * dy)
            })
            .collect();
        nearest.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let links = 2 + rng.random_range(0..=1);
        for (to, _) in nearest.into_iter().take(links) {
            let (from, to_sorted) = if district.id < to {
                (district.id, to)
            } else {
                (to, district.id)
            };
            if !roads.iter().any(|r| r.from == from && r.to == to_sorted) {
                roads.push(Road {
                    from,
                    to: to_sorted,
                    primary: rng.random_bool(0.35),
                });
            }
        }
    }
    roads
}

fn compute_gates(districts: &[DistrictNode], has_walls: bool) -> Vec<usize> {
    if !has_walls || districts.is_empty() {
        return Vec::new();
    }

    let mut sorted: Vec<&DistrictNode> = districts.iter().collect();
    sorted.sort_by(|a, b| {
        let da = a.center.0 * a.center.0 + a.center.1 * a.center.1;
        let db = b.center.0 * b.center.0 + b.center.1 * b.center.1;
        db.partial_cmp(&da).unwrap_or(std::cmp::Ordering::Equal)
    });
    sorted.into_iter().take(4).map(|d| d.id).collect()
}

fn compute_landmarks(districts: &[DistrictNode]) -> Vec<Landmark> {
    let mut landmarks = Vec::new();

    for d in districts {
        match d.ward {
            WardType::Castle => landmarks.push(Landmark {
                name: "Keep".to_string(),
                district_id: d.id,
            }),
            WardType::Cathedral => landmarks.push(Landmark {
                name: "Grand Cathedral".to_string(),
                district_id: d.id,
            }),
            WardType::Market => landmarks.push(Landmark {
                name: "Great Market".to_string(),
                district_id: d.id,
            }),
            WardType::Military => landmarks.push(Landmark {
                name: "Barracks".to_string(),
                district_id: d.id,
            }),
            WardType::Gate => landmarks.push(Landmark {
                name: "City Gate".to_string(),
                district_id: d.id,
            }),
            _ => {}
        }
    }

    landmarks
}

/// Handles player input and actions. Extend with real input handling.
pub fn handle_player_input() {
    println!("[GameLogic] Handling player input. (Stub)");
}

/// Updates NPCs and their AI. Extend with real AI logic.
pub fn update_npcs() {
    println!("[GameLogic] Updating NPCs. (Stub)");
}

/// Saves the current game state. Extend with real save logic.
pub fn save_game() {
    println!("[GameLogic] Saving game. (Stub)");
}

/// Loads a saved game state. Extend with real load logic.
pub fn load_game() {
    println!("[GameLogic] Loading game. (Stub)");
}

/// Example game logic stub.
pub fn example_game_logic() {
    println!("[GameLogic] Example game logic called. (Stub)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn town_generation_is_deterministic_for_same_settings() {
        let settings = TownGeneratorSettings {
            seed: 12345,
            town_name: "Determinism".to_string(),
            size: 900,
            rings: 4,
            districts_per_ring: 7,
            has_river: true,
            has_walls: true,
        };
        let a = generate_town_map(&settings);
        let b = generate_town_map(&settings);
        let sa = serde_json::to_string(&a).unwrap();
        let sb = serde_json::to_string(&b).unwrap();
        assert_eq!(sa, sb);
    }

    #[test]
    fn town_generation_varies_for_different_seed() {
        let mut s1 = TownGeneratorSettings::default();
        s1.seed = 111;
        let mut s2 = TownGeneratorSettings::default();
        s2.seed = 222;
        let a = generate_town_map(&s1);
        let b = generate_town_map(&s2);
        assert_ne!(a.seed, b.seed);
        assert_ne!(a.districts[0].center, b.districts[0].center);
    }
}
