//! Networking: Dedicated server logic.

pub struct Server {
    pub address: String,
}

impl Server {
    pub fn new(address: &str) -> Self {
        Self { address: address.to_string() }
    }
    pub fn start(&self) {
        // TODO: implement server start
    }
}
