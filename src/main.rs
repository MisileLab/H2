use std::path::Path;

use std::thread::sleep;
use std::time::Duration;

use std::collections::HashSet;

use h1::{Client, ClientTrait, StreamProfile};

use tokio::{fs::{File, write}, io::AsyncReadExt};

use serde::Deserialize;

use reqwest::StatusCode;

use notify_rust::{Notification, Timeout};

#[derive(Deserialize)]
struct TwitchInformation {
  client_id: String,
  client_secret: String,
  streamer: HashSet<String>
}

#[derive(Deserialize)]
struct TwitchResponse {
  access_token: String
}

#[tokio::main]
async fn main() {
  let mut seen: HashSet<String> = HashSet::new();
  let mut string = "".to_string();
  let mut file = File::open("config.json").await.unwrap();
  file.read_to_string(&mut string).await.unwrap();
  let credentials: TwitchInformation = serde_json::from_str(&string).unwrap();
  let mut client = Client { 
    access_token: get_access_token().await, 
    filter: credentials.streamer
  };
  drop((file, string));
  loop {
    let mut temp: Option<Vec<StreamProfile>> = None;
    match client.get_stream_informations().await {
      Ok(data) => { temp = data; },
      Err(err) => {
        if err.status() == Some(StatusCode::UNAUTHORIZED) {
          client.access_token = regenerate_token().await;
        } else if err.status() != Some(StatusCode::TOO_MANY_REQUESTS) {
          panic!("Error in request, error code: {}", err.status().unwrap().as_str());
        }
      }
    };
    if temp.is_some() {
      let mut tempseen: HashSet<String> = HashSet::new();
      for t in temp.unwrap() {
        let streamer_name = t.streamer_name;
        let game_name = t.game_name;
        tempseen.insert(streamer_name.clone());
        if !seen.contains(&streamer_name) {
          Notification::new()
          .summary(format!("{streamer_name}님의 방송이 켜졌어요!").as_str()) // for english, It says {streamer_name}'s stream on!
          .body(format!("{game_name} 플레이 중").as_str()) // for english, It says playing {game_name}
          .timeout(Timeout::Milliseconds(6000))
          .show().unwrap();
        }
      }
      if !tempseen.len() == 0 {
        seen = tempseen
      }
    }
    sleep(Duration::from_secs(10))
  }
}

async fn get_access_token() -> String {
  let mut string = "".to_string();
  if Path::exists(Path::new("token.txt")) {
    File::open("token.txt").await.unwrap().read_to_string(&mut string).await.unwrap();
    string
  } else {
    File::create("token.txt").await.unwrap();
    regenerate_token().await
  }
}

async fn regenerate_token() -> String {
  let mut string = "".to_string();
  File::open("config.json").await.unwrap().read_to_string(&mut string).await.unwrap();
  File::open("token.txt").await.unwrap().set_len(0).await.unwrap();
  let tempstruct: TwitchInformation = serde_json::from_str(&string).unwrap();
  drop(string);
  let temp = reqwest::Client::new()
  .post(format!("https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials", tempstruct.client_id, tempstruct.client_secret))
  .send().await.unwrap().json::<TwitchResponse>().await.unwrap().access_token;
  write("token.txt", &temp).await.unwrap();
  temp
}