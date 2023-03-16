use error::XrpcError;
use reqwest::Error as ReqwestError;
use std::error::Error as StdError;
use std::fmt;
use std::fmt::Display;


pub mod com;
pub mod error;

#[derive(Debug)]
pub enum Error {
    Xrpc(XrpcError),
    Http(ReqwestError),
}

impl StdError for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Http(error)
    }
}

impl From<error::XrpcError> for Error {
    fn from(error: XrpcError) -> Self {
        Error::Xrpc(error)
    }
}
