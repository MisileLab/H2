extern crate strfmt;

use std::path::Path;

use std::thread::sleep;
use std::time::Duration;

use std::collections::{HashSet, HashMap};

use h1::{Client, ClientTrait, StreamProfile};

use tokio::{fs::{File, write}, io::AsyncReadExt};

use serde::Deserialize;

use reqwest::StatusCode;

use notify_rust::{Notification, Timeout};

use strfmt::Format;

#[derive(Deserialize)]
struct TwitchInformation {
  client_id: String,
  client_secret: String,
  streamer: HashSet<String>,
  lang: String
}

#[derive(Deserialize)]
struct TranslationHandler {
  ko_kr: Vec<String>,
  en_us: Vec<String>
}

trait TranslationTrait {
  fn get_translations(&self) -> HashMap<String, Vec<String>>;
}

impl TranslationTrait for TranslationHandler {
  fn get_translations(&self) -> HashMap<String, Vec<String>> {
    let mut a = HashMap::new();
    a.insert("ko_kr".to_string(), self.ko_kr.clone());
    a.insert("en_us".to_string(), self.en_us.clone());
    a
  }
}

#[derive(Deserialize)]
struct TwitchResponse {
  access_token: String
}

#[tokio::main]
async fn main() {
  let mut string = "".to_string();
  File::open("translation.json").await.unwrap().read_to_string(&mut string).await.unwrap();
  let translations = serde_json::from_str::<TranslationHandler>(&string).unwrap().get_translations();
  let mut seen: HashSet<String> = HashSet::new();
  let mut string = "".to_string();
  File::open("config.json").await.unwrap().read_to_string(&mut string).await.unwrap();
  let credentials: TwitchInformation = serde_json::from_str(&string).unwrap();
  let mut client = Client { 
    access_token: get_access_token().await, 
    client_id: credentials.client_id,
    filter: credentials.streamer
  };
  let mut lang: Option<Vec<String>> = None;
  for i in translations {
    if credentials.lang == i.0 {
      lang = Some(i.1);
    };
  }
  drop(string);
  loop {
    let mut temp: Option<Vec<StreamProfile>> = None;
    match client.get_stream_informations().await {
      Ok(data) => { temp = data; },
      Err(err) => {
        if err.status() == Some(StatusCode::UNAUTHORIZED) {
          client.access_token = regenerate_token().await;
        } else if err.status() != Some(StatusCode::TOO_MANY_REQUESTS) {
          println!("{:#?}", err);
          panic!("Error in request, error code: {}", err.status().unwrap_or(StatusCode::BAD_GATEWAY).as_str());
        }
      }
    };
    if let Some(tsome) = temp {
      let mut tempseen: HashSet<String> = HashSet::new();
      for t in tsome {
        let streamer_name = t.streamer_name;
        let game_name = t.game_name;
        let mut tempmap: HashMap<String, String> = HashMap::new();
        tempmap.insert("streamer_name".to_string(), streamer_name.clone());
        tempmap.insert("game_name".to_string(), game_name);
        tempseen.insert(streamer_name.clone());
        if !seen.contains(&streamer_name) {
          Notification::new()
          .summary(lang.clone().unwrap()[0].format(&tempmap).unwrap().as_ref())
          .body(lang.clone().unwrap()[1].format(&tempmap).unwrap().as_ref()) 
          .timeout(Timeout::Milliseconds(6000))
          .show().unwrap();
        }
      }
      if !tempseen.is_empty() {
        seen = tempseen;
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
    regenerate_token().await
  }
}

async fn regenerate_token() -> String {
  let mut string = "".to_string();
  File::open("config.json").await.unwrap().read_to_string(&mut string).await.unwrap();
  File::create("token.txt").await.unwrap();
  let tempstruct: TwitchInformation = serde_json::from_str(&string).unwrap();
  drop(string);
  let temp = reqwest::Client::new()
  .post(format!("https://id.twitch.tv/oauth2/token?client_id={}&client_secret={}&grant_type=client_credentials", tempstruct.client_id, tempstruct.client_secret))
  .send().await.unwrap().json::<TwitchResponse>().await.unwrap().access_token;
  write("token.txt", &temp).await.unwrap();
  temp
}
