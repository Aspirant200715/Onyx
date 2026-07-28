use std::{
    io::{Read,Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

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
    let (mut stream, address) = self.accept_connection(&listener)?;
    println!("Connection received from {}", address);
    let request = self.read_request(&mut stream)?;
    println!("RAW REQUEST");
    println!("{}", request);
    self.write_response(&mut stream, 200, "Welcome to Ember!")?;
    println!("Response sent successfully.");
    Ok(())
  }

    fn accept_connection(
        &self,
        listener: &TcpListener,
    )-> Result<(TcpStream, SocketAddr), EmberError> {
        listener
            .accept()
            .map_err(|error| EmberError::Network(error.to_string()))
    }


    fn read_request(
     &self,
     stream: &mut TcpStream,
    ) -> Result<String, EmberError> {
    let mut buffer = [0; 1024];

    let bytes_read = stream
        .read(&mut buffer)
        .map_err(|error| EmberError::Network(error.to_string()))?;

    Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }

    fn write_response(
        &self,
        stream: &mut TcpStream,
        status_code: u16,
        content: &str,
    ) -> Result<(), EmberError> {
        let response = format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
            status_code,
            content.len(),
            content
        );

        stream
            .write_all(response.as_bytes())
            .map_err(|error| EmberError::Network(error.to_string()))
    }
}