use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde_json::{self, error};
pub struct ReqwestMod {
  pub client: reqwest::Client
}

impl ReqwestMod {
  pub fn new() -> ReqwestMod {
    ReqwestMod { client: reqwest::Client::new() }
  }

  pub async fn get_iced_recipes(&self) -> serde_json::Value {
    let res = &self.client
        .get("https://api.sampleapis.com/coffee/iced")
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await;


    match res {
        Ok(json_res) => json_res.to_owned(),
        Err(e) => Self::err_json(e),
    }
  }

  fn err_json (e: &reqwest::Error) -> serde_json::Value {
    let error_json = format!("
      {{
        \"Message\": \"UH-OH! We got an error!\", 
        \"Error\": {:?}
      }}", 
      e.to_string()
    );

    let err = error_json.as_str();

    serde_json::from_str(err).unwrap()
  }

  pub async fn run() -> reqwest::Response {
    match reqwest::get("https://api.sampleapis.com/coffee/iced").await {
      Ok(response) => response,
      Err(err) => panic!("Error: {:?}", err)
    }
  }

  pub fn status_code(res: &reqwest::Response) -> reqwest::StatusCode {
    res.status() 
  }
}