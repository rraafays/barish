// https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2643123

use colored::Colorize;
use curl::easy::{ Easy2, Handler, WriteError };
use serde::Deserialize;
use serde_xml_rs::from_str;

fn main() {
  let mut curl = Easy2::new(Collector(Vec::new()));
  curl.get(true).unwrap();
  curl.url("https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2643123").unwrap();
  curl.perform().unwrap();
  assert_eq!(curl.response_code().unwrap(), 200);

  let xml = String::from_utf8_lossy(&curl.get_ref().0);
  let test: Rss = from_str(&xml).unwrap();
  println!("{}", test.channel.title.blue());
  println!("{}", test.channel.description.white());
  println!("{}", test.channel.items[0].title);
  println!("{}", test.channel.items[0].description);
}

struct Collector(Vec<u8>);
impl Handler for Collector {
  fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
    self.0.extend_from_slice(data);
    Ok(data.len())
  }
}

#[derive(Deserialize)]
struct Rss {
  channel: Channel
}

#[derive(Deserialize)]
struct Channel {
  title: String,
  description: String,
  #[serde(rename = "item")]
  items: [ Item; 3 ]
}

#[derive(Deserialize)]
struct Item {
  title: String,
  description: String
}
