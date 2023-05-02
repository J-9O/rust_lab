use reqwest;

pub struct ReqwestMod;

impl ReqwestMod {
  pub async fn run() -> reqwest::Response {
    match reqwest::get("https://api.spotify.com/v1/search").await {
      Ok(response) => response,
      Err(err) => panic!("Error: {:?}", err)
    }
  }

  pub fn status_code(res: &reqwest::Response) -> reqwest::StatusCode {
    res.status() 
  }
}