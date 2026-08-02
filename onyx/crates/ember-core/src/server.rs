use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::Arc,
};

use crate::middleware::{Middleware, Next};
use ember_http::response::Response;

use crate::{error::EmberError, router::Router};

pub struct Server {
    address: String,
    router: Router,
    middleware: Vec<Arc<dyn Middleware>>,
}

impl Server {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            router: Router::new(),
            middleware: Vec::new(),
        }
    }

    pub fn use_middleware<M>(&mut self, middleware: M)
    where
        M: Middleware + 'static,
    {
        self.middleware.push(Arc::new(middleware));
    }

    pub fn router_mut(&mut self) -> &mut Router {
        &mut self.router
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn is_default_port(&self) -> bool {
        self.address.ends_with(":8080")
    }

    pub fn bind(&self) -> Result<TcpListener, EmberError> {
        TcpListener::bind(&self.address).map_err(|error| EmberError::Network(error.to_string()))
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

                            match ember_http::parser::HttpParser::parse_request(&request) {
                                Ok(parsed_request) => {
                                    println!("{:#?}", parsed_request);

                                    let pipeline = self.build_pipeline();
                                    let response = pipeline.run(parsed_request);

                                    if let Err(e) = self.write_response(&mut stream, response) {
                                        eprintln!("Failed to send response: {:?}", e);
                                    } else {
                                        println!("Response sent successfully.");
                                    }
                                }

                                Err(error) => {
                                    eprintln!("Parse error: {:?}", error);
                                }
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
    ) -> Result<(TcpStream, SocketAddr), EmberError> {
        listener
            .accept()
            .map_err(|error| EmberError::Network(error.to_string()))
    }

    fn read_request(&self, stream: &mut TcpStream) -> Result<String, EmberError> {
        let mut buffer = [0; 1024];

        let bytes_read = stream
            .read(&mut buffer)
            .map_err(|error| EmberError::Network(error.to_string()))?;

        Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
    }

    fn write_response(&self, stream: &mut TcpStream, response: Response) -> Result<(), EmberError> {
        let bytes = response.serialize();

        stream
            .write_all(&bytes)
            .map_err(|error| EmberError::Network(error.to_string()))?;

        Ok(())
    }

    fn build_pipeline(&self) -> Next {
        let router = self.router.clone();

        let mut next = Next::from_handler(move |request| router.dispatch(request));

        for middleware in self.middleware.iter().rev() {
            let current = next.clone();

            let middleware = Arc::clone(middleware);

            next = Next::from_handler(move |request| {
                middleware.handle(request, &current)
            });
        }

        next
    }
}
