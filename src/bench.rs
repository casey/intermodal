#![allow(clippy::unwrap_used)]

use crate::common::*;

use std::io::BufWriter;

use rand::RngCore;
use tempfile::{Builder, NamedTempFile};

pub const TEMPFILE_BYTES: u64 = 256 << 10;

pub trait Bench {
  fn init() -> Self;

  fn name(&self) -> String;

  fn iteration(&self);
}

pub struct HasherBench {
  tempfile: NamedTempFile,
}

impl Bench for HasherBench {
  fn name(&self) -> String {
    format!("bench::HasherBench: {} tempfile", Bytes(TEMPFILE_BYTES))
  }

  fn init() -> Self {
    let mut tempfile = Builder::new()
      .prefix("imdl-bench-hasher")
      .tempfile()
      .unwrap();

    {
      let mut bytes = vec![0; 1024];

      let mut written = 0;

      let mut writer = BufWriter::new(&mut tempfile);

      while written < TEMPFILE_BYTES {
        rand::thread_rng().fill_bytes(&mut bytes);
        writer.write_all(&bytes).unwrap();
        written += bytes.len().into_u64();
      }

      writer.flush().unwrap();
    }

    Self { tempfile }
  }

  fn iteration(&self) {
    let files = Files::file(
      self.tempfile.as_ref().to_owned(),
      Bytes::from(TEMPFILE_BYTES),
    );

    let hasher = Hasher::new(false, 16 << 10, None);

    let _result = hasher.hash_files(&files).unwrap();
  }
}
