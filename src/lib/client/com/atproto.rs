use reqwest::blocking::Client;

use self::session::Session;

pub mod account;
pub mod session;

pub struct ATProto<'a> {
    pub session: Session<'a>,
}

impl<'a> ATProto<'a> {
    pub fn new(http_client: &'a Client, server: &str) -> Self {
        ATProto {
            session: Session::new(http_client, server),
        }
    }
}