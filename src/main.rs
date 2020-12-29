use server::Server;
use http::method::Method;
use std::fmt::Display;

mod http;
mod server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}
