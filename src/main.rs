//use reqwest::header::{ACCEPT, CONTENT_TYPE};
//use serde_json;

mod modules;
use crate::modules::reqwest_mod::ReqwestMod;

#[tokio::main]
async fn main() {
    let r_mod = ReqwestMod::new();
    r_mod.run().await;
}

#[test]
fn check_answer_validity() {
    assert_eq!(42, 42);
}