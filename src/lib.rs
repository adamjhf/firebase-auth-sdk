pub mod api;
mod error;

use reqwest::Client;

pub use error::Error;

#[derive(Clone, Debug)]
pub struct FireAuth {
    pub api_key: String,
    pub client: Client,
}

impl FireAuth {
    pub fn new(api_key: String, client: Client) -> Self {
        Self { api_key, client }
    }
}
