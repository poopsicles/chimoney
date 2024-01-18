use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Welcome {
    pub message: String,
    pub swaggerdocs: String,
    #[serde(rename = "apiDocs")]
    pub api_docs: String,
}
