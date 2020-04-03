use crate::common::*;

pub(crate) struct Hasher {
  buffer: Vec<u8>,
  length: u64,
  md5sum: bool,
  piece_bytes_hashed: usize,
  piece_length: usize,
  pieces: PieceList,
  sha1: Sha1,
  progress_bar: Option<ProgressBar>,
}

impl Hasher {
  pub(crate) fn hash(
    files: &Files,
    md5sum: bool,
    piece_length: usize,
    progress_bar: Option<ProgressBar>,
  ) -> Result<(Mode, PieceList), Error> {
    Self::new(md5sum, piece_length, progress_bar).hash_files(files)
  }

  fn new(md5sum: bool, piece_length: usize, progress_bar: Option<ProgressBar>) -> Self {
    Self {
      buffer: vec![0; piece_length],
      length: 0,
      piece_bytes_hashed: 0,
      pieces: PieceList::new(),
      sha1: Sha1::new(),
      piece_length,
      md5sum,
      progress_bar,
    }
  }

  fn hash_files(mut self, files: &Files) -> Result<(Mode, PieceList), Error> {
    let mode = if let Some(contents) = files.contents() {
      let files = self.hash_contents(&files.root(), contents)?;

      Mode::Multiple { files }
    } else {
      let (md5sum, length) = self.hash_file(files.root())?;

      Mode::Single {
        md5sum: md5sum.map(|md5sum| md5sum.into()),
        length,
      }
    };

    if self.piece_bytes_hashed > 0 {
      self.pieces.push(self.sha1.digest().into());
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
        md5sum: md5sum.map(|md5sum| md5sum.into()),
        path: file_path.clone(),
        length,
      });
    }

    Ok(files)
  }

  fn hash_file(&mut self, file: &Path) -> Result<(Option<md5::Digest>, Bytes), Error> {
    self
      .hash_file_io(file)
      .context(error::Filesystem { path: file })
  }

  fn hash_file_io(&mut self, file: &Path) -> io::Result<(Option<md5::Digest>, Bytes)> {
    let stdin = io::stdin();
    let buffer_len = self.buffer.len();
    let mut bytes_length = 0;
    let mut should_hash = true;

    let mut file: Box<dyn Read> = if file == PathBuf::from("-") {
      Box::new(stdin.lock())
    } else {
      Box::new(File::open(file)?)
    };

    let mut md5 = if self.md5sum {
      Some(md5::Context::new())
    } else {
      None
    };

    while should_hash {
      let buffer = &mut self.buffer[0..buffer_len];

      let read_length = file.read(buffer)?;
      bytes_length += read_length;
      should_hash = read_length == 0;

      for index in 0..read_length {
        let byte = buffer[index];
        self.sha1.update(&[byte]);

        self.piece_bytes_hashed += 1;

        if self.piece_bytes_hashed == self.piece_length {
          self.pieces.push(self.sha1.digest().into());
          self.sha1.reset();
          self.piece_bytes_hashed = 0;
        }
      }

      if let Some(md5) = md5.as_mut() {
        md5.consume(&buffer);
      }

      if let Some(progress_bar) = &self.progress_bar {
        progress_bar.inc(read_length.into_u64());
      }
    }

    self.length += bytes_length.into_u64();

    Ok((
      md5.map(md5::Context::compute),
      Bytes::from(bytes_length.into_u64()),
    ))
  }
}
