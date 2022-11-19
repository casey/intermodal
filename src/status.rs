use crate::common::*;

#[derive(Debug)]
pub(crate) enum Status {
  Single {
    pieces: bool,
    error: Option<FileError>,
  },
  Multiple {
    pieces: bool,
    files: Vec<FileStatus>,
  },
}

impl Status {
  pub(crate) fn single(pieces: bool, error: Option<FileError>) -> Self {
    Status::Single { pieces, error }
  }

  pub(crate) fn multiple(pieces: bool, files: Vec<FileStatus>) -> Self {
    Status::Multiple { pieces, files }
  }

  pub(crate) fn pieces(&self) -> bool {
    match self {
      Self::Single { pieces, .. } | Self::Multiple { pieces, .. } => *pieces,
    }
  }

  pub(crate) fn good(&self) -> bool {
    self.pieces()
      && match self {
        Self::Single { error, .. } => error.is_none(),
        Self::Multiple { files, .. } => files.iter().all(FileStatus::is_good),
      }
  }

  #[cfg(test)]
  pub(crate) fn count_bad(&self) -> usize {
    match self {
      Self::Single { error, .. } => error.is_some().into(),
      Self::Multiple { files, .. } => files.iter().filter(|file| file.is_bad()).count(),
    }
  }

  pub(crate) fn print(&self, env: &mut Env) -> Result<()> {
    match self {
      Self::Single { error, .. } => {
        if let Some(error) = error {
          error.println(env.err_mut()).context(error::Stderr)?;
        }
      }
      Self::Multiple { files, .. } => {
        for file in files {
          if let Some(error) = file.error() {
            let style = env.err().style();
            err!(
              env,
              "{}{}:{} ",
              style.message().prefix(),
              file.path(),
              style.message().suffix(),
            )?;
            error.println(env.err_mut()).context(error::Stderr)?;
          }
        }
      }
    }

    if !self.pieces() {
      errln!(env, "Pieces corrupted.")?;
    }

    Ok(())
  }
}
