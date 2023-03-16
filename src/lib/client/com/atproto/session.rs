use std::collections::HashMap;
use crate::xrpc::Error;
use crate::xrpc::com::atproto::session::{SessionManager, SessionState};
use crate::xrpc::error::XrpcError;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};


pub struct Session<'a> {
    client: &'a reqwest::blocking::Client,
    server: String,
}

impl<'a> Session<'a> {
    pub fn new(http_client: &'a Client, server: &str) -> Session<'a> {
        Session {
            client: http_client,
            server: String::from(server),
        }
    }
}

impl<'a> SessionManager for Session<'a> {
    fn create(&self, handle: &str, password: &str) -> Result<SessionState, Error> {
        println!("Getting account info for {0} from {1}", self.server, handle);

        let mut body = HashMap::new();
        body.insert("handle", handle);
        body.insert("password", password);

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        let url = format!("{0}/xrpc/com.atproto.session.create", self.server);
        let response = self.client.post(url).headers(headers).json(&body).send()?;

        if response.status().is_success() {
            let session: SessionState = response.json::<SessionState>()?;
            return Ok(session);
        }

        Err(Error::from(XrpcError::from_response(response)?))
    }
}
