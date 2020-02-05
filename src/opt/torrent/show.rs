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
    let summary = TorrentSummary::load(&env.resolve(self.input))?;

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
        private: Some(1),
        piece_length: 16 * 1024,
        source: Some("source".into()),
        name: "foo".into(),
        pieces: vec![
          0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        ],
        mode: Mode::Single {
          length: 20,
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
Source\tsource
Info Hash\tb7595205a46491b3e8686e10b28efe7144d066cc
Torrent Size\t252
Content Size\t20
Private\tyes
Trackers\tannounce\tb\tc
Piece Size\t16384
Piece Count\t1
File Count\t1
";

      assert_eq!(have, want);
    }
  }
}
