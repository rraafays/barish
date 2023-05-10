use colored::Color;
use curl::easy::{ Handler, WriteError };

pub fn get_colour(colour: u8) -> Color {
  match colour {
    0 => Color::BrightBlack,
    1 => Color::BrightRed,
    2 => Color::BrightGreen,
    3 => Color::BrightYellow,
    4 => Color::BrightBlue,
    5 => Color::BrightMagenta,
    6 => Color::BrightCyan,
    _ => Color::BrightWhite
  }
}

pub struct Collector(pub Vec<u8>);
impl Handler for Collector {
  fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
    self.0.extend_from_slice(data);
    Ok(data.len())
  }
}
