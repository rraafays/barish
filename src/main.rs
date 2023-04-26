// https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2643123
use curl::easy::{ Easy2, Handler, WriteError };

fn main() {
  let mut curl = Easy2::new(Collector(Vec::new()));
  curl.get(true).unwrap();
  curl.url("https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2643123").unwrap();
  curl.perform().unwrap();

  assert_eq!(curl.response_code().unwrap(), 200);
  let xml = &curl.get_ref().0;
  println!("{}", String::from_utf8_lossy(xml));
}

struct Collector(Vec<u8>);
impl Handler for Collector {
  fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
    self.0.extend_from_slice(data);
    Ok(data.len())
  }
}
