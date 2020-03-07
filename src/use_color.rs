use crate::common::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum UseColor {
  Auto,
  Always,
  Never,
}

impl UseColor {
  pub(crate) const ALWAYS: &'static str = "always";
  pub(crate) const AUTO: &'static str = "auto";
  pub(crate) const NEVER: &'static str = "never";
  pub(crate) const VALUES: &'static [&'static str] = &[Self::AUTO, Self::ALWAYS, Self::NEVER];
}

impl FromStr for UseColor {
  type Err = Infallible;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text.to_lowercase().as_str() {
      Self::AUTO => Ok(Self::Auto),
      Self::ALWAYS => Ok(Self::Always),
      Self::NEVER => Ok(Self::Never),
      _ => unreachable!(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_str() {
    assert_eq!(UseColor::Auto, UseColor::AUTO.parse().unwrap());
    assert_eq!(UseColor::Always, UseColor::ALWAYS.parse().unwrap());
    assert_eq!(UseColor::Never, UseColor::NEVER.parse().unwrap());
    assert_eq!(
      UseColor::Auto,
      UseColor::AUTO.to_uppercase().parse().unwrap()
    );
    assert_eq!(
      UseColor::Always,
      UseColor::ALWAYS.to_uppercase().parse().unwrap()
    );
    assert_eq!(
      UseColor::Never,
      UseColor::NEVER.to_uppercase().parse().unwrap()
    );
  }
}
