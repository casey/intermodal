use crate::common::*;

pub(crate) struct Verifier {
  buffer: Vec<u8>,
  piece_length: Bytes,
  pieces: Vec<u8>,
  poisoned: bool,
  sha1: Sha1,
  piece_bytes_hashed: usize,
}

impl Verifier {
  pub(crate) fn new(piece_length: Bytes) -> Verifier {
    Verifier {
      buffer: vec![0; piece_length.count() as usize],
      piece_bytes_hashed: 0,
      sha1: Sha1::new(),
      poisoned: false,
      pieces: Vec::new(),
      piece_length,
    }
  }

  pub(crate) fn verify(metainfo: &Metainfo, base: &Path) -> Status {
    let mut status = Vec::new();

    let mut hasher = Self::new(metainfo.info.piece_length);

    for (path, len, md5sum) in metainfo.files(&base) {
      status.push(FileStatus::status(&path, len, md5sum));
      hasher.hash(&path, len);
    }

    if hasher.piece_bytes_hashed > 0 {
      hasher.pieces.extend(&hasher.sha1.digest().bytes());
      hasher.sha1.reset();
      hasher.piece_bytes_hashed = 0;
    }

    let pieces = hasher.pieces == metainfo.info.pieces;

    Status::new(pieces, status)
  }

  pub(crate) fn hash(&mut self, path: &Path, expected_len: Bytes) -> io::Result<()> {
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

        if Bytes::from(self.piece_bytes_hashed.into_u64()) == self.piece_length {
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
