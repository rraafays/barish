// https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2643123
use curl::easy::{ Easy2, Handler, WriteError };

fn main() {
  let mut easy = Easy2::new(Collector(Vec::new()));
  easy.get(true).unwrap();
  easy.url("https://weather-broker-cdn.api.bbci.co.uk/en/forecast/rss/3day/2643123").unwrap();
  easy.perform().unwrap();

  assert_eq!(easy.response_code().unwrap(), 200);
  let contents = easy.get_ref();
  println!("{}", String::from_utf8_lossy(&contents.0));
}

struct Collector(Vec<u8>);
impl Handler for Collector {
  fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
    self.0.extend_from_slice(data);
    Ok(data.len())
  }
}
