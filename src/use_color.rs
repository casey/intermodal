use crate::common::*;

#[derive(Copy, Clone, Debug, PartialEq, EnumVariantNames, EnumString, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum UseColor {
  Auto,
  Always,
  Never,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn variants() {
    assert_eq!(UseColor::VARIANTS, &["auto", "always", "never"]);
  }

  #[test]
  fn from_str() {
    assert_eq!(UseColor::Auto, "auto".parse().unwrap());
    assert_eq!(UseColor::Always, "always".parse().unwrap());
    assert_eq!(UseColor::Never, "never".parse().unwrap());
  }
}
