// https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2643123
mod rss;

use rss::Rss;
use colored::Colorize;
use curl::easy::{ Easy2, Handler, WriteError };
use serde_xml_rs::from_str;

fn main() {
  let mut curl = Easy2::new(Collector(Vec::new()));
  curl.get(true).unwrap();
  curl.url("https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2644688").unwrap();
  curl.perform().unwrap();
  assert_eq!(curl.response_code().unwrap(), 200);

  let xml = String::from_utf8_lossy(&curl.get_ref().0);
  let test: Rss = from_str(&xml).unwrap();

  println!("{}", test.channel.title.blue());
  println!("{}", test.channel.description.white());
  println!("{}", test.channel.items[0].title);
  println!("{}", test.channel.items[0].description);

  println!("{}", bbc_location::Location::Leeds as u32);
}

struct Collector(Vec<u8>);
impl Handler for Collector {
  fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
    self.0.extend_from_slice(data);
    Ok(data.len())
  }
}

mod bbc_location {
  pub enum Location {
    Leeds = 2644688,
  }
}
