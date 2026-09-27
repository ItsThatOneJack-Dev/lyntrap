pub mod error;
pub mod media;
pub mod model;
pub(crate) mod request;

#[cfg(feature = "async")]
pub mod r#async;

#[cfg(feature = "blocking")]
pub mod blocking;

pub use error::Error;
pub use media::Image;

pub(crate) const DEFAULT_BASE_URL: &str = "https://lyntr.gizmowizard.tech/api/v2";

#[derive(Debug, Clone)]
pub(crate) struct Config {
    pub base_url: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Config {
    pub fn new(client_id: impl Into<String>, client_secret: impl Into<String>) -> Self {
        Self {
            base_url: DEFAULT_BASE_URL.to_string(),
            client_id: client_id.into(),
            client_secret: client_secret.into(),
        }
    }

    pub fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }
}
