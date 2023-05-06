// https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2643123
mod rss;
mod locations;

use rss::Rss;
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

fn main() {
  let arguments = Arguments::parse();

  let mut curl = Easy2::new(Collector(Vec::new()));
  curl.get(true).unwrap();

  let mut link: String = String::from("https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/").to_owned();
  link.push_str(&locations::get_location_code(arguments.location).to_string());

  curl.url(&link).unwrap() ;
  curl.perform().unwrap();

  if curl.response_code().unwrap() != 200 
  { 
    println!("{} please provide a valid location!", "error:".red()); 
    return;
  }

  let xml = String::from_utf8_lossy(&curl.get_ref().0);
  let test: Rss = from_str(&xml).unwrap();

  if arguments.verbose { println!("{}\n", test.channel.description.bright_blue()); }

  println!("{}", test.channel.items[0].title.replace("Today:", &"Today:".bright_cyan().to_string()));
  if arguments.verbose { println!("{}\n", test.channel.items[0].description.bright_black()); }
  println!("{}", test.channel.items[1].title
    .replace("Monday:", &"Monday:".bright_magenta().to_string())
    .replace("Tuesday:", &"Tuesday:".bright_magenta().to_string())
    .replace("Wednesday:", &"Wednesday:".bright_magenta().to_string())
    .replace("Thursday:", &"Thursday:".bright_magenta().to_string())
    .replace("Friday:", &"Friday:".bright_magenta().to_string())
    .replace("Saturday:", &"Saturday:".bright_magenta().to_string())
    .replace("Sunday:", &"Sunday:".bright_magenta().to_string())
  );
  if arguments.verbose { println!("{}\n", test.channel.items[1].description.bright_black()); }
  println!("{}", test.channel.items[2].title
    .replace("Monday:", &"Monday:".bright_yellow().to_string())
    .replace("Tuesday:", &"Tuesday:".bright_yellow().to_string())
    .replace("Wednesday:", &"Wednesday:".bright_yellow().to_string())
    .replace("Thursday:", &"Thursday:".bright_yellow().to_string())
    .replace("Friday:", &"Friday:".bright_yellow().to_string())
    .replace("Saturday:", &"Saturday:".bright_yellow().to_string())
    .replace("Sunday:", &"Sunday:".bright_yellow().to_string())
  );
  if arguments.verbose { println!("{}\n", test.channel.items[2].description.bright_black()); }
}

struct Collector(Vec<u8>);
impl Handler for Collector {
  fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
    self.0.extend_from_slice(data);
    Ok(data.len())
  }
}
