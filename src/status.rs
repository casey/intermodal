use crate::common::*;

#[derive(Debug)]
pub(crate) struct Status {
  pieces: bool,
  files: Vec<FileStatus>,
}

impl Status {
  pub(crate) fn new(pieces: bool, files: Vec<FileStatus>) -> Status {
    Status { pieces, files }
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

  // pub(crate) fn write(&self, out: &mut Env) -> Result<()> {
  //   for file in &self.files {
  //     errln!(out, "{} {}", file.icon(), file.path().display());
  //   }

  //   if !self.pieces() {
  //     errln!(out, "Piece hashes incorrect");
  //   }

  //   Ok(())
  // }
  //
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
