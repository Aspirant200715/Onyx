use crate::router::initialize_router;

pub fn start_server() {
    initialize_router();
    initialize_network();

    println!("Server started.");
}

fn initialize_network() {
    println!("Initializing TCP listener...");
}

pub fn stop_server() {
    cleanup_resources();
    println!("Server stopped.");
}

fn cleanup_resources() {
    println!("Cleaning resources...");
}