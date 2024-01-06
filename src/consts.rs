pub(crate) const ABOUT: &str = concat!(
  env!("CARGO_PKG_DESCRIPTION"),
  " - ",
  env!("CARGO_PKG_HOMEPAGE")
);

pub(crate) const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");

/// Default value for `created by` torrent metainfo field.
///
/// Example: imdl/0.0.0 (1234567890ab)
pub(crate) const CREATED_BY_DEFAULT: &str = concat!(
  "imdl/",
  env!("CARGO_PKG_VERSION"),
  env!("GIT_HEAD_PARTIAL_HASH"),
);

/// Value for `encoding` torrent metainfo field.
pub(crate) const ENCODING_UTF8: &str = "UTF-8";

pub(crate) const HELP_MESSAGE: &str = "Print help message.";

/// The progress chars are from the
/// [Block Elements unicode block](https://en.wikipedia.org/wiki/Block_Elements).
pub(crate) const PROGRESS_CHARS: &str = "█▉▊▋▌▍▎▏ ";

/// The tick chars are from the
/// [Braille Patterns unicode block](https://en.wikipedia.org/wiki/Braille_Patterns).
///
/// The chars are ordered to represent the 8 bit numbers in increasing
/// order. The individual braille cells represent bits, with empty cells
/// representing `0` and full cells representing `1`.
///
/// Digits are ordered from least significant to most significant from
/// top to bottom, and then left to right, like so:
///
/// ```text
/// ╔═════╗
/// ║ 0 4 ║
/// ║ 1 5 ║
/// ║ 2 6 ║
/// ║ 3 7 ║
/// ╚═════╝
/// ```
pub(crate) const TICK_CHARS: &str = concat!(
  "⠀⠁⠂⠃⠄⠅⠆⠇⡀⡁⡂⡃⡄⡅⡆⡇", // 0b0000----
  "⠈⠉⠊⠋⠌⠍⠎⠏⡈⡉⡊⡋⡌⡍⡎⡏", // 0b0001----
  "⠐⠑⠒⠓⠔⠕⠖⠗⡐⡑⡒⡓⡔⡕⡖⡗", // 0b0010----
  "⠘⠙⠚⠛⠜⠝⠞⠟⡘⡙⡚⡛⡜⡝⡞⡟", // 0b0011----
  "⠠⠡⠢⠣⠤⠥⠦⠧⡠⡡⡢⡣⡤⡥⡦⡧", // 0b0100----
  "⠨⠩⠪⠫⠬⠭⠮⠯⡨⡩⡪⡫⡬⡭⡮⡯", // 0b0101----
  "⠰⠱⠲⠳⠴⠵⠶⠷⡰⡱⡲⡳⡴⡵⡶⡷", // 0b0110----
  "⠸⠹⠺⠻⠼⠽⠾⠿⡸⡹⡺⡻⡼⡽⡾⡿", // 0b0111----
  "⢀⢁⢂⢃⢄⢅⢆⢇⣀⣁⣂⣃⣄⣅⣆⣇", // 0b1000----
  "⢈⢉⢊⢋⢌⢍⢎⢏⣈⣉⣊⣋⣌⣍⣎⣏", // 0b1001----
  "⢐⢑⢒⢓⢔⢕⢖⢗⣐⣑⣒⣓⣔⣕⣖⣗", // 0b1010----
  "⢘⢙⢚⢛⢜⢝⢞⢟⣘⣙⣚⣛⣜⣝⣞⣟", // 0b1011----
  "⢠⢡⢢⢣⢤⢥⢦⢧⣠⣡⣢⣣⣤⣥⣦⣧", // 0b1100----
  "⢨⢩⢪⢫⢬⢭⢮⢯⣨⣩⣪⣫⣬⣭⣮⣯", // 0b1101----
  "⢰⢱⢲⢳⢴⢵⢶⢷⣰⣱⣲⣳⣴⣵⣶⣷", // 0b1110----
  "⢸⢹⢺⢻⢼⢽⢾⢿⣸⣹⣺⣻⣼⣽⣾⣿", // 0b1111----
);

pub(crate) const PROGRESS_STYLE: &str = "{spinner:.green} ⟪{elapsed_precise}⟫ ⟦{wide_bar:.cyan}⟧ \
                                         {binary_bytes}/{binary_total_bytes} \
                                         ⟨{binary_bytes_per_sec}, {eta}⟩";

pub(crate) const VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));

pub(crate) const VERSION_MESSAGE: &str = "Print version number.";

#[cfg(test)]
mod tests {
  use super::*;
  use crate::common::*;

  #[test]
  fn created_by() {
    let pattern = Regex::new(
      r"(?x)
      imdl/
      [0-9]+.[0-9]+.[0-9]+(-.*)?
      (
        [\ ]
        \(
          [0-9a-f]{12}
        \)
      )?
    ",
    )
    .unwrap();

    assert!(
      pattern.is_match(CREATED_BY_DEFAULT),
      "Bad created by string: `{CREATED_BY_DEFAULT}`",
    );
  }
}
