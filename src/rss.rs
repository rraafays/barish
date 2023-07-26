use serde::Deserialize;

#[derive(Deserialize)]
pub struct Rss {
  pub channel: Channel,
}

#[derive(Deserialize)]
pub struct Channel {
  pub title: String,
  pub description: String,
  #[serde(rename = "item")]
  pub items: [Item; 3],
}

#[derive(Deserialize)]
pub struct Item {
  pub title: String,
  pub description: String,
}
