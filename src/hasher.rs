use crate::common::*;

pub(crate) struct Hasher {
  buffer: Vec<u8>,
  length: u64,
  md5sum: bool,
  piece_bytes_hashed: u64,
  piece_length: u64,
  pieces: Vec<u8>,
  sha1: Sha1,
}

impl Hasher {
  pub(crate) fn hash(
    root: &Path,
    md5sum: bool,
    piece_length: u64,
  ) -> Result<(Mode, Vec<u8>), Error> {
    Hasher::new(md5sum, piece_length).hash_root(root)
  }

  fn new(md5sum: bool, piece_length: u64) -> Hasher {
    Hasher {
      buffer: vec![0; piece_length as usize],
      length: 0,
      piece_bytes_hashed: 0,
      pieces: Vec::new(),
      sha1: Sha1::new(),
      md5sum,
      piece_length,
    }
  }

  fn hash_root(mut self, root: &Path) -> Result<(Mode, Vec<u8>), Error> {
    let single = root
      .metadata()
      .context(error::Filesystem { path: root })?
      .is_file();

    if single {
      let (md5sum, length) = self.hash_file(&root)?;

      if self.piece_bytes_hashed > 0 {
        self.pieces.extend(&self.sha1.digest().bytes());
        self.sha1.reset();
        self.piece_bytes_hashed = 0;
      }

      Ok((
        Mode::Single {
          md5sum: md5sum.map(|md5sum| format!("{:x}", md5sum)),
          length,
        },
        self.pieces,
      ))
    } else {
      let files = self.hash_dir(root)?;

      if self.piece_bytes_hashed > 0 {
        self.pieces.extend(&self.sha1.digest().bytes());
        self.sha1.reset();
        self.piece_bytes_hashed = 0;
      }

      Ok((Mode::Multiple { files }, self.pieces))
    }
  }

  fn hash_dir(&mut self, dir: &Path) -> Result<Vec<FileInfo>, Error> {
    for result in WalkDir::new(dir).sort_by(|a, b| a.file_name().cmp(b.file_name())) {
      let entry = result?;

      let path = entry.path();

      if entry.metadata()?.is_file() {
        let (_md5sum, _length) = self.hash_file(path)?;
      }
    }

    Ok(Vec::new())
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
      let to_buffer = self.buffer.len().min(remaining as usize);
      let buffer = &mut self.buffer[0..to_buffer];

      file.read_exact(buffer)?;

      for byte in buffer.iter().cloned() {
        self.sha1.update(&[byte]);

        self.piece_bytes_hashed += 1;

        if self.piece_bytes_hashed == self.piece_length {
          self.pieces.extend(&self.sha1.digest().bytes());
          self.sha1.reset();
          self.piece_bytes_hashed = 0;
        }
      }

      if let Some(md5) = md5.as_mut() {
        md5.consume(&buffer);
      }

      remaining -= buffer.len() as u64;
    }

    self.length += length;

    Ok((md5.map(md5::Context::compute), length))
  }
}
