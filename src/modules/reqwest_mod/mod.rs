use reqwest;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde_json::{self};
pub struct ReqwestMod {
  pub client: reqwest::Client
}

impl ReqwestMod {
  pub fn new() -> ReqwestMod {
    ReqwestMod { client: reqwest::Client::new() }
  }

  async fn get_sample_json(&self, url: &str) -> serde_json::Value {
    let res = &self.client
      .get(url)
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

  pub async fn get_iced_recipes(&self) -> serde_json::Value {
    Self::get_sample_json(&self, "https://api.sampleapis.com/coffee/iced").await
  }

  pub async fn get_hot_recipes(&self) -> serde_json::Value {
    Self::get_sample_json(&self, "https://api.sampleapis.com/coffee/hot").await
  }

  pub async fn get_bank_info(&self) -> serde_json::Value {
    Self::get_sample_json(&self,"https://api.sampleapis.com/fakebank/accounts").await
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
  /** Demo function that returns the first JSON object of each API request. */
  pub async fn run(&self) {
    // Run all 3 routes, and return the first `Object` from each JSON body
    let iced_recipes = Self::get_iced_recipes(&self).await;
    let hot_recipes = Self::get_hot_recipes(&self).await;
    let bank_info = Self::get_bank_info(&self).await;

    println!("{}\n", iced_recipes[0]);
    println!("{}\n", hot_recipes[0]);
    println!("{}\n", bank_info[0]);
  }

  fn status_code(res: &reqwest::Response) -> reqwest::StatusCode {
    res.status() 
  }
}