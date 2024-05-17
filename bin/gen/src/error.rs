use crate::common::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub(crate) enum Error {
  #[snafu(display("Failed to deserialize `Cargo.toml`: {}", source))]
  CargoToml { source: cargo_toml::Error },
  #[snafu(display("Failed to decode command `{}` output: {}", command, source))]
  CommandDecode {
    command: String,
    source: FromUtf8Error,
  },
  #[snafu(display("Failed to invoke command `{}` output: {}", command, source))]
  CommandInvoke { command: String, source: io::Error },
  #[snafu(display("Command `{}` failed: {}", command, exit_status))]
  CommandStatus {
    command: String,
    exit_status: ExitStatus,
  },
  #[snafu(display("Failed to deserialize config from `{}`: {}", path.display(), source))]
  ConfigDeserialize {
    path: PathBuf,
    source: serde_yaml::Error,
  },
  #[snafu(display("Failed to get current dir: {}", source))]
  CurrentDir { source: io::Error },
  #[snafu(display(
    "Example commands `{}` don't match bin commands `{}`",
    example.iter().map(|command| command.deref()).collect::<Vec<&str>>().join(","),
    bin.iter().map(|command| command.deref()).collect::<Vec<&str>>().join(","),
  ))]
  ExampleCommands {
    example: BTreeSet<String>,
    bin: BTreeSet<String>,
  },
  #[snafu(display("I/O error at `{}`: {}", path.display(), source))]
  Filesystem { path: PathBuf, source: io::Error },
  #[snafu(display("I/O error copying `{}` to `{}`: {}", src.display(), dst.display(), source))]
  FilesystemRecursiveCopy {
    src: PathBuf,
    dst: PathBuf,
    source: fs_extra::error::Error,
  },
  #[snafu(display("Git error: {}", source))]
  Git { source: git2::Error },
  #[snafu(display("Regex compilation error: {}", source))]
  Regex { source: regex::Error },
  #[snafu(display("Failed to find Git repository from `{}`: {}", start_dir.display(), source))]
  RepositoryDiscover {
    start_dir: PathBuf,
    source: git2::Error,
  },
  #[snafu(display("Failed to create tempdir: {}", source))]
  Tempdir { source: io::Error },
  #[snafu(display("Failed to render template: {}", source))]
  TemplateRender { source: askama::Error },
}

impl From<regex::Error> for Error {
  fn from(source: regex::Error) -> Self {
    Self::Regex { source }
  }
}

impl From<git2::Error> for Error {
  fn from(source: git2::Error) -> Self {
    Self::Git { source }
  }
}

impl From<cargo_toml::Error> for Error {
  fn from(source: cargo_toml::Error) -> Self {
    Self::CargoToml { source }
  }
}
