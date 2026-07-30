use ember_core::server::Server;
use ember_http::request::Request;

fn home(_: Request) -> &'static str {
    "Hello from Onyx!"
}

fn about(_: Request) -> String {
    "About Onyx Framework".to_string()
}

fn main() {
    let mut server = Server::new("127.0.0.1:8080");

    server.router_mut().get("/", home);
    server.router_mut().get("/about", about);

    if let Err(error) = server.run() {
        println!("Server error: {:?}", error);
    }
}