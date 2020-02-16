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
      nodes: Some(vec![
        "x:12".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[2001:0db8:85a3::0000:8a2e:0370]:7334".parse().unwrap(),
      ]),
      comment: Some("comment".into()),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["xyz", "abc"]),
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
   Info Hash  e12253978dc6d50db11d05747abcea1ad03b51c5
Torrent Size  339 bytes
Content Size  20 bytes
     Private  yes
    Trackers  Tier 1: announce
                      b
              Tier 2: c
   DHT Nodes  x:12
              1.1.1.1:16
              [2001:db8:85a3::8a2e:370]:7334
  Piece Size  16 KiB
 Piece Count  2
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
Info Hash\te12253978dc6d50db11d05747abcea1ad03b51c5
Torrent Size\t339
Content Size\t20
Private\tyes
Trackers\tannounce\tb\tc
DHT Nodes\tx:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
Piece Size\t16384
Piece Count\t2
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
      nodes: Some(vec![
        "x:12".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[2001:0db8:85a3::0000:8a2e:0370]:7334".parse().unwrap(),
      ]),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["xyz", "abc"]),
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
   Info Hash  e12253978dc6d50db11d05747abcea1ad03b51c5
Torrent Size  327 bytes
Content Size  20 bytes
     Private  yes
    Trackers  a
              x
              y
              z
   DHT Nodes  x:12
              1.1.1.1:16
              [2001:db8:85a3::8a2e:370]:7334
  Piece Size  16 KiB
 Piece Count  2
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
Info Hash\te12253978dc6d50db11d05747abcea1ad03b51c5
Torrent Size\t327
Content Size\t20
Private\tyes
Trackers\ta\tx\ty\tz
DHT Nodes\tx:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
Piece Size\t16384
Piece Count\t2
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
      nodes: Some(vec![
        "x:12".parse().unwrap(),
        "1.1.1.1:16".parse().unwrap(),
        "[2001:0db8:85a3::8a2e:0370]:7334".parse().unwrap(),
      ]),
      created_by: Some("created by".into()),
      creation_date: Some(1),
      encoding: Some("UTF-8".into()),
      info: Info {
        private: Some(true),
        piece_length: Bytes(16 * 1024),
        source: Some("source".into()),
        name: "foo".into(),
        pieces: PieceList::from_pieces(&["abc"]),
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
   Info Hash  b9cd9cae5748518c99d00d8ae86c0162510be4d9
Torrent Size  307 bytes
Content Size  20 bytes
     Private  yes
    Trackers  b
              c
              a
   DHT Nodes  x:12
              1.1.1.1:16
              [2001:db8:85a3::8a2e:370]:7334
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
Info Hash\tb9cd9cae5748518c99d00d8ae86c0162510be4d9
Torrent Size\t307
Content Size\t20
Private\tyes
Trackers\tb\tc\ta
DHT Nodes\tx:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
Piece Size\t16384
Piece Count\t1
File Count\t1
Files\tfoo
";

      assert_eq!(have, want);
    }
  }
}
