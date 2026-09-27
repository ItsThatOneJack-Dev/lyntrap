#![cfg(feature = "blocking")]

use lyntrap::blocking::APIClient;
use lyntrap::model::FeedType;

fn client() -> APIClient {
    let id = std::env::var("LYNTR_CLIENT_ID").expect("set LYNTR_CLIENT_ID");
    let secret = std::env::var("LYNTR_CLIENT_SECRET").expect("set LYNTR_CLIENT_SECRET");
    APIClient::new(id, secret)
}

#[test]
#[ignore]
fn get_me_works() {
    let me = client().get_me().unwrap();
    assert!(!me.handle.is_empty());
}

#[test]
#[ignore]
fn feed_returns_lynts() {
    let feed = client().get_feed(FeedType::New, None).unwrap();
    assert!(!feed.is_empty());
}
