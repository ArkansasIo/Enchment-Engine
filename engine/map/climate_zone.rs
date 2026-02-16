//! ClimateZone: regional climate classification.

pub struct ClimateZone {
    pub name: String,
    pub zone_type: String,
    pub area: Vec<(i32, i32)>,
}

impl ClimateZone {
    pub fn new(name: &str, zone_type: &str, area: Vec<(i32, i32)>) -> Self {
        Self { name: name.to_string(), zone_type: zone_type.to_string(), area }
    }

    pub fn assign_rectangular_area(name: &str, zone_type: &str, top_left: (i32, i32), bottom_right: (i32, i32)) -> Self {
        let mut area = Vec::new();
        for x in top_left.0..=bottom_right.0 {
            for y in top_left.1..=bottom_right.1 {
                area.push((x, y));
            }
        }
        Self { name: name.to_string(), zone_type: zone_type.to_string(), area }
    }
}
