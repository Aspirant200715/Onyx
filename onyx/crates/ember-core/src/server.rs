use std::{
    io::{Read,Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

use ember_http::{
    response::Response,
    status::StatusCode,
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
        
        loop {
            match self.accept_connection(&listener) {
                Ok((mut stream, address)) => {
                    println!("Connection received from {}", address);
                    
                    match self.read_request(&mut stream) {
                        Ok(request) => {
                            println!("RAW REQUEST");
                            println!("{}", request);
                            if let Err(e) = self.write_response(&mut stream, StatusCode::Ok, "Welcome to Onyx!") {
                                eprintln!("Failed to send response: {:?}", e);
                            } else {
                                println!("Response sent successfully.");
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading request: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to accept connection: {:?}", e);
                }
            }
        }
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
        status: StatusCode,
        body: &str,
    ) -> Result<(), EmberError> {
        let response = Response::new(status)
            .header("Content-Type", "text/plain")
            .header("Server", "Ember")
            .body(body);

        let bytes = response.serialize();

        stream.write_all(&bytes)
            .map_err(|error| EmberError::Network(error.to_string()))?;

        Ok(())
    }
}