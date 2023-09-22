extern crate reqwest;

use reqwest::Client;

struct HttpClient {
    jwt: Option<String>,
    client: Client,
}

impl HttpClient {
    fn new() -> HttpClient {
        HttpClient {
            jwt: None,
            client: Client::new(),
        }
    }
}
