use chimoney::client::ChimoneyClient;
use dotenvy::dotenv;
use std::env;

#[tokio::test]
async fn info_airtime_countries_works() {
    dotenv().ok();

    let api_key = env::var("CHIMONEY_API_KEY").expect("unable to read key from environment");

    let client = ChimoneyClient::sandbox(&api_key);

    let res = client
        .get_airtime_countries()
        .await
        .expect("unable to get airtime countries");

    println!("{res:?}");
    assert!(!res.is_empty());
}
