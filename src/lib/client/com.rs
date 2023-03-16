use reqwest::blocking::Client;

use self::atproto::ATProto;

pub mod atproto;

pub struct Com<'a> {
    pub atproto: ATProto<'a>,
}

impl<'a> Com<'a> {
    pub fn new(http_client: &'a Client, server: &str) -> Self {
        Com {
            atproto: ATProto::new(http_client, server),
        }
    }
}
