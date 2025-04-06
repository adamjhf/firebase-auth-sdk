pub mod api;
mod error;

use reqwest_middleware::ClientWithMiddleware;

pub use error::Error;

#[derive(Clone, Debug)]
pub struct FireAuth {
    pub api_key: String,
    pub client: ClientWithMiddleware,
}

impl FireAuth {
    pub fn new(api_key: String, client: ClientWithMiddleware) -> Self {
        Self { api_key, client }
    }
}
