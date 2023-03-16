use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct XrpcErrorDescription {
    error: String,
    message: String,
}

#[derive(Debug)]
pub struct XrpcError {
    error_code: u16,
    description: XrpcErrorDescription
}

impl XrpcError {
    pub fn new(error_code: u16, description: XrpcErrorDescription) -> XrpcError{
        XrpcError {
            error_code,
            description
        }
    }  
}