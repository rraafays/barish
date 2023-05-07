mod rss;
mod locations;
mod utils;

use rss::Rss;
use locations::get_location_code;
use utils::get_colour;

use colored::Colorize;
use curl::easy::{ Easy2, Handler, WriteError };
use serde_xml_rs::from_str;
use clap::Parser;

#[derive(Parser)]
struct Arguments {
  #[arg(short, long, default_value_t = false)]
  verbose: bool,
  #[arg(short, long)]
  location: String
}

struct Collector(Vec<u8>);
impl Handler for Collector {
  fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
    self.0.extend_from_slice(data);
    Ok(data.len())
  }
}

fn main() {
  let arguments = Arguments::parse();

  let mut curl = Easy2::new(Collector(Vec::new()));
  curl.get(true).unwrap();

  let mut link: String = String::from("https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/").to_owned();
  link.push_str(&get_location_code(arguments.location).to_string());

  curl.url(&link).unwrap() ;
  curl.perform().unwrap();

  if curl.response_code().unwrap() != 200 
  { 
    println!("{} please provide a valid location!", "error:".red()); 
    return;
  }

  let xml = String::from_utf8_lossy(&curl.get_ref().0);
  let rss: Rss = from_str(&xml).unwrap();

  let mut label: &str;
  let mut x: u8 = 4;
  for item in rss.channel.items {
    label = item.title
      .split_whitespace()
      .next()
      .unwrap_or("");
    println!("{}", item.title.replace(label, &label.color(get_colour(x)).to_string()));
    if arguments.verbose { println!("{}\n", item.description.bright_black()); }
    x += 1;
  }
}
