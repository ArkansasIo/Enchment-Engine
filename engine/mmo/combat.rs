//! MMO Combat: formulas, balancing

pub fn calculate_damage(attack: i32, defense: i32, skill_multiplier: f32, crit: bool, crit_mult: f32) -> i32 {
    let base = (attack - defense).max(1) as f32 * skill_multiplier;
    if crit {
        (base * crit_mult).round() as i32
    } else {
        base.round() as i32
    }
}

pub fn calculate_crit_chance(dex: i32, base: f32) -> f32 {
    base + (dex as f32 * 0.05)
}

pub fn calculate_heal(power: i32, skill_multiplier: f32) -> i32 {
    (power as f32 * skill_multiplier).round() as i32
}
