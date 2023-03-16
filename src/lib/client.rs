use self::com::Com;
use reqwest::blocking::Client;

pub mod com;
pub mod error;

pub struct BlueSkyClient<'a> {
    pub com: Com<'a>,
}

impl<'a> BlueSkyClient<'a> {
    pub fn new(server: &str, http_client: &'a Client) -> BlueSkyClient<'a> {
        BlueSkyClient {
            com: Com::new(http_client, &server),
        }
    }
}
