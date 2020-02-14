use crate::common::*;

pub(crate) struct Verifier {
  buffer: Vec<u8>,
  piece_length: usize,
  pieces: Vec<u8>,
  sha1: Sha1,
  piece_bytes_hashed: usize,
}

impl Verifier {
  pub(crate) fn new(piece_length: usize) -> Verifier {
    Verifier {
      buffer: vec![0; piece_length],
      piece_bytes_hashed: 0,
      sha1: Sha1::new(),
      pieces: Vec::new(),
      piece_length,
    }
  }

  pub(crate) fn verify(metainfo: &Metainfo, base: &Path) -> Result<Status> {
    let piece_length = metainfo.info.piece_length.as_piece_length()?;

    let piece_length = piece_length.into_usize();

    let mut status = Vec::new();

    let mut hasher = Self::new(piece_length);

    for (path, len, md5sum) in metainfo.files(&base) {
      status.push(FileStatus::status(&path, len, md5sum));
      hasher.hash(&path).ok();
    }

    if hasher.piece_bytes_hashed > 0 {
      hasher.pieces.extend(&hasher.sha1.digest().bytes());
      hasher.sha1.reset();
      hasher.piece_bytes_hashed = 0;
    }

    let pieces = hasher.pieces == metainfo.info.pieces;

    Ok(Status::new(pieces, status))
  }

  pub(crate) fn hash(&mut self, path: &Path) -> io::Result<()> {
    let mut file = File::open(path)?;

    let mut remaining = path.metadata()?.len();

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

        if self.piece_bytes_hashed == self.piece_length {
          self.pieces.extend(&self.sha1.digest().bytes());
          self.sha1.reset();
          self.piece_bytes_hashed = 0;
        }
      }

      remaining -= buffer.len().into_u64();
    }

    Ok(())
  }
}
