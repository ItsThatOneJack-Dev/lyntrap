#![cfg(feature = "async")]

use lyntrap::r#async::APIClient;
use lyntrap::model::FeedType;

fn client() -> APIClient {
    let id = std::env::var("LYNTR_CLIENT_ID").expect("set LYNTR_CLIENT_ID");
    let secret = std::env::var("LYNTR_CLIENT_SECRET").expect("set LYNTR_CLIENT_SECRET");
    APIClient::new(id, secret)
}

#[tokio::test(flavor = "current_thread")]
#[ignore]
async fn get_me_works() {
    let me = client().get_me().await.unwrap();
    assert!(!me.handle.is_empty());
}

#[tokio::test(flavor = "current_thread")]
#[ignore]
async fn feed_returns_lynts() {
    let feed = client().get_feed(FeedType::New, None).await.unwrap();
    assert!(!feed.is_empty());
}
