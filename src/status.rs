use crate::common::*;

#[derive(Debug)]
pub(crate) struct Status {
  pieces: bool,
  files: Vec<FileStatus>,
}

impl Status {
  pub(crate) fn new(pieces: bool, files: Vec<FileStatus>) -> Self {
    Self { pieces, files }
  }

  pub(crate) fn pieces(&self) -> bool {
    self.pieces
  }

  #[cfg(test)]
  pub(crate) fn files(&self) -> &[FileStatus] {
    &self.files
  }

  pub(crate) fn good(&self) -> bool {
    self.pieces && self.files.iter().all(FileStatus::good)
  }
}

impl Display for Status {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let bad = self.files.iter().filter(|status| status.bad()).count();

    if bad != 0 {
      write!(f, "{} of {} files corrupted", bad, self.files.len())?;
      return Ok(());
    }

    if !self.pieces() {
      write!(f, "pieces corrupted")?;
      return Ok(());
    }

    write!(f, "ok")?;

    Ok(())
  }
}
