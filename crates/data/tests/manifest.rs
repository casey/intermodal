#[cfg(aspirational)]
mod aspirational {

  #[data::structure]
  struct Manifest {
    magic_number: [u8; 8], // IMDL + 4 bytes that aren't valid UTF-8
    version: Version,      // 256.256.256
  }

  #[data::structure]
  struct Version {
    major: u64,
    minor: u64,
    patch: u64,
  }

  #[data::table]
  struct Manifest;

  #[data::table_impl]
  impl Manifest {
    fn files(&self) -> Slice<Hash>;

    fn directory(&self) -> Directory;
  }

  #[data::structure]
  struct Directory {
    records: Slice<Record>,
  }

  #[data::structure]
  struct Record {
    name: &str,
    node: Node,
  }

  #[data::enumeration]
  enum Node {
    Directory(Directory),
    File(u64),
  }

  #[data::structure]
  struct Hash {
    bytes: [u8; 32],
  }
}
