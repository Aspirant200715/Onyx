use ember_core::server::Server;
use ember_http::request::Request;


use ember_core::{extractor::FromRequest, path::Path,
query::Query,
header::Header,};

fn home(_: Request) -> &'static str {
    "Hello from Onyx!"
}

fn about(_: Request) -> String {
    "About Onyx Framework".to_string()
}

fn user(request: Request) -> String {
    let id = Path::named("id").from_request(&request).unwrap();

    format!("User ID: {}", id)
}

fn search(request: Request) -> String {
    let query = Query::named("q").from_request(&request).unwrap();

    format!("Search query: {}", query)
}

fn host(request: Request) -> String {
    let host = Header::named("Host").from_request(&request).unwrap();

    format!("Host: {}", host)
}

fn main() {
    let mut server = Server::new("127.0.0.1:8080");

    server.router_mut().get("/", home);
    server.router_mut().get("/about", about);
    server.router_mut().get("/users/:id", user);
    server.router_mut().get("/search", search);
    server.router_mut().get("/host", host);
    if let Err(error) = server.run() {
        println!("Server error: {:?}", error);
    }
}
