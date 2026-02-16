//! Castle building definition.

pub struct Castle {
    pub lord: String,
    pub towers: u32,
}

impl Castle {
    pub fn new(lord: &str, towers: u32) -> Self {
        Self { lord: lord.to_string(), towers }
    }
}
