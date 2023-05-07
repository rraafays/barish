use colored::Color;

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
