use std::path::Path;
use tokio::fs::File;

struct TwitchInformation {
  client_id: String,
  client_secret: String
}

#[tokio::main]
async fn main() {
  
}

async fn backend(requestHandler: reqwest::Client) {
  // let res =
}

async fn get_access_token() {
  let file = if Path::exists(Path::new("token.txt")) {
    File::open("token.txt").await.unwrap()
  } else {
    let temp = File::create("token.txt").await.unwrap();
    let temp2: TwitchInformation = serde_json::from_str(File::open("config.json").await.unwrap());
  };
}