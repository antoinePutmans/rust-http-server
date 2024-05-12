use std::fs;

use crate::http::method::Method;
use crate::http::request::Request;
use crate::http::response::Response;
use crate::http::status_code::StatusCode;
use crate::server::Handler;

/// Test
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {

    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Not cool");
                    None
                }
            }
            Err(_) => { None }
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::Get => match request.path() {
                "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => Response::new(StatusCode::Ok, self.read_file("hello.html")),
                "/interesting" => Response::new(StatusCode::Ok, self.read_file("interesting.json")),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}