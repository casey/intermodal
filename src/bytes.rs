use crate::common::*;

const KI: u128 = 1 << 10;
const MI: u128 = KI << 10;
const GI: u128 = MI << 10;
const TI: u128 = GI << 10;
const PI: u128 = TI << 10;
const EI: u128 = PI << 10;
const ZI: u128 = EI << 10;
const YI: u128 = ZI << 10;

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) struct Bytes(pub(crate) u128);

impl Bytes {
  pub(crate) fn is_power_of_two(self) -> bool {
    self.0 == 0 || self.0 & (self.0 - 1) == 0
  }
}

fn float_to_int(x: f64) -> u128 {
  #![allow(
    clippy::as_conversions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
  )]
  x as u128
}

fn int_to_float(x: u128) -> f64 {
  #![allow(clippy::as_conversions, clippy::cast_precision_loss)]
  x as f64
}

impl FromStr for Bytes {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn is_digit(c: &char) -> bool {
      match c {
        '0'..='9' | '.' => true,
        _ => false,
      }
    }

    let digits = text.chars().take_while(is_digit).collect::<String>();

    let suffix = text.chars().skip_while(is_digit).collect::<String>();

    let value = digits.parse::<f64>().map_err(|source| Error::ByteParse {
      text: text.to_owned(),
      source,
    })?;

    let multiple = match suffix.to_lowercase().as_str() {
      "" | "b" | "byte" | "bytes" => 1,
      "kib" => KI,
      "mib" => MI,
      "gib" => GI,
      "tib" => TI,
      "pib" => PI,
      "eib" => EI,
      "zib" => ZI,
      "yib" => YI,
      _ => {
        return Err(Error::ByteSuffix {
          text: text.to_owned(),
          suffix: suffix.to_owned(),
        })
      }
    };

    Ok(Bytes(float_to_int(value * int_to_float(multiple))))
  }
}

impl Display for Bytes {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    const DISPLAY_SUFFIXES: &[&str] = &["KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];

    let mut value = int_to_float(self.0);

    let mut i = 0;

    while value >= 1024.0 {
      value /= 1024.0;
      i += 1;
    }

    let suffix = if i == 0 {
      if value == 1.0 {
        "byte"
      } else {
        "bytes"
      }
    } else {
      DISPLAY_SUFFIXES[i - 1]
    };

    let formatted = format!("{:.2}", value);
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    write!(f, "{} {}", trimmed, suffix)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ok() {
    const CASES: &[(&str, u128)] = &[
      ("0", 0),
      ("0kib", 0),
      ("1", 1),
      ("1b", 1),
      ("1byte", 1),
      ("1bytes", 1),
      ("1kib", KI),
      ("1KiB", KI),
      ("12kib", 12 * KI),
      ("1.5mib", 1 * MI + 512 * KI),
      ("1yib", 1 * YI),
    ];

    for (text, value) in CASES {
      assert_eq!(
        text.parse::<Bytes>().unwrap(),
        Bytes(*value),
        "text: {}",
        text
      );
    }
  }

  #[test]
  fn err() {
    assert_matches!(
      "100foo".parse::<Bytes>().unwrap_err(),
      Error::ByteSuffix { text, suffix }
      if text == "100foo" && suffix == "foo"
    );

    assert_matches!(
      "1.0.0foo".parse::<Bytes>().unwrap_err(),
      Error::ByteParse { .. }
    );
  }

  #[test]
  fn display() {
    assert_eq!(Bytes(0).to_string(), "0 bytes");
    assert_eq!(Bytes(1).to_string(), "1 byte");
    assert_eq!(Bytes(2).to_string(), "2 bytes");
    assert_eq!(Bytes(KI).to_string(), "1 KiB");
    assert_eq!(Bytes(512 * KI).to_string(), "512 KiB");
    assert_eq!(Bytes(MI).to_string(), "1 MiB");
    assert_eq!(Bytes(MI + 512 * KI).to_string(), "1.5 MiB");
    assert_eq!(Bytes(1024 * MI + 512 * MI).to_string(), "1.5 GiB");
    assert_eq!(Bytes(GI).to_string(), "1 GiB");
    assert_eq!(Bytes(TI).to_string(), "1 TiB");
    assert_eq!(Bytes(PI).to_string(), "1 PiB");
    assert_eq!(Bytes(EI).to_string(), "1 EiB");
    assert_eq!(Bytes(ZI).to_string(), "1 ZiB");
    assert_eq!(Bytes(YI).to_string(), "1 YiB");
  }
}
