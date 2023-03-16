use crate::xrpc::error::{XrpcError, XrpcErrorDescription};
use crate::xrpc::Error;
use reqwest::blocking::Response;

impl XrpcError {
    pub fn from_response(response: Response) -> Result<XrpcError, Error> {
        let code = response.status().as_u16();
        let error_description: XrpcErrorDescription = response.json()?;

        return Ok(XrpcError::new(code, error_description));
    }
}
