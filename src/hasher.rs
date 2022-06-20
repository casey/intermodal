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
  pub(crate) fn new(md5sum: bool, piece_length: usize, progress_bar: Option<ProgressBar>) -> Self {
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

  pub(crate) fn hash_files(mut self, files: &Files) -> Result<(Mode, PieceList), Error> {
    let mode = if let Some(contents) = files.contents() {
      let files = self.hash_contents(files.root(), contents)?;

      Mode::Multiple { files }
    } else {
      let (md5sum, length) = self.hash_file(files.root())?;

      Mode::Single { md5sum, length }
    };

    self.finish();

    Ok((mode, self.pieces))
  }

  pub(crate) fn hash_stdin(mut self, stdin: &mut dyn BufRead) -> Result<(Mode, PieceList), Error> {
    let (md5sum, length) = self.hash_read_io(stdin).context(error::Stdin)?;

    let mode = Mode::Single { md5sum, length };

    self.finish();

    Ok((mode, self.pieces))
  }

  fn finish(&mut self) {
    if self.piece_bytes_hashed > 0 {
      self.pieces.push(self.sha1.digest().into());
      self.sha1.reset();
      self.piece_bytes_hashed = 0;
    }
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
        path: file_path.clone(),
        md5sum,
        length,
      });
    }

    Ok(files)
  }

  fn hash_file(&mut self, path: &Path) -> Result<(Option<Md5Digest>, Bytes), Error> {
    let file = File::open(path).context(error::Filesystem { path })?;

    self
      .hash_read_io(&mut BufReader::new(file))
      .context(error::Filesystem { path })
  }

  fn hash_read_io(&mut self, file: &mut dyn BufRead) -> io::Result<(Option<Md5Digest>, Bytes)> {
    let mut bytes_hashed = 0;

    let mut md5 = if self.md5sum {
      Some(md5::Context::new())
    } else {
      None
    };

    loop {
      let remaining = &mut self.buffer[..self.piece_length - self.piece_bytes_hashed];

      let bytes_read = file.read(remaining)?;

      if bytes_read == 0 {
        break;
      }

      let read = &remaining[..bytes_read];

      self.sha1.update(read);

      bytes_hashed += bytes_read;
      self.piece_bytes_hashed += bytes_read;

      if self.piece_bytes_hashed == self.piece_length {
        self.pieces.push(self.sha1.digest().into());
        self.sha1.reset();
        self.piece_bytes_hashed = 0;
      }

      if let Some(md5) = md5.as_mut() {
        md5.consume(read);
      }

      if let Some(progress_bar) = &self.progress_bar {
        progress_bar.inc(bytes_read.into_u64());
      }
    }

    self.length += bytes_hashed.into_u64();

    Ok((
      md5.map(|context| context.compute().into()),
      Bytes::from(bytes_hashed.into_u64()),
    ))
  }
}
