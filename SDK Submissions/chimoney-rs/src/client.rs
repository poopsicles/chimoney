use crate::schema::responses::Welcome;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChimoneyError {
    #[error("Something went wrong, `{0}`")]
    Generic(String),
}

type ChiResult<T> = Result<T, ChimoneyError>;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[allow(clippy::module_name_repetitions)]
pub struct ChimoneyClient {
    key: String,
    server: String,
    client: reqwest::Client,
}

impl ChimoneyClient {
    /// Create a new client for interacting with the production API.
    #[must_use]
    pub fn new(api_key: &str) -> Self {
        Self {
            key: api_key.to_string(),
            server: "https://api.chimoney.io/".to_string(),

            ..Default::default()
        }
    }

    /// Create a new client for interacting with the sandbox API.
    #[must_use]
    pub fn sandbox(api_key: &str) -> Self {
        Self {
            key: api_key.to_string(),
            server: "https://api-v2-sandbox.chimoney.io/".to_string(),

            ..Default::default()
        }
    }

    /// Ping the Chimoney API to test your connection.
    pub async fn ping(&self) -> ChiResult<Welcome> {
        match self.client.get(&self.server).send().await {
            Err(e) => Err(ChimoneyError::Generic(e.to_string())),
            Ok(r) => match r.json::<Welcome>().await {
                Ok(x) => Ok(x),
                Err(e) => Err(ChimoneyError::Generic(e.to_string())),
            },
        }
    }
}

impl Default for ChimoneyClient {
    fn default() -> Self {
        Self {
            key: String::new(),
            server: String::new(),
            client: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .build()
                .expect("unable to make reqwest client"),
        }
    }
}
