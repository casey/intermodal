use super::*;

use structopt::clap;

#[derive(EnumVariantNames, EnumString)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum Shell {
  Zsh,
  Bash,
  Fish,
  Powershell,
  Elvish,
}

impl Into<clap::Shell> for Shell {
  fn into(self) -> clap::Shell {
    match self {
      Self::Bash => clap::Shell::Bash,
      Self::Fish => clap::Shell::Fish,
      Self::Zsh => clap::Shell::Zsh,
      Self::Powershell => clap::Shell::PowerShell,
      Self::Elvish => clap::Shell::Elvish,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn variants() {
    assert_eq!(Shell::VARIANTS, clap::Shell::variants())
  }
}
