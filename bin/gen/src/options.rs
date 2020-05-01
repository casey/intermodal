use crate::common::*;

#[derive(StructOpt)]
pub(crate) struct Options {
  #[structopt(
    long("bin"),
    value_name("EXECUTABLE"),
    help("Path to the `imdl` binary.")
  )]
  pub(crate) bin: PathBuf,
}
