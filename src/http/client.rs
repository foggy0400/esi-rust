extern crate reqwest;

use reqwest::Client;

pub struct HttpClient {
    jwt: Option<String>,
    client: Client,
}

impl HttpClient {
    pub fn new() -> HttpClient {
        HttpClient {
            jwt: None,
            client: Client::new(),
        }
    }
}
