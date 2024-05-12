use std::str::{from_utf8, FromStr, Utf8Error};

use crate::http::errors::parse_error::ParseError;
use crate::http::method::Method;
use crate::http::query_string::QueryString;

#[derive(Debug)]
pub struct Request<'buff> {
    path: &'buff str,
    query_string: Option<QueryString<'buff>>,
    method: Method,
}


impl<'buff> Request<'buff> {
    pub fn new(path: &'buff str, query_string: Option<QueryString<'buff>>, method: Method) -> Self {
        Self { path, query_string, method }
    }

    pub fn path(&self) -> &'buff str {
        self.path
    }
    pub fn query_string(&self) -> &Option<QueryString<'buff>> {
        &self.query_string
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
}

impl<'buff> TryFrom<&'buff [u8]> for Request<'buff> {
    type Error = ParseError;

    fn try_from(value: &'buff [u8]) -> Result<Self, Self::Error> {
        let req = from_utf8(value)?;

        let (method, req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (mut path, req): (&str, &str) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _req) = get_next_word(req).ok_or(ParseError::InvalidRequest)?;

        let protocol = protocol.trim();

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocolError);
        }
        let method: Method = Method::from_str(method)?;

        let mut query_string = None;

        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }


        Ok(Self {
            path,
            query_string,
            method,
        })
    }
}


fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\n' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }
    None
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
