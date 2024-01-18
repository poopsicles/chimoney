use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Welcome {
    pub message: String,
    pub swaggerdocs: String,
    #[serde(rename = "apiDocs")]
    pub api_docs: String,
}

#[derive(Debug, Deserialize)]
pub struct GoodResponse<T> {
    pub status: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    pub status: String,
    pub message: String,
}

pub type AirtimeCountriesResponse = Vec<String>;
