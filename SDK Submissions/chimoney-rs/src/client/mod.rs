mod info;

use crate::schema::responses::Welcome;

#[derive(Debug, thiserror::Error)]
pub enum ChimoneyError {
    #[error("Something went wrong, `{0}`")]
    Generic(String),
    #[error("Unable to deserialise into target type, `{0}`")]
    Deserialise(String),
    #[error("Request unauthenticated, `{0}`")]
    Unauthenticated(String),
    #[error("Request forbidden, `{0}`")]
    Forbidden(String),
    #[error("Something's wrong and it's not documented with the Chimoney API, `{0}`")]
    Undocumented(String),
}

pub type ChimoneyResult<T> = Result<T, ChimoneyError>;

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub struct ChimoneyClient<'a> {
    api_key: &'a str,
    server: &'static str,
    reqwest_client: reqwest::Client,
}

impl<'a> ChimoneyClient<'a> {
    /// Create a new client for interacting with the production API.
    #[must_use]
    pub fn new(api_key: &'a str) -> Self {
        Self {
            api_key,
            server: "https://api.chimoney.io",

            ..Default::default()
        }
    }

    /// Create a new client for interacting with the sandbox API.
    #[must_use]
    pub fn sandbox(api_key: &'a str) -> Self {
        Self {
            api_key,
            server: "https://api-v2-sandbox.chimoney.io",

            ..Default::default()
        }
    }

    /// Ping the Chimoney API to test your connection.
    pub async fn ping(&self) -> ChimoneyResult<Welcome> {
        match self.reqwest_client.get(self.server).send().await {
            Err(e) => Err(ChimoneyError::Generic(e.to_string())),
            Ok(r) => match r.json().await {
                Ok(x) => Ok(x),
                Err(e) => Err(ChimoneyError::Deserialise(e.to_string())),
            },
        }
    }
}

impl<'a> Default for ChimoneyClient<'a> {
    fn default() -> Self {
        Self {
            api_key: "",
            server: "",
            reqwest_client: reqwest::Client::builder()
                .user_agent(APP_USER_AGENT)
                .build()
                .expect("unable to make reqwest client"),
        }
    }
}
