use crate::common::*;

use structopt::clap;

#[derive(Copy, Clone, EnumVariantNames, IntoStaticStr, EnumString, EnumIter, Debug)]
#[strum(serialize_all = "kebab-case")]
pub(crate) enum Shell {
  Zsh,
  Bash,
  Fish,
  Powershell,
  Elvish,
}

impl Shell {
  pub(crate) fn completion_script(self) -> Result<String> {
    let buffer = Vec::new();
    let mut cursor = Cursor::new(buffer);

    Arguments::clap().gen_completions_to(env!("CARGO_PKG_NAME"), self.into(), &mut cursor);

    let buffer = cursor.into_inner();

    let script = String::from_utf8(buffer).context(error::ShellDecode { shell: self })?;

    let mut script = script.trim().to_owned();
    script.push('\n');

    Ok(script)
  }

  pub(crate) fn completion_script_filename(self) -> &'static str {
    match self {
      Self::Bash => "imdl.bash",
      Self::Fish => "imdl.fish",
      Self::Zsh => "_imdl",
      Self::Powershell => "_imdl.ps1",
      Self::Elvish => "imdl.elvish",
    }
  }

  pub(crate) fn name(self) -> &'static str {
    self.into()
  }
}

impl From<Shell> for clap::Shell {
  fn from(shell: Shell) -> Self {
    match shell {
      Shell::Bash => clap::Shell::Bash,
      Shell::Fish => clap::Shell::Fish,
      Shell::Zsh => clap::Shell::Zsh,
      Shell::Powershell => clap::Shell::PowerShell,
      Shell::Elvish => clap::Shell::Elvish,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn variants() {
    assert_eq!(Shell::VARIANTS, clap::Shell::variants());
  }
}
