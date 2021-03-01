use std::net::TcpListener;
use std::io::Read;
use std::convert::TryFrom;
use crate::http::{ParseError, Request};

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            self::to_request(&buffer)
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }

    fn to_request(buffer: &[u8]) {
        match Request::try_from(buffer) {
            Ok(Request { path, query_string, method}) => {

                println!("path: {}, method: {:?}",path, method)
            },
            Err(e) => println!("Failed to process request: {}", e),
        }
    }
}
