//! Climate: global or regional climate data.

pub struct Climate {
    pub climate_type: String,
    pub avg_temp: f32,
}

impl Climate {
    pub fn new(climate_type: &str, avg_temp: f32) -> Self {
        Self { climate_type: climate_type.to_string(), avg_temp }
    }
}
