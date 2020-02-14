use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Display information about a `.torrent` file.")
)]
pub(crate) struct Show {
  #[structopt(
    name = "TORRENT",
    long = "input",
    help = "Show information about `TORRENT`.",
    parse(from_os_str)
  )]
  input: PathBuf,
}

impl Show {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let input = env.resolve(&self.input);
    let summary = TorrentSummary::load(&input)?;
    summary.write(env)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn output() {
    let metainfo = Metainfo {
      announce: "announce".into(),
      announce_list: Some(vec![vec!["announce".into(), "b".into()], vec!["c".into()]]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: vec![
          0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
        mode: Mode::Single {
          length: Bytes(20),
          md5sum: None,
        },
      },
    };

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent");

      metainfo.dump(path).unwrap();

      env.run().unwrap();

      let have = env.out();
      let want = "        Name  foo
     Comment  comment
     Created  1970-01-01 00:00:01 UTC
  Created By  created by
      Source  source
   Info Hash  b7595205a46491b3e8686e10b28efe7144d066cc
Torrent Size  252 bytes
Content Size  20 bytes
     Private  yes
    Trackers  Tier 1: announce
                      b
              Tier 2: c
  Piece Size  16 KiB
 Piece Count  1
  File Count  1
       Files  foo
";

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent");

      metainfo.dump(path).unwrap();

      env.run().unwrap();

      let have = env.out();
      let want = "\
Name\tfoo
Comment\tcomment
Created\t1970-01-01 00:00:01 UTC
Created By\tcreated by
Source\tsource
Info Hash\tb7595205a46491b3e8686e10b28efe7144d066cc
Torrent Size\t252
Content Size\t20
Private\tyes
Trackers\tannounce\tb\tc
Piece Size\t16384
Piece Count\t1
File Count\t1
Files\tfoo
";

      assert_eq!(have, want);
    }
  }

  #[test]
  fn tier_list_with_main() {
    let metainfo = Metainfo {
      announce: "a".into(),
      announce_list: Some(vec![vec!["x".into()], vec!["y".into()], vec!["z".into()]]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: vec![
          0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
        mode: Mode::Single {
          length: Bytes(20),
          md5sum: None,
        },
      },
    };

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent");

      metainfo.dump(path).unwrap();

      env.run().unwrap();

      let have = env.out();
      let want = "        Name  foo
     Comment  comment
     Created  1970-01-01 00:00:01 UTC
  Created By  created by
      Source  source
   Info Hash  b7595205a46491b3e8686e10b28efe7144d066cc
Torrent Size  240 bytes
Content Size  20 bytes
     Private  yes
    Trackers  a
              x
              y
              z
  Piece Size  16 KiB
 Piece Count  1
  File Count  1
       Files  foo
";

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent");

      metainfo.dump(path).unwrap();

      env.run().unwrap();

      let have = env.out();
      let want = "\
Name\tfoo
Comment\tcomment
Created\t1970-01-01 00:00:01 UTC
Created By\tcreated by
Source\tsource
Info Hash\tb7595205a46491b3e8686e10b28efe7144d066cc
Torrent Size\t240
Content Size\t20
Private\tyes
Trackers\ta\tx\ty\tz
Piece Size\t16384
Piece Count\t1
File Count\t1
Files\tfoo
";

      assert_eq!(have, want);
    }
  }

  #[test]
  fn tier_list_without_main() {
    let metainfo = Metainfo {
      announce: "a".into(),
      announce_list: Some(vec![vec!["b".into()], vec!["c".into()], vec!["a".into()]]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: vec![
          0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
        mode: Mode::Single {
          length: Bytes(20),
          md5sum: None,
        },
      },
    };

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent");

      metainfo.dump(path).unwrap();

      env.run().unwrap();

      let have = env.out();
      let want = "        Name  foo
     Comment  comment
     Created  1970-01-01 00:00:01 UTC
  Created By  created by
      Source  source
   Info Hash  b7595205a46491b3e8686e10b28efe7144d066cc
Torrent Size  240 bytes
Content Size  20 bytes
     Private  yes
    Trackers  b
              c
              a
  Piece Size  16 KiB
 Piece Count  1
  File Count  1
       Files  foo
";

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent");

      metainfo.dump(path).unwrap();

      env.run().unwrap();

      let have = env.out();
      let want = "\
Name\tfoo
Comment\tcomment
Created\t1970-01-01 00:00:01 UTC
Created By\tcreated by
Source\tsource
Info Hash\tb7595205a46491b3e8686e10b28efe7144d066cc
Torrent Size\t240
Content Size\t20
Private\tyes
Trackers\tb\tc\ta
Piece Size\t16384
Piece Count\t1
File Count\t1
Files\tfoo
";

      assert_eq!(have, want);
    }
  }
}
