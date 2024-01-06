use crate::common::*;

const KI: u64 = 1 << 10;
const MI: u64 = KI << 10;
const GI: u64 = MI << 10;
const TI: u64 = GI << 10;
const PI: u64 = TI << 10;
const EI: u64 = PI << 10;

#[derive(Debug, PartialEq, Copy, Clone, PartialOrd, Ord, Eq, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub(crate) struct Bytes(pub(crate) u64);

impl Bytes {
  pub(crate) fn kib() -> Self {
    Bytes::from(KI)
  }

  pub(crate) fn mib() -> Self {
    Bytes::from(MI)
  }

  pub(crate) fn count(self) -> u64 {
    self.0
  }

  pub(crate) fn as_piece_length(self) -> Result<u32> {
    self
      .count()
      .try_into()
      .context(error::PieceLengthTooLarge { bytes: self })
  }

  pub(crate) fn absolute_difference(self, other: Bytes) -> Bytes {
    if self > other {
      self - other
    } else {
      other - self
    }
  }
}

fn float_to_int(x: f64) -> u64 {
  #![allow(
    clippy::as_conversions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
  )]
  x as u64
}

fn int_to_float(x: u64) -> f64 {
  #![allow(clippy::as_conversions, clippy::cast_precision_loss)]
  x as f64
}

impl<I: Into<u64>> From<I> for Bytes {
  fn from(n: I) -> Bytes {
    Bytes(n.into())
  }
}

impl FromStr for Bytes {
  type Err = Error;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    #[allow(clippy::trivially_copy_pass_by_ref)]
    fn is_digit(c: &char) -> bool {
      matches!(c, '0'..='9' | '.')
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
      _ => {
        return Err(Error::ByteSuffix {
          text: text.to_owned(),
          suffix: suffix.clone(),
        })
      }
    };

    Ok(Bytes(float_to_int(value * int_to_float(multiple))))
  }
}

impl Div<Bytes> for Bytes {
  type Output = u64;

  fn div(self, rhs: Bytes) -> u64 {
    self.0 / rhs.0
  }
}

impl Sub<Bytes> for Bytes {
  type Output = Bytes;

  fn sub(self, rhs: Bytes) -> Bytes {
    Bytes(self.count() - rhs.count())
  }
}

impl Div<u64> for Bytes {
  type Output = Bytes;

  fn div(self, rhs: u64) -> Bytes {
    Bytes::from(self.0 / rhs)
  }
}

impl Mul<u64> for Bytes {
  type Output = Bytes;

  fn mul(self, rhs: u64) -> Self {
    Bytes::from(self.0 * rhs)
  }
}

impl DivAssign<u64> for Bytes {
  fn div_assign(&mut self, rhs: u64) {
    self.0 /= rhs;
  }
}

impl MulAssign<u64> for Bytes {
  fn mul_assign(&mut self, rhs: u64) {
    self.0 *= rhs;
  }
}

impl AddAssign<Bytes> for Bytes {
  fn add_assign(&mut self, rhs: Bytes) {
    self.0 += rhs.0;
  }
}

impl SubAssign<u64> for Bytes {
  fn sub_assign(&mut self, rhs: u64) {
    self.0 -= rhs;
  }
}

impl Sum for Bytes {
  fn sum<I>(iter: I) -> Self
  where
    I: Iterator<Item = Self>,
  {
    let mut sum = Bytes(0);

    for item in iter {
      sum += item;
    }

    sum
  }
}

impl Display for Bytes {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    const DISPLAY_SUFFIXES: &[&str] = &["KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

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

    let formatted = format!("{value:.2}");
    let trimmed = formatted.trim_end_matches('0').trim_end_matches('.');
    write!(f, "{trimmed} {suffix}")
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn ok() {
    const CASES: &[(&str, u64)] = &[
      ("0", 0),
      ("0kib", 0),
      ("1", 1),
      ("1b", 1),
      ("1byte", 1),
      ("1bytes", 1),
      ("1kib", KI),
      ("1KiB", KI),
      ("12kib", 12 * KI),
      ("1.5mib", MI + 512 * KI),
    ];

    for (text, value) in CASES {
      assert_eq!(
        text.parse::<Bytes>().unwrap(),
        Bytes(*value),
        "text: {text}",
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
  }

  #[test]
  fn bencode() {
    assert_eq!(
      bendy::serde::ser::to_bytes(&Bytes::kib()).unwrap(),
      b"i1024e"
    );
    assert_eq!(
      Bytes::kib(),
      bendy::serde::de::from_bytes(b"i1024e").unwrap(),
    );
  }
}
