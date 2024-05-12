use std::fmt;
use std::fmt::{Display, Formatter};

use crate::http::method::MethodError;

#[derive(Debug)]
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocolError,
    InvalidMethod,
    InvalidHeader,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            ParseError::InvalidMethod => "Invalid method exception",
            ParseError::InvalidEncoding => "Invalid encoding exception",
            ParseError::InvalidProtocolError => "Invalid protocol exception",
            ParseError::InvalidRequest => "Invalid request exception",
            ParseError::InvalidHeader => "Invalid header exception"
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}


impl From<MethodError> for ParseError {
    fn from(_value: MethodError) -> Self {
        Self::InvalidEncoding
    }
}
