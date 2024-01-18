use chimoney::client::ChimoneyClient;
use dotenvy::dotenv;
use std::env;

#[tokio::test]
async fn ping_sandbox_works() {
    dotenv().ok();

    let client = ChimoneyClient::sandbox(
        env::var("CHIMONEY_API_KEY")
            .expect("unable to read key from environment")
            .as_str(),
    );

    let res = client.ping().await.expect("unable to ping API");

    assert_eq!("Welcome to Chimoney's API service", res.message);
    assert_eq!(
        "https://api-v2-sandbox.chimoney.io/api-docs/",
        res.swaggerdocs
    );
    assert_eq!(
        "https://chimoney.readme.io/reference/introduction",
        res.api_docs
    );
}
