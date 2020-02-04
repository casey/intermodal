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
    help = "Show information about `TORRENT`."
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

  #[test]
  fn output() {
    let mut env = testing::env(
      ["torrent", "show", "--input", "foo.torrent"]
        .iter()
        .cloned(),
    );

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

    let path = env.resolve("foo.torrent");

    metainfo.dump(path).unwrap();

    env.run().unwrap();

    let have = env.out();
    let want = "        Name  foo
     Comment  comment
     Created  1970-01-01 00:00:01 UTC
   Info Hash  bd68a8a5ab377e37e8cdbfd37b670408c59a009f
Torrent Size  236 bytes
Content Size  20 bytes
     Private  yes
    Trackers  Main:   announce
              Tier 1: announce
                      b
              Tier 2: c
  Piece Size  16 KiB
 Piece Count  1
  File Count  1
";

    assert_eq!(have, want);
  }
}
