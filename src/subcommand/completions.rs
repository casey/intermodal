use crate::common::*;

const SHELL_FLAG: &str = "shell-flag";

const SHELL_POSITIONAL: &str = "<SHELL>";

const SHELL_HELP: &str = "Print completion script for `SHELL`.";

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Print shell completion scripts to standard output.")
)]
pub(crate) struct Completions {
  #[structopt(
    name = SHELL_FLAG,
    long = "shell",
    short = "s",
    value_name = "SHELL",
    possible_values = Shell::VARIANTS,
    help = SHELL_HELP,
  )]
  shell_flag: Option<Shell>,
  #[structopt(
    name = SHELL_POSITIONAL,
    value_name = "SHELL",
    possible_values = Shell::VARIANTS,
    required_unless = "dir",
    required_unless = SHELL_FLAG,
    conflicts_with = SHELL_FLAG,
    help = SHELL_HELP,
  )]
  shell_positional: Option<Shell>,
  #[structopt(
    long = "dir",
    short = "d",
    value_name = "DIR",
    empty_values = false,
    parse(from_os_str),
    help = "Write completion script to `DIR` with an appropriate filename. If `--shell` is not \
            given, write all completion scripts."
  )]
  dir: Option<PathBuf>,
}

impl Completions {
  pub(crate) fn run(self, env: &mut Env) -> Result<()> {
    if self.shell_flag.is_some() || self.shell_positional.is_some() {
      let shell = xor_args(
        "shell_flag",
        self.shell_flag.as_ref(),
        "shell_positional",
        self.shell_positional.as_ref(),
      )?;

      if let Some(dir) = self.dir {
        Self::write(env, &dir, shell)?;
      } else {
        let script = shell.completion_script()?;
        out!(env, "{}", script)?;
      }
    } else {
      let dir = self
        .dir
        .ok_or_else(|| Error::internal("Expected `--dir` to be set"))?;

      for shell in Shell::iter() {
        Self::write(env, &dir, shell)?;
      }
    }

    Ok(())
  }

  fn write(env: &mut Env, dir: &Path, shell: Shell) -> Result<()> {
    let script = shell.completion_script()?;
    let dst = dir.join(shell.completion_script_filename());
    env.write(dst, script)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn shell_required() {
    let mut env = test_env! {
      args: [
        "completions",
      ],
      tree: {},
    };

    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn output() {
    let mut env = test_env! {
      args: [
        "completions",
        "--shell",
        "bash",
      ],
      tree: {},
    };

    env.assert_ok();

    assert!(env.out().starts_with("_imdl() {"));
  }

  #[test]
  fn output_positional() {
    let mut env = test_env! {
      args: [
        "completions",
        "bash",
      ],
      tree: {},
    };

    env.assert_ok();

    assert!(env.out().starts_with("_imdl() {"));
  }

  #[test]
  fn single_dir() {
    let mut env = test_env! {
      args: [
        "completions",
        "--shell",
        "bash",
        "--dir",
        ".",
      ],
      tree: {},
    };

    env.assert_ok();

    let script = env.read_to_string("imdl.bash");

    assert!(script.starts_with("_imdl() {"));
  }

  #[test]
  fn single_positional() {
    let mut env = test_env! {
      args: [
        "completions",
        "bash",
        "--dir",
        ".",
      ],
      tree: {},
    };

    env.assert_ok();

    let script = env.read_to_string("imdl.bash");

    assert!(script.starts_with("_imdl() {"));
  }

  #[test]
  fn all_dir() {
    let mut env = test_env! {
      args: [
        "completions",
        "--dir",
        ".",
      ],
      tree: {},
    };

    env.assert_ok();

    let script = env.read_to_string("imdl.bash");
    assert!(script.starts_with("_imdl() {"));

    let script = env.read_to_string("_imdl.ps1");
    assert!(script.starts_with("using namespace"));
  }
}
