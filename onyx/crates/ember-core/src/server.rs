/// Represents the main Onyx HTTP server.
pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }
}