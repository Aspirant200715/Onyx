use ember_core::server::Server;
use ember_http::{
    request::Request,
    response::Response,
    status::StatusCode,
};

fn home(_: Request) -> Response {
    Response::new(StatusCode::Ok)
        .body("Welcome to Onyx!")
}

fn about(_: Request) -> Response {
    Response::new(StatusCode::Ok)
        .body("About Onyx Framework")
}

fn main() {
    let mut server = Server::new("127.0.0.1:8080");

    server.router_mut().get("/", home);
    server.router_mut().get("/about", about);

    if let Err(error) = server.run() {
        println!("Server error: {:?}", error);
    }
}