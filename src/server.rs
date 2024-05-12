use std::fmt::Debug;
use std::io::Read;
use std::net::TcpListener;

use crate::http::errors::parse_error::ParseError;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;

pub struct Server {
    addr: String,
}

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}


impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server running on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut tcp, sockaddr)) => {
                    println!("New connection from: {}", sockaddr.ip());
                    let mut buffer = [0; 1024];
                    match tcp.read(&mut buffer) {
                        Ok(_size) => {
                            println!("Received a new request");
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    let res = handler.handle_request(&request);
                                    if let Err(err) = res.send(&mut tcp) {
                                        println!("Error while sending the response: {}", err);
                                    }
                                }
                                Err(error) => {
                                    handler.handle_bad_request(&error);
                                }
                            };
                        }
                        Err(err) => println!("Error while reading data from tcp connection: {}", err)
                    }
                }
                Err(err) => println!("Error: {}", err)
            }
        }
    }
}

