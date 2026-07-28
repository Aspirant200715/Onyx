use std::net::TcpListener;

use crate::error::EmberError;

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

    pub fn is_default_port(&self) -> bool {
        self.address.ends_with(":8080")
    }

    pub fn bind(&self) -> Result<TcpListener, EmberError> {
        TcpListener::bind(&self.address)
            .map_err(|error| EmberError::Network(error.to_string()))
    }

    pub fn run(&self) -> Result<(), EmberError> {
        let listener = self.bind()?;

        println!("Ember listening on {}", self.address);

        let (stream, address) = listener
            .accept()
            .map_err(|error| EmberError::Network(error.to_string()))?;

        println!("Connection received from {}", address);

        drop(stream);

        Ok(())
    }
}