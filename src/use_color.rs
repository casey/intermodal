use crate::common::*;

pub(crate) const AUTO: &str = "auto";
pub(crate) const ALWAYS: &str = "always";
pub(crate) const NEVER: &str = "never";

pub(crate) const VALUES: &[&str] = &[AUTO, ALWAYS, NEVER];

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum UseColor {
  Auto,
  Always,
  Never,
}

impl FromStr for UseColor {
  type Err = Error;
  fn from_str(text: &str) -> Result<Self, Self::Err> {
    match text.to_lowercase().as_str() {
      AUTO => Ok(UseColor::Auto),
      ALWAYS => Ok(UseColor::Always),
      NEVER => Ok(UseColor::Never),
      _ => unreachable!(),
    }
  }
}
