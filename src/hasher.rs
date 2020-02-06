use crate::common::*;

pub(crate) struct Hasher {
  buffer: Vec<u8>,
  length: u64,
  md5sum: bool,
  piece_bytes_hashed: u64,
  piece_length: u32,
  pieces: Vec<u8>,
  sha1: Sha1,
}

impl Hasher {
  pub(crate) fn hash(
    files: &Files,
    md5sum: bool,
    piece_length: u32,
  ) -> Result<(Mode, Vec<u8>), Error> {
    Self::new(md5sum, piece_length).hash_files(files)
  }

  fn new(md5sum: bool, piece_length: u32) -> Self {
    Self {
      buffer: vec![0; piece_length.into_usize()],
      length: 0,
      piece_bytes_hashed: 0,
      pieces: Vec::new(),
      sha1: Sha1::new(),
      md5sum,
      piece_length,
    }
  }

  fn hash_files(mut self, files: &Files) -> Result<(Mode, Vec<u8>), Error> {
    let mode = if let Some(contents) = files.contents() {
      let files = self.hash_contents(&files.root(), contents)?;

      Mode::Multiple { files }
    } else {
      let (md5sum, length) = self.hash_file(files.root())?;

      Mode::Single {
        md5sum: md5sum.map(|md5sum| format!("{:x}", md5sum)),
        length,
      }
    };

    if self.piece_bytes_hashed > 0 {
      self.pieces.extend(&self.sha1.digest().bytes());
      self.sha1.reset();
      self.piece_bytes_hashed = 0;
    }

    Ok((mode, self.pieces))
  }

  fn hash_contents(
    &mut self,
    root: &Path,
    file_paths: &[FilePath],
  ) -> Result<Vec<FileInfo>, Error> {
    let mut files = Vec::new();

    for file_path in file_paths {
      let path = file_path.absolute(root);

      let (md5sum, length) = self.hash_file(&path)?;

      files.push(FileInfo {
        md5sum: md5sum.map(|md5sum| format!("{:x}", md5sum)),
        path: file_path.clone(),
        length,
      });
    }

    Ok(files)
  }

  fn hash_file(&mut self, file: &Path) -> Result<(Option<md5::Digest>, u64), Error> {
    self
      .hash_file_io(file)
      .context(error::Filesystem { path: file })
  }

  fn hash_file_io(&mut self, file: &Path) -> io::Result<(Option<md5::Digest>, u64)> {
    let length = file.metadata()?.len();

    let mut remaining = length;

    let mut file = File::open(file)?;

    let mut md5 = if self.md5sum {
      Some(md5::Context::new())
    } else {
      None
    };

    while remaining > 0 {
      let to_buffer: usize = remaining
        .min(self.buffer.len().into_u64())
        .try_into()
        .unwrap();
      let buffer = &mut self.buffer[0..to_buffer];

      file.read_exact(buffer)?;

      for byte in buffer.iter().cloned() {
        self.sha1.update(&[byte]);

        self.piece_bytes_hashed += 1;

        if self.piece_bytes_hashed == self.piece_length.into() {
          self.pieces.extend(&self.sha1.digest().bytes());
          self.sha1.reset();
          self.piece_bytes_hashed = 0;
        }
      }

      if let Some(md5) = md5.as_mut() {
        md5.consume(&buffer);
      }

      remaining -= buffer.len().into_u64();
    }

    self.length += length;

    Ok((md5.map(md5::Context::compute), length))
  }
}
