use crate::common::*;

const INPUT_HELP: &str = "Show information about torrent at `INPUT`. If `INPUT` is `-`, read \
                          torrent metainfo from standard input.";

const INPUT_FLAG: &str = "input-flag";

const INPUT_POSITIONAL: &str = "<INPUT>";

const INPUT_VALUE: &str = "INPUT";

const JSON_OUTPUT: &str = "json";

const JSON_HELP: &str = "Output data as JSON instead of the default format.";

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Display information about a .torrent file.")
)]
pub(crate) struct Show {
  #[structopt(
    name = INPUT_FLAG,
    long = "input",
    short = "i",
    value_name = INPUT_VALUE,
    empty_values(false),
    parse(try_from_os_str = InputTarget::try_from_os_str),
    help = INPUT_HELP,
  )]
  input_flag: Option<InputTarget>,
  #[structopt(
    name = INPUT_POSITIONAL,
    value_name = INPUT_VALUE,
    empty_values(false),
    parse(try_from_os_str = InputTarget::try_from_os_str),
    required_unless = INPUT_FLAG,
    conflicts_with = INPUT_FLAG,
    help = INPUT_HELP,
  )]
  input_positional: Option<InputTarget>,
  #[structopt(
    name = JSON_OUTPUT,
    long = "json",
    short = "j",
    help = JSON_HELP,
  )]
  json: bool,
}

impl Show {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let target = xor_args(
      "input_flag",
      &self.input_flag,
      "input_positional",
      &self.input_positional,
    )?;

    let input = env.read(target)?;
    let summary = TorrentSummary::from_input(&input)?;
    if self.json {
      summary.write_json(env)?;
    } else {
      summary.write(env)?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn input_required() {
    test_env! {
      args: [
        "torrent",
        "show",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "show",
        "--input",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Filesystem { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "show",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Filesystem { .. }),
    };

    test_env! {
      args: [
        "torrent",
        "show",
        "--input",
        "foo",
        "foo",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };
  }

  #[test]
  fn output() -> Result<()> {
    let metainfo = Metainfo::test_value_single();

    #[rustfmt::skip]
    let want_human_readable = format!(
"         Name  NAME
      Comment  COMMENT
Creation Date  1970-01-01 00:00:01 UTC
   Created By  CREATED BY
       Source  SOURCE
    Info Hash  {}
 Torrent Size  {}
 Content Size  32 KiB
      Private  yes
      Tracker  udp://announce.example:1337
Announce List  Tier 1: http://a.example:4567
                       https://b.example:77
               Tier 2: udp://c.example:88
   Update URL  https://update.example/
    DHT Nodes  node.example:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  2
   File Count  1
        Files  NAME
", Metainfo::test_value_single_infohash(), Metainfo::test_value_single_torrent_size());

    #[rustfmt::skip]
    let want_machine_readable = format!("\
name\tNAME
comment\tCOMMENT
creation date\t1970-01-01 00:00:01 UTC
created by\tCREATED BY
source\tSOURCE
info hash\t{}
torrent size\t{}
content size\t32768
private\tyes
tracker\tudp://announce.example:1337
announce list\thttp://a.example:4567\thttps://b.example:77\tudp://c.example:88
update url\thttps://update.example/
dht nodes\tnode.example:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
piece size\t16384
piece count\t2
file count\t1
files\tNAME
", Metainfo::test_value_single_infohash(), Metainfo::test_value_single_torrent_size().count());

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      assert_eq!(have, want_human_readable);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();

      assert_eq!(have, want_human_readable);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();

      assert_eq!(have, want_machine_readable);
    }

    Ok(())
  }

  #[test]
  fn tier_list_with_main() -> Result<()> {
    let metainfo = Metainfo::test_value_single();

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();

      #[rustfmt::skip]
      let want = format!(
        "         Name  NAME
      Comment  COMMENT
Creation Date  1970-01-01 00:00:01 UTC
   Created By  CREATED BY
       Source  SOURCE
    Info Hash  {}
 Torrent Size  {}
 Content Size  32 KiB
      Private  yes
      Tracker  udp://announce.example:1337
Announce List  Tier 1: http://a.example:4567
                       https://b.example:77
               Tier 2: udp://c.example:88
   Update URL  https://update.example/
    DHT Nodes  node.example:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  2
   File Count  1
        Files  NAME
",
        Metainfo::test_value_single_infohash(),
        Metainfo::test_value_single_torrent_size()
      );

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();

      #[rustfmt::skip]
      let want = format!(
        "\
name\tNAME
comment\tCOMMENT
creation date\t1970-01-01 00:00:01 UTC
created by\tCREATED BY
source\tSOURCE
info hash\t{}
torrent size\t{}
content size\t32768
private\tyes
tracker\tudp://announce.example:1337
announce list\thttp://a.example:4567\thttps://b.example:77\tudp://c.example:88
update url\thttps://update.example/
dht nodes\tnode.example:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
piece size\t16384
piece count\t2
file count\t1
files\tNAME
",
        Metainfo::test_value_single_infohash(),
        Metainfo::test_value_single_torrent_size().count()
      );

      assert_eq!(have, want);
    }

    Ok(())
  }

  #[test]
  fn tier_list_without_main() -> Result<()> {
    let mut metainfo = Metainfo::test_value_single();

    metainfo.announce_list = Some(vec![
      vec!["B".into()],
      vec!["C".into()],
      vec!["ANNOUNCE".into()],
    ]);

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();

      #[rustfmt::skip]
      let want = format!(
        "         Name  NAME
      Comment  COMMENT
Creation Date  1970-01-01 00:00:01 UTC
   Created By  CREATED BY
       Source  SOURCE
    Info Hash  {}
 Torrent Size  {}
 Content Size  32 KiB
      Private  yes
      Tracker  udp://announce.example:1337
Announce List  Tier 1: B
               Tier 2: C
               Tier 3: ANNOUNCE
   Update URL  https://update.example/
    DHT Nodes  node.example:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  2
   File Count  1
        Files  NAME
",
        Metainfo::test_value_single_infohash(),
        Bytes(Metainfo::test_value_single_torrent_size().count() - 50)
      );

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();

      #[rustfmt::skip]
      let want = format!("\
name\tNAME
comment\tCOMMENT
creation date\t1970-01-01 00:00:01 UTC
created by\tCREATED BY
source\tSOURCE
info hash\t{}
torrent size\t{}
content size\t32768
private\tyes
tracker\tudp://announce.example:1337
announce list\tB\tC\tANNOUNCE
update url\thttps://update.example/
dht nodes\tnode.example:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
piece size\t16384
piece count\t2
file count\t1
files\tNAME
",
        Metainfo::test_value_single_infohash(),
        Metainfo::test_value_single_torrent_size().count() - 50
      );

      assert_eq!(have, want);
    }

    Ok(())
  }

  #[test]
  fn trackerless() -> Result<()> {
    let mut metainfo = Metainfo::test_value_single();
    metainfo.announce = None;
    metainfo.announce_list = None;

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();

      #[rustfmt::skip]
      let want = format!("         Name  NAME
      Comment  COMMENT
Creation Date  1970-01-01 00:00:01 UTC
   Created By  CREATED BY
       Source  SOURCE
    Info Hash  {}
 Torrent Size  {}
 Content Size  32 KiB
      Private  yes
   Update URL  https://update.example/
    DHT Nodes  node.example:12
               1.1.1.1:16
               [2001:db8:85a3::8a2e:370]:7334
   Piece Size  16 KiB
  Piece Count  2
   File Count  1
        Files  NAME
",
        Metainfo::test_value_single_infohash(),
        Bytes(Metainfo::test_value_single_torrent_size().count() - 130)
      );

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      #[rustfmt::skip]
      let want = format!(
        "\
name\tNAME
comment\tCOMMENT
creation date\t1970-01-01 00:00:01 UTC
created by\tCREATED BY
source\tSOURCE
info hash\t{}
torrent size\t{}
content size\t32768
private\tyes
update url\thttps://update.example/
dht nodes\tnode.example:12\t1.1.1.1:16\t[2001:db8:85a3::8a2e:370]:7334
piece size\t16384
piece count\t2
file count\t1
files\tNAME
",
        Metainfo::test_value_single_infohash(),
        Metainfo::test_value_single_torrent_size().count() - 130
      );

      assert_eq!(have, want);
    }

    Ok(())
  }

  #[test]
  fn unset() -> Result<()> {
    let metainfo = Metainfo::test_value_single_unset();

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .out_is_term()
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();

      #[rustfmt::skip]
      let want = format!("        Name  NAME
   Info Hash  {}
Torrent Size  {}
Content Size  5 bytes
     Private  no
  Piece Size  1 KiB
 Piece Count  1
  File Count  1
       Files  NAME
",
        Metainfo::test_value_single_unset_infohash(),
        Metainfo::test_value_single_unset_torrent_size()
      );

      assert_eq!(have, want);
    }

    {
      let mut env = TestEnvBuilder::new()
        .arg_slice(&["imdl", "torrent", "show", "--input", "foo.torrent"])
        .build();

      let path = env.resolve("foo.torrent")?;

      metainfo.dump(path).unwrap();

      env.assert_ok();

      let have = env.out();
      #[rustfmt::skip]
      let want = format!(
        "\
name\tNAME
info hash\t{}
torrent size\t{}
content size\t5
private\tno
piece size\t1024
piece count\t1
file count\t1
files\tNAME
",
        Metainfo::test_value_single_unset_infohash(),
        Metainfo::test_value_single_unset_torrent_size().count()
      );

      assert_eq!(have, want);
    }

    Ok(())
  }

  #[test]
  fn output_json() {
    {
      let metainfo = Metainfo::test_value_single();
      let mut want = r#"{"name":"NAME","comment":"COMMENT","creation_date":1,
"created_by":"CREATED BY","source":"SOURCE","info_hash":"5d6f53772b4c20536fcce0c4c364d764a6efa39c",
"torrent_size":509,"content_size":32768,"private":true,"tracker":
"udp://announce.example:1337","announce_list":[["http://a.example:4567",
"https://b.example:77"],["udp://c.example:88"]],"update_url":"https://update.example/",
"dht_nodes":["node.example:12","1.1.1.1:16","[2001:db8:85a3::8a2e:370]:7334"],
"piece_size":16384,"piece_count":2,"file_count":1,"files":["NAME"]}"#
        .replace('\n', "");
      want.push('\n');
      let mut env = TestEnvBuilder::new()
        .arg_slice(&[
          "imdl",
          "torrent",
          "show",
          "--input",
          "foo.torrent",
          "--json",
        ])
        .out_is_term()
        .build();
      let path = env.resolve("foo.torrent").unwrap();
      metainfo.dump(path).unwrap();
      env.assert_ok();
      let have = env.out();
      assert_eq!(have, want);
    }

    {
      let metainfo = Metainfo::test_value_single_unset();
      let mut want = r#"{"name":"NAME","comment":null,"creation_date":null,
"created_by":null,"source":null,"info_hash":"a9105b0ff5f7cefeee5599ed7831749be21cc04e",
"torrent_size":85,"content_size":5,"private":false,"tracker":null,"announce_list":[],
"update_url":null,"dht_nodes":[],"piece_size":1024,"piece_count":1,"file_count":1,
"files":["NAME"]}"#
        .replace('\n', "");
      want.push('\n');
      let mut env = TestEnvBuilder::new()
        .arg_slice(&[
          "imdl",
          "torrent",
          "show",
          "--input",
          "foo.torrent",
          "--json",
        ])
        .out_is_term()
        .build();
      let path = env.resolve("foo.torrent").unwrap();
      metainfo.dump(path).unwrap();
      env.assert_ok();
      let have = env.out();
      assert_eq!(have, want);
    }
  }
}
