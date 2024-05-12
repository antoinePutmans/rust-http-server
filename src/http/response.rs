use std::fmt::Display;
use std::io::Write;

use crate::http::status_code::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Self { status_code, body }
    }

    pub fn send(&self, tcp: &mut impl Write) -> std::io::Result<()> {
        let body = match &self.body {
            Some(val) => val,
            None => "No body"
        };

        write!(tcp, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
    }
}