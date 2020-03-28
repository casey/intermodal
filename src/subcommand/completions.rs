use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Print shell completion scripts to standard output.")
)]
pub(crate) struct Completions {
  #[structopt(
    long = "shell",
    short = "s",
    value_name = "SHELL",
    possible_values = Shell::VARIANTS,
    help = "Print completions for `SHELL`.",
  )]
  shell: Shell,
}

impl Completions {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let buffer = Vec::new();
    let mut cursor = Cursor::new(buffer);

    Arguments::clap().gen_completions_to(env!("CARGO_PKG_NAME"), self.shell.into(), &mut cursor);

    let buffer = cursor.into_inner();

    let script = String::from_utf8(buffer).expect("Clap completion not UTF-8");

    outln!(env, "{}", script.trim())?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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

    assert_matches!(env.run(), Ok(()));

    assert!(env.out().starts_with("_imdl() {"));
  }
}
