use ember_core::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080");

    if let Err(error) = server.run() {
        println!("Server error: {:?}", error);
    }
}