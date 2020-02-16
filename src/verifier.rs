use crate::common::*;

pub(crate) struct Verifier<'a> {
  metainfo: &'a Metainfo,
  base: &'a Path,
  buffer: Vec<u8>,
  piece_length: usize,
  pieces: PieceList,
  sha1: Sha1,
  piece_bytes_hashed: usize,
}

// enum PieceStatus {
//   Good,
//   Bad,
//   Missing,
// }

// match
// mismatch

impl<'a> Verifier<'a> {
  fn new(metainfo: &'a Metainfo, base: &'a Path) -> Result<Verifier<'a>> {
    let piece_length = metainfo.info.piece_length.as_piece_length()?.into_usize();

    Ok(Verifier {
      buffer: vec![0; piece_length],
      piece_bytes_hashed: 0,
      pieces: PieceList::new(),
      sha1: Sha1::new(),
      base,
      metainfo,
      piece_length,
    })
  }

  pub(crate) fn verify(metainfo: &'a Metainfo, base: &'a Path) -> Result<Status> {
    Self::new(metainfo, base)?.verify_metainfo()
  }

  fn verify_metainfo(mut self) -> Result<Status> {
    // let mut expected_piece_count =
    //   self.metainfo.content_size().count() / self.metainfo.info.piece_length.count();

    // if self.metainfo.content_size().count() % self.metainfo.info.piece_length.count() != 0 {
    //   expected_piece_count += 1;
    // }

    let mut status = Vec::new();

    for (path, len, md5sum) in self.metainfo.files(&self.base) {
      status.push(FileStatus::status(&path, len, md5sum));
      self.hash(&path).ok();
    }

    if self.piece_bytes_hashed > 0 {
      self.pieces.push(self.sha1.digest().into());
      self.sha1.reset();
      self.piece_bytes_hashed = 0;
    }

    let pieces = self.pieces == self.metainfo.info.pieces;

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
          self.pieces.push(self.sha1.digest().into());
          self.sha1.reset();
          self.piece_bytes_hashed = 0;
        }
      }

      remaining -= buffer.len().into_u64();
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn good() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "https://bar",
      ],
      tree: {
        foo: {
          a: "abc",
          d: "efg",
          h: "ijk",
        },
      },
    };

    env.run()?;

    let metainfo = env.load_metainfo("foo.torrent");

    assert!(metainfo.verify(&env.resolve("foo"))?.good());

    Ok(())
  }

  #[test]
  fn piece_mismatch() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "https://bar",
      ],
      tree: {
        foo: {
          a: "abc",
          d: "efg",
          h: "ijk",
        },
      },
    };

    env.run()?;

    env.write("foo/a", "xyz");

    let metainfo = env.load_metainfo("foo.torrent");

    let status = metainfo.verify(&env.resolve("foo"))?;

    assert!(status.files().iter().all(FileStatus::good));

    assert!(!status.pieces());

    Ok(())
  }
}
