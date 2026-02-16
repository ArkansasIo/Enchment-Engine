//! Shop building definition.

pub struct Shop {
    pub shop_type: String,
    pub inventory_size: u32,
}

impl Shop {
    pub fn new(shop_type: &str, inventory_size: u32) -> Self {
        Self { shop_type: shop_type.to_string(), inventory_size }
    }
}
