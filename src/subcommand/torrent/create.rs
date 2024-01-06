use crate::common::*;
use create_content::CreateContent;
use create_step::CreateStep;

mod create_content;
mod create_step;

const INPUT_HELP: &str = "Read torrent contents from `INPUT`. If `INPUT` is a file, torrent will \
                          be a single-file torrent.  If `INPUT` is a directory, torrent will be a \
                          multi-file torrent.  If `INPUT` is `-`, read from standard input. Piece \
                          length defaults to 256KiB when reading from standard input if \
                          `--piece-length` is not given.";

const INPUT_FLAG: &str = "input-flag";

const INPUT_POSITIONAL: &str = "<INPUT>";

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Create a .torrent file.")
)]
pub(crate) struct Create {
  #[structopt(
    long = "announce",
    short = "a",
    value_name = "URL",
    help = "Use `URL` as the primary tracker announce URL. To supply multiple announce URLs, also \
            use `--announce-tier`."
  )]
  announce: Option<Url>,
  #[structopt(
    long = "allow",
    short = "A",
    value_name = "LINT",
    possible_values = Lint::VARIANTS,
    help = "Allow `LINT`. Lints check for conditions which, although permitted, are not usually \
            desirable. For example, piece length can be any non-zero value, but probably \
            shouldn't be below 16 KiB. The lint `small-piece-size` checks for this, and \
            `--allow small-piece-size` can be used to disable this check.",
  )]
  allowed_lints: Vec<Lint>,
  #[structopt(
    long = "announce-tier",
    short = "t",
    value_name = "URL-LIST",
    help = "Use `URL-LIST` as a tracker announce tier. Each instance adds a new \
            tier. To add multiple trackers to a given tier, separate their announce URLs \
            with commas:\n\
            \n\
            `--announce-tier udp://example.com:80/announce,https://example.net:443/announce`
            \n\
            Announce tiers are stored in the `announce-list` key of the top-level metainfo \
            dictionary as a list of lists of strings, as defined by BEP 12: Multitracker \
            Metadata Extension.
            \n\
            Note: Many BitTorrent clients do not implement the behavior described in BEP \
            12. See the discussion here for more details: \
            https://github.com/bittorrent/bittorrent.org/issues/82"
  )]
  announce_tiers: Vec<String>,
  #[structopt(
    long = "comment",
    short = "c",
    value_name = "TEXT",
    help = "Include `TEXT` as the comment for generated `.torrent` file. Stored under `comment` \
            key of top-level metainfo dictionary."
  )]
  comment: Option<String>,
  #[structopt(
    long = "node",
    value_name = "NODE",
    help = "Add DHT bootstrap node `NODE` to torrent. `NODE` should be in the form `HOST:PORT`, \
            where `HOST` is a domain name, an IPv4 address, or an IPv6 address surrounded by \
            brackets. May be given more than once to add multiple bootstrap nodes.

Examples:

    --node router.example.com:1337

    --node 203.0.113.0:2290

    --node [2001:db8:4275:7920:6269:7463:6f69:6e21]:8832"
  )]
  dht_nodes: Vec<HostPort>,
  #[structopt(
    long = "dry-run",
    short = "n",
    help = "Skip writing `.torrent` file to disk."
  )]
  dry_run: bool,
  #[structopt(
    long = "follow-symlinks",
    short = "F",
    help = "Follow symlinks in torrent input. By default, symlinks to files and directories are \
            not included in torrent contents."
  )]
  follow_symlinks: bool,
  #[structopt(
    long = "force",
    short = "f",
    help = "Overwrite the destination `.torrent` file, if it exists."
  )]
  force: bool,
  #[structopt(
    long = "glob",
    short = "g",
    value_name = "GLOB",
    help = "Include or exclude files that match `GLOB`. Multiple glob may be provided, with the \
            last one taking precedence. Precede a glob with `!` to exclude it."
  )]
  globs: Vec<String>,
  #[structopt(
    long = "include-hidden",
    short = "h",
    help = "Include hidden files that would otherwise be skipped, such as files that start with a \
            `.`, and files hidden by file attributes on macOS and Windows."
  )]
  include_hidden: bool,
  #[structopt(
    long = "include-junk",
    short = "j",
    help = "Include junk files that would otherwise be skipped."
  )]
  include_junk: bool,
  #[structopt(
    name = INPUT_POSITIONAL,
    value_name = "INPUT",
    empty_values = false,
    required_unless = INPUT_FLAG,
    conflicts_with = INPUT_FLAG,
    parse(try_from_os_str = InputTarget::try_from_os_str),
    help = INPUT_HELP,
  )]
  input_positional: Option<InputTarget>,
  #[structopt(
    name = INPUT_FLAG,
    long = "input",
    short = "i",
    value_name = "INPUT",
    empty_values = false,
    parse(try_from_os_str = InputTarget::try_from_os_str),
    help = INPUT_HELP,
  )]
  input_flag: Option<InputTarget>,
  #[structopt(
    long = "link",
    help = "Print created torrent `magnet:` URL to standard output"
  )]
  print_magnet_link: bool,
  #[structopt(
    long = "md5",
    short = "M",
    help = "Include MD5 checksum of each file in the torrent. N.B. MD5 is cryptographically \
            broken and only suitable for checking for accidental corruption."
  )]
  md5sum: bool,
  #[structopt(
    long = "name",
    short = "N",
    value_name = "TEXT",
    help = "Set name of torrent to `TEXT`. Defaults to the filename of the argument to `--input`. \
            Required when `--input -`.",
    required_if(INPUT_FLAG, "-"),
    required_if(INPUT_POSITIONAL, "-")
  )]
  name: Option<String>,
  #[structopt(
    long = "no-created-by",
    help = "Do not populate `created by` key of generated torrent with imdl version information."
  )]
  no_created_by: bool,
  #[structopt(
    long = "no-creation-date",
    help = "Do not populate `creation date` key of generated torrent with current time."
  )]
  no_creation_date: bool,
  #[structopt(
    long = "open",
    short = "O",
    help = "Open `.torrent` file after creation. Uses `xdg-open`, `gnome-open`, or `kde-open` on \
            Linux; `open` on macOS; and `cmd /C start` on Windows"
  )]
  open: bool,
  #[structopt(
    long = "sort-by",
    value_name = "SPEC",
    help = "Set the order of files within a torrent. `SPEC` should be of the form `KEY:ORDER`, \
            with `KEY` being one of `path` or `size`, and `ORDER` being `ascending` or \
            `descending`. `:ORDER` defaults to `ascending` if omitted. The `--sort-by` flag may \
            be given more than once, with later values being used to break ties. Ties that remain \
            are broken in ascending path order.

Sort in ascending order by path, the default:

    --sort-by path:ascending

Sort in ascending order by path, more concisely:

    --sort-by path

Sort in ascending order by size, break ties in descending path order:

    --sort-by size:ascending --sort-by path:descending"
  )]
  sort_by: Vec<SortSpec>,
  #[structopt(
    long = "output",
    short = "o",
    value_name = "TARGET",
    empty_values(false),
    parse(try_from_os_str = OutputTarget::try_from_os_str),
    required_if(INPUT_FLAG, "-"),
    required_if(INPUT_POSITIONAL, "-"),
    help = "Save `.torrent` file to `TARGET`, or print to standard output if `TARGET` is `-`. \
            Defaults to the argument to `--input` with an `.torrent` extension appended. Required \
            when `--input -`.",
  )]
  output: Option<OutputTarget>,
  #[structopt(
    long = "peer",
    value_name = "PEER",
    help = "Add `PEER` to magnet link.",
    requires("print-magnet-link")
  )]
  peers: Vec<HostPort>,
  #[structopt(
    long = "piece-length",
    short = "p",
    value_name = "BYTES",
    help = "Set piece length to `BYTES`. Accepts SI units, e.g. kib, mib, and gib."
  )]
  piece_length: Option<Bytes>,
  #[structopt(
    long = "private",
    short = "P",
    help = "Set the `private` flag. Torrent clients that understand the flag and participate in \
            the swarm of a torrent with the flag set will only announce themselves to the \
            announce URLs included in the torrent, and will not use other peer discovery \
            mechanisms, such as the DHT or local peer discovery. See BEP 27: Private Torrents for \
            more information."
  )]
  private: bool,
  #[structopt(
    long = "show",
    short = "S",
    help = "Display information about created torrent file."
  )]
  show: bool,
  #[structopt(
    long = "source",
    short = "s",
    value_name = "TEXT",
    help = "Set torrent source to `TEXT`. Stored under `source` key of info dictionary. This is \
            useful for keeping statistics from being mis-reported when participating in swarms \
            with the same contents, but with different trackers. When source is set to a unique \
            value for torrents with the same contents, torrent clients will treat them as \
            distinct torrents, and not share peers between them, and will correctly report \
            download and upload statistics to multiple trackers."
  )]
  source: Option<String>,
  #[structopt(
    long = "ignore",
    help = "Skip files listed in `.gitignore`, `.ignore`, `.git/info/exclude`, and `git config \
            --get core.excludesFile`."
  )]
  ignore: bool,
  #[structopt(
    long = "update-url",
    value_name = "URL",
    help = "Set torrent feed URL to `URL`, stored in the `update-url` key of the info dictionary. \
            Clients that support BEP 39 will use the update URL to download revised versions of \
            the torret's metainfo. Note that BEP 39 is not widely supported."
  )]
  update_url: Option<Url>,
}

impl Create {
  pub(crate) fn run(self, env: &mut Env, options: &Options) -> Result<(), Error> {
    let input = xor_args(
      "input_positional",
      &self.input_positional,
      "input_flag",
      &self.input_flag,
    )?;

    let mut linter = Linter::new();
    linter.allow(self.allowed_lints.iter().copied());

    let mut announce_list = Vec::new();
    for tier in &self.announce_tiers {
      let tier = tier.split(',').map(str::to_string).collect::<Vec<String>>();

      tier
        .iter()
        .map(|announce| announce.parse())
        .collect::<Result<Vec<Url>, url::ParseError>>()
        .context(error::AnnounceUrlParse)?;

      announce_list.push(tier);
    }

    if linter.is_denied(Lint::PrivateTrackerless) && self.private && self.announce.is_none() {
      return Err(Error::PrivateTrackerless);
    }

    let creation_date = if self.no_creation_date {
      None
    } else {
      Some(
        SystemTime::now()
          .duration_since(SystemTime::UNIX_EPOCH)?
          .as_secs(),
      )
    };

    CreateStep::Searching { input: &input }.print(env)?;

    let content = CreateContent::from_create(&self, &input, env)?;

    let output = content.output.resolve(env)?;

    if content.piece_length.count() == 0 {
      return Err(Error::PieceLengthZero);
    }

    if linter.is_denied(Lint::UnevenPieceLength) && !content.piece_length.count().is_power_of_two()
    {
      return Err(Error::PieceLengthUneven {
        bytes: content.piece_length,
      });
    }

    if linter.is_denied(Lint::SmallPieceLength) && content.piece_length.count() < 16 * 1024 {
      return Err(Error::PieceLengthSmall);
    }

    if let OutputTarget::Path(path) = &output {
      if !self.force && path.exists() {
        return Err(Error::OutputExists { path: path.clone() });
      }
    }

    let private = if self.private { Some(true) } else { None };

    let created_by = if self.no_created_by {
      None
    } else {
      Some(String::from(consts::CREATED_BY_DEFAULT))
    };

    CreateStep::Hashing.print(env)?;

    let hasher = Hasher::new(
      self.md5sum,
      content.piece_length.as_piece_length()?.into_usize(),
      if env.err().is_styled_term() && !options.quiet {
        Some(content.progress_bar)
      } else {
        None
      },
    );

    let (mode, pieces) = if let Some(files) = content.files {
      hasher.hash_files(&files)?
    } else {
      hasher.hash_stdin(&mut env.input())?
    };

    CreateStep::Writing {
      output: &content.output,
    }
    .print(env)?;

    let info = Info {
      name: content.name,
      piece_length: content.piece_length,
      source: self.source,
      update_url: self.update_url,
      mode,
      pieces,
      private,
    };

    let metainfo = Metainfo {
      comment: self.comment,
      encoding: Some(consts::ENCODING_UTF8.to_owned()),
      announce: self.announce.map(|url| url.to_string()),
      announce_list: if announce_list.is_empty() {
        None
      } else {
        Some(announce_list)
      },
      nodes: if self.dht_nodes.is_empty() {
        None
      } else {
        Some(self.dht_nodes)
      },
      creation_date,
      created_by,
      info,
    };

    let bytes = metainfo.serialize()?;

    if !self.dry_run {
      match &output {
        OutputTarget::Path(path) => {
          let mut open_options = fs::OpenOptions::new();

          if self.force {
            open_options.write(true).create(true).truncate(true);
          } else {
            open_options.write(true).create_new(true);
          }

          open_options
            .open(path)
            .and_then(|mut file| file.write_all(&bytes))
            .context(error::Filesystem { path })?;
        }
        OutputTarget::Stdout => env.out_mut().write_all(&bytes).context(error::Stdout)?,
      }
    }

    #[cfg(test)]
    {
      if let InputTarget::Path(path) = &input {
        let deserialized = bendy::serde::de::from_bytes::<Metainfo>(&bytes).unwrap();

        assert_eq!(deserialized, metainfo);

        let status = metainfo.verify(&env.resolve(path)?, None)?;

        status.print(env)?;

        if !status.good() {
          return Err(Error::Verify);
        }
      }
    }

    errln!(env, "\u{2728}\u{2728} Done! \u{2728}\u{2728}")?;

    if self.show {
      // We just created this torrent, so no extra fields have been discarded.
      TorrentSummary::from_metainfo_lossy(metainfo.clone())?.write(env)?;
    }

    if self.print_magnet_link {
      // We just created this torrent, so no extra fields have been discarded.
      let mut link = MagnetLink::from_metainfo_lossy(&metainfo)?;
      for peer in self.peers {
        link.add_peer(peer);
      }
      outln!(env, "{}", link)?;
    }

    if let OutputTarget::Path(path) = output {
      if self.open {
        Platform::open_file(&path)?;
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn require_input_argument() {
    let mut env = test_env! { args: [], tree: {} };
    assert!(matches!(env.run(), Err(Error::Clap { .. })));
  }

  #[test]
  fn input_arguments_conflict() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "bar",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn require_name_if_input_flag_stdin() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "-",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn require_name_if_input_positional_stdin() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "-",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn require_output_if_input_flag_stdin() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "-",
        "--name",
        "foo",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn require_output_if_input_positional_stdin() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "-",
        "--name",
        "foo",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn require_input_present() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {},
    };
    assert!(matches!(env.run(), Err(Error::Filesystem { .. })));
  }

  #[test]
  fn announce_is_optional() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
      ],
      tree: {
        foo: "",
      },
    };

    env.assert_ok();
  }

  #[test]
  fn torrent_file_is_bencode_dict() -> Result<()> {
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
        foo: "",
      }
    };
    env.assert_ok();
    let torrent = env.resolve("foo.torrent")?;
    let bytes = fs::read(torrent).unwrap();
    let value = Value::from_bencode(&bytes).unwrap();
    assert!(matches!(value, Value::Dict(_)));
    Ok(())
  }

  #[test]
  fn input_positional() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "foo",
      ],
      tree: {
        foo: {
          bar: "",
          baz: "",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.name, "foo");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple{files} if files.len() == 2
    );
  }

  #[test]
  fn input_dot() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        ".",
        "--announce",
        "https://bar",
      ],
      cwd: "dir",
      tree: {
        dir: {
          foo: "",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("../dir.torrent");
    assert_eq!(metainfo.info.name, "dir");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple{files} if files.len() == 1
    );
  }

  #[test]
  fn input_dot_dot() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "..",
        "--announce",
        "https://bar",
      ],
      cwd: "a/b",
      tree: {
        a: {
          b: {
            foo: "",
          },
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("../../a.torrent");
    assert_eq!(metainfo.info.name, "a");
    assert_matches!(metainfo.info.mode, Mode::Multiple{files} if files.len() == 1);
  }

  #[test]
  fn privacy_defaults_to_false() {
    let mut env = test_env! {
      args: ["torrent", "create", "--input", "foo", "--announce", "https://bar"],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.private, None);
  }

  #[test]
  fn privacy_flag_sets_privacy() {
    let mut env = test_env! {
      args: ["torrent", "create", "--input", "foo", "--announce", "https://bar", "--private"],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.private, Some(true));
  }

  #[test]
  fn tracker_flag_must_be_url() {
    let mut env = test_env! {
      args: ["torrent", "create", "--input", "foo", "--announce", "bar"],
      tree: {
        foo: "",
      }
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn announce_single() {
    let mut env = test_env! {
      args: ["torrent", "create", "--input", "foo", "--announce", "http://bar"],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.announce, Some("http://bar/".into()));
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_udp() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "udp://tracker.opentrackr.org:1337/announce",
      ],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.announce.as_deref(),
      Some("udp://tracker.opentrackr.org:1337/announce")
    );
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_wss_tracker() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "wss://tracker.btorrent.xyz",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.announce.as_deref(),
      Some("wss://tracker.btorrent.xyz/")
    );
    assert!(metainfo.announce_list.is_none());
  }

  #[test]
  fn announce_single_tier() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--announce-tier",
        "http://bar,http://baz",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.announce.as_deref(), Some("http://bar/"));
    assert_eq!(
      metainfo.announce_list,
      Some(vec![vec!["http://bar".into(), "http://baz".into()]]),
    );
  }

  #[test]
  fn announce_multiple_tiers() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--announce-tier",
        "http://bar,http://baz",
        "--announce-tier",
        "http://abc,http://xyz",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.announce.as_deref(), Some("http://bar/"));
    assert_eq!(
      metainfo.announce_list,
      Some(vec![
        vec!["http://bar".into(), "http://baz".into()],
        vec!["http://abc".into(), "http://xyz".into()],
      ])
    );
  }

  #[test]
  fn comment_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.comment, None);
  }

  #[test]
  fn comment_set() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--comment",
        "Hello, world!",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.comment.unwrap(), "Hello, world!");
  }

  #[test]
  fn piece_length_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.piece_length, Bytes::from(16 * 2u32.pow(10)));
  }

  #[test]
  fn piece_length_override() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "64KiB",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.piece_length, Bytes(64 * 1024));
  }

  #[test]
  fn si_piece_size() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "0.5MiB",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.piece_length, Bytes(512 * 1024));
  }

  #[test]
  fn name() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "16KiB",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.name, "foo");
  }

  #[test]
  fn name_subdir() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo/bar",
        "--announce",
        "http://bar",
        "--piece-length",
        "32KiB",
      ],
      tree: {
        foo: {
          bar: "",
        },
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo/bar.torrent");
    assert_eq!(metainfo.info.name, "bar");
  }

  #[test]
  fn destination_override() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--output",
        "x.torrent",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    env.load_metainfo("x.torrent");
  }

  #[test]
  fn created_by_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.created_by.unwrap(), consts::CREATED_BY_DEFAULT);
  }

  #[test]
  fn created_by_unset() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--no-created-by",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.created_by, None);
  }

  #[test]
  fn encoding() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.encoding, Some("UTF-8".into()));
  }

  #[test]
  fn created_date_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap()
      .as_secs();
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert!(metainfo.creation_date.unwrap() < now + 10);
    assert!(metainfo.creation_date.unwrap() > now - 10);
  }

  #[test]
  fn created_date_unset() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--no-creation-date",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.creation_date, None);
  }

  #[test]
  fn uneven_last_piece() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--allow",
        "small-piece-length",
        "--piece-length",
        "4",
        ],
      tree: {
        foo: "123",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["123"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(3),
        md5sum: None,
      }
    );
  }

  #[test]
  fn even_last_piece() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--allow",
        "small-piece-length",
        "--piece-length",
        "4",
        ],
      tree: {
        foo: "1234",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["1234"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(4),
        md5sum: None,
      }
    );
  }

  #[test]
  fn multi_piece_file() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--allow",
        "small-piece-length",
        "--piece-length",
        "2",
        ],
      tree: {
        foo: "1234",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["12", "34"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(4),
        md5sum: None,
      }
    );
  }

  #[test]
  fn multi_file_piece() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "dir",
        "--announce",
        "http://bar",
        "--allow",
        "small-piece-length",
        "--piece-length",
        "8",
        "--md5",
        ],
      tree: {
        dir: {
          foo: "1234",
          bar: "5678",
        },
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("dir.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["56781234"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Multiple {
        files: vec![
          FileInfo {
            path: FilePath::from_components(&["bar"]),
            length: Bytes(4),
            md5sum: Some(Md5Digest::from_data("5678")),
          },
          FileInfo {
            path: FilePath::from_components(&["foo"]),
            length: Bytes(4),
            md5sum: Some(Md5Digest::from_data("1234")),
          },
        ],
      }
    );
  }

  #[test]
  fn single_small() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        ],
      tree: {
        foo: "bar",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["bar"]));
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(3),
        md5sum: None,
      }
    );
  }

  #[test]
  fn single_one_byte_piece() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "1",
        "--allow",
        "small-piece-length",
      ],
      tree: {
        foo: "bar",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.info.pieces,
      PieceList::from_pieces(["b", "a", "r"])
    );
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(3),
        md5sum: None,
      }
    );
  }

  #[test]
  fn single_empty() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces.count(), 0);
    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(0),
        md5sum: None,
      }
    );
  }

  #[test]
  fn multiple_no_files() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: {},
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces.count(), 0);
    assert_eq!(metainfo.info.mode, Mode::Multiple { files: Vec::new() });
  }

  #[test]
  fn multiple_one_file_md5() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5",
      ],
      tree: {
        foo: {
          bar: "bar",
        },
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["bar"]));
    match metainfo.info.mode {
      Mode::Multiple { files } => {
        assert_eq!(
          files,
          &[FileInfo {
            length: Bytes(3),
            md5sum: Some(Md5Digest::from_hex("37b51d194a7513e45b56f6524f2d51f2")),
            path: FilePath::from_components(&["bar"]),
          },]
        );
      }
      Mode::Single { .. } => panic!("Expected multi-file torrent"),
    }
  }

  #[test]
  fn multiple_one_file_md5_off() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: {
          bar: "bar",
        },
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["bar"]));
    match metainfo.info.mode {
      Mode::Multiple { files } => {
        assert_eq!(
          files,
          &[FileInfo {
            length: Bytes(3),
            md5sum: None,
            path: FilePath::from_components(&["bar"]),
          },]
        );
      }
      Mode::Single { .. } => panic!("Expected multi-file torrent"),
    }
  }

  #[test]
  fn multiple_three_files() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5"
      ],
      tree: {
        foo: {
          a: "abc",
          x: "xyz",
          h: "hij",
        },
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["abchijxyz"]));
    match metainfo.info.mode {
      Mode::Multiple { files } => {
        assert_eq!(
          files,
          &[
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("900150983cd24fb0d6963f7d28e17f72")),
              path: FilePath::from_components(&["a"]),
            },
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("857c4402ad934005eae4638a93812bf7")),
              path: FilePath::from_components(&["h"]),
            },
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("d16fb36f0911f878998c136191af705e")),
              path: FilePath::from_components(&["x"]),
            },
          ]
        );
      }
      Mode::Single { .. } => panic!("Expected multi-file torrent"),
    }
  }

  #[test]
  fn uneven_piece_length() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "17KiB",
      ],
      tree: {
        foo: {},
      },
    };
    assert_matches!(
      env.run(),
      Err(Error::PieceLengthUneven { bytes }) if bytes.0 == 17 * 1024
    );
  }

  #[test]
  fn uneven_piece_length_allow() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "17KiB",
        "--allow",
        "uneven-piece-length",
      ],
      tree: {
        foo: {},
      },
    };
    env.assert_ok();
    env.load_metainfo("foo.torrent");
  }

  #[test]
  fn zero_piece_length() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "0",
      ],
      tree: {
        foo: {},
      },
    };
    assert_matches!(env.run(), Err(Error::PieceLengthZero));
  }

  #[test]
  fn small_piece_length() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "8KiB",
      ],
      tree: {
        foo: "",
      },
    };
    assert_matches!(env.run(), Err(Error::PieceLengthSmall));
  }

  #[test]
  fn small_piece_length_allow() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--piece-length",
        "8KiB",
        "--allow",
        "small-piece-length",
      ],
      tree: {
        foo: {},
      }
    };
    env.assert_ok();
    env.load_metainfo("foo.torrent");
  }

  #[test]
  fn output() -> Result<()> {
    let mut env = TestEnvBuilder::new()
      .arg_slice(&[
        "imdl",
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--no-creation-date",
      ])
      .out_is_term()
      .build();

    let dir = env.resolve("foo")?;
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("a"), "abc").unwrap();
    fs::write(dir.join("x"), "xyz").unwrap();
    fs::write(dir.join("h"), "hij").unwrap();
    env.assert_ok();
    assert_eq!(env.out(), "");
    Ok(())
  }

  #[test]
  fn show() -> Result<()> {
    let mut env = TestEnvBuilder::new()
      .arg_slice(&[
        "imdl",
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--no-creation-date",
        "--show",
      ])
      .out_is_term()
      .build();

    let dir = env.resolve("foo")?;
    fs::create_dir(&dir).unwrap();
    fs::write(dir.join("a"), "abc").unwrap();
    fs::write(dir.join("x"), "xyz").unwrap();
    fs::write(dir.join("h"), "hij").unwrap();
    env.assert_ok();
    let have = env.out();
    #[rustfmt::skip]
    let want = format!(
      "        Name  foo
  Created By  {}
   Info Hash  d3432a4b9d18baa413095a70f1e417021ceaca5b
Torrent Size  {} bytes
Content Size  9 bytes
     Private  no
     Tracker  http://bar/
  Piece Size  16 KiB
 Piece Count  1
  File Count  3
       Files  foo
              ├─a
              ├─h
              └─x
",
      consts::CREATED_BY_DEFAULT,
      212 + consts::CREATED_BY_DEFAULT.len()
    );
    assert_eq!(have, want);
    Ok(())
  }

  #[test]
  fn write_to_stdout() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--output",
        "-",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();

    let bytes = env.out_bytes();
    Metainfo::from_bytes(&bytes);
  }

  #[test]
  fn force_default() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar"
      ],
      tree: {
        foo: "",
        "foo.torrent": "foo",
      },
    };
    assert_matches!(
      env.run().unwrap_err(),
      Error::OutputExists {path}
      if path == env.resolve("foo.torrent")?
    );
    Ok(())
  }

  #[test]
  fn force_true() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--force",
      ],
      tree: {
        foo: "",
        "foo.torrent": "foo",
      },
    };
    env.assert_ok();
    env.load_metainfo("foo.torrent");
  }

  #[test]
  fn exclude_junk() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: {
          "Thumbs.db": "abc",
          "Desktop.ini": "abc",
        },
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn include_junk() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--include-junk",
      ],
      tree: {
        foo: {
          "Thumbs.db": "abc",
          "Desktop.ini": "abc",
        },
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 2
    );
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["abcabc"]));
  }

  #[test]
  fn skip_hidden() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: {
          ".hidden": "abc",
          hidden: "abc",
        },
      }
    };

    if cfg!(target_os = "windows") {
      Command::new("attrib")
        .arg("+h")
        .arg(env.resolve("foo/hidden")?)
        .status()
        .unwrap();
    } else {
      fs::remove_file(env.resolve("foo/hidden")?).unwrap();
    }

    env.assert_ok();

    let metainfo = env.load_metainfo("foo.torrent");

    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
    Ok(())
  }

  #[test]
  fn include_hidden() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--include-hidden",
      ],
      tree: {
        foo: {
          ".hidden": "abc",
          hidden: "abc",
        },
      }
    };

    if cfg!(target_os = "windows") {
      Command::new("attrib")
        .arg("+h")
        .arg(env.resolve("foo/hidden")?)
        .status()
        .unwrap();
    } else if cfg!(target_os = "macos") {
      Command::new("chflags")
        .arg("hidden")
        .arg(env.resolve("foo/hidden")?)
        .status()
        .unwrap();
    }

    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 2
    );
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["abcabc"]));
    Ok(())
  }

  fn populate_symlinks(env: &Env) -> Result<()> {
    let dir = env.resolve("foo")?;
    let file_src = env.resolve("bar")?;
    let dir_src = env.resolve("dir-src")?;
    let dir_contents = dir_src.join("baz");
    fs::create_dir(&dir_src).unwrap();
    fs::write(dir_contents, "baz").unwrap();

    fs::create_dir(dir).unwrap();
    fs::write(file_src, "bar").unwrap();
    #[cfg(unix)]
    {
      let file_link = env.resolve("foo/bar")?;
      let dir_link = env.resolve("foo/dir")?;
      Command::new("ln")
        .arg("-s")
        .arg("../bar")
        .arg(file_link)
        .status()
        .unwrap();

      Command::new("ln")
        .arg("-s")
        .arg("../dir-src")
        .arg(dir_link)
        .status()
        .unwrap();
    }

    Ok(())
  }

  #[test]
  fn skip_symlinks() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5",
      ],
      tree: {},
    };
    populate_symlinks(&env)?;
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
    Ok(())
  }

  #[test]
  #[cfg(unix)]
  fn follow_symlinks() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--follow-symlinks",
        "--md5",
      ],
      tree: {},
    };
    populate_symlinks(&env)?;
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    let mut pieces = PieceList::new();
    pieces.push(Sha1::from("barbaz").digest().into());
    assert_eq!(metainfo.info.pieces, pieces);
    match metainfo.info.mode {
      Mode::Multiple { files } => {
        assert_eq!(
          files,
          &[
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("37b51d194a7513e45b56f6524f2d51f2")),
              path: FilePath::from_components(&["bar"]),
            },
            FileInfo {
              length: Bytes(3),
              md5sum: Some(Md5Digest::from_hex("73feffa4b7f6bb68e44cf984c85f6e88")),
              path: FilePath::from_components(&["dir", "baz"]),
            },
          ]
        );
      }
      Mode::Single { .. } => panic!("Expected multi-file torrent"),
    }
    Ok(())
  }

  #[test]
  #[cfg(unix)]
  fn symlink_root() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5",
      ],
      tree: {},
    };

    let file_src = env.resolve("bar")?;
    let file_link = env.resolve("foo")?;

    Command::new("ln")
      .arg("-s")
      .arg(&file_src)
      .arg(&file_link)
      .status()
      .unwrap();

    assert_matches!(env.run().unwrap_err(), Error::SymlinkRoot { root } if root == file_link);
    Ok(())
  }

  #[test]
  fn skip_dot_dir_contents() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5",
      ],
      tree: {
        foo: {
          ".bar": {
            baz: "baz",
          },
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn skip_hidden_attribute_dir_contents() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--md5"
      ],
      tree: {
        foo: {
          bar: {},
        },
      },
    };

    #[cfg(target_os = "windows")]
    {
      env.write("foo/bar/baz", "baz");
      let path = env.resolve("foo/bar").unwrap();
      Command::new("attrib")
        .arg("+h")
        .arg(&path)
        .status()
        .unwrap();
    }

    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn glob_exclude() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "!a"
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 2
    );
    let mut pieces = PieceList::new();
    pieces.push(Sha1::from("bc").digest().into());
    assert_eq!(metainfo.info.pieces, pieces);
  }

  #[test]
  fn glob_exclude_nomatch() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "!x"
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };

    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 3
    );
    let mut pieces = PieceList::new();
    pieces.push(Sha1::from("abc").digest().into());
    assert_eq!(metainfo.info.pieces, pieces);
  }

  #[test]
  fn glob_include() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "[bc]",
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 2
    );
    let mut pieces = PieceList::new();
    pieces.push(Sha1::from("bc").digest().into());
    assert_eq!(metainfo.info.pieces, pieces);
  }

  #[test]
  fn glob_include_nomatch() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "x",
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.is_empty()
    );
    assert_eq!(metainfo.info.pieces, PieceList::new());
  }

  #[test]
  fn glob_entire_path() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "x*",
      ],
      tree: {
        foo: {
          a: "a",
          x: {
            y: "yyy",
          },
          c: "c",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 1
    );
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["yyy"]));
  }

  #[test]
  fn glob_torrent_root() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "foo*",
      ],
      tree: {
        foo: {
          a: "a",
          foo: {
            y: "yyy",
          },
          c: "c",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 1
    );
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["yyy"]));
  }

  #[test]
  fn glob_precedence() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--glob",
        "!*",
        "--glob",
        "[ab]",
        "--glob",
        "!b",
      ],
      tree: {
        foo: {
          a: "a",
          b: "b",
          c: "c",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 1
    );
    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["a"]));
  }

  #[test]
  fn ignore_files_in_gitignore() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--ignore",
      ],
      tree: {
        foo: {
          ".gitignore": "a",
          a: "a",
          b: "b",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 1
    );
  }

  #[test]
  fn ignore_files_in_ignore() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--ignore",
      ],
      tree: {
        foo: {
          ".ignore": "a",
          a: "a",
          b: "b",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 1
    );
  }

  #[test]
  fn ignore_files_in_git_exclude() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--ignore",
      ],
      tree: {
        foo: {
          ".git": {
            info: {
              exclude: "a",
            },
          },
          a: "a",
          b: "b",
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_matches!(
      metainfo.info.mode,
      Mode::Multiple { files } if files.len() == 1
    );
  }

  #[test]
  fn nodes_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
      ],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert!(metainfo.nodes.is_none());
  }

  #[test]
  fn nodes_invalid() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--node",
        "blah",
      ],
      tree: {
        foo: "",
      },
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn nodes_valid() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "http://bar",
        "--node",
        "router.example.com:1337",
        "--node",
        "203.0.113.0:2290",
        "--node",
        "[2001:db8:4275:7920:6269:7463:6f69:6e21]:8832",
      ],
      tree: {
        foo: "",
      },
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.nodes,
      Some(vec![
        "router.example.com:1337".parse().unwrap(),
        "203.0.113.0:2290".parse().unwrap(),
        "[2001:db8:4275:7920:6269:7463:6f69:6e21]:8832"
          .parse()
          .unwrap(),
      ]),
    );
  }

  #[test]
  fn private_requires_announce() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--private",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(
      env.run(),
      Err(error @ Error::PrivateTrackerless)
      if error.lint() == Some(Lint::PrivateTrackerless)
    );
  }

  #[test]
  fn private_trackerless_announce() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--private",
        "--allow",
        "private-trackerless",
      ],
      tree: {
        foo: "",
      },
    };

    env.assert_ok();
  }

  #[test]
  fn no_print_magnet_link() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
      ],
      tree: {
        foo: "",
      },
    };

    env.assert_ok();
    assert_eq!(env.out(), "");
  }

  #[test]
  fn print_magnet_link() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--link",
      ],
      tree: {
        foo: "",
      },
    };

    env.assert_ok();
    assert_eq!(
      env.out(),
      "magnet:?xt=urn:btih:516735f4b80f2b5487eed5f226075bdcde33a54e&dn=foo\n"
    );
  }

  #[test]
  fn print_magnet_link_with_announce() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--link",
        "--announce",
        "http://foo.com/announce",
      ],
      tree: {
        foo: "",
      },
    };

    env.assert_ok();
    assert_eq!(
      env.out(),
      "magnet:\
      ?xt=urn:btih:516735f4b80f2b5487eed5f226075bdcde33a54e\
      &dn=foo\
      &tr=http://foo.com/announce\n"
    );
  }

  #[test]
  fn peer_requires_link() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--peer",
      ],
      tree: {
        foo: "",
      },
    };

    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn link_with_peers() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--peer",
        "foo:1337",
        "--peer",
        "bar:666",
        "--link"
      ],
      tree: {
        foo: "",
      },
    };

    env.assert_ok();
    assert_eq!(
      env.out(),
      "magnet:?xt=urn:btih:516735f4b80f2b5487eed5f226075bdcde33a54e&dn=foo&x.pe=foo:1337&x.pe=bar:\
       666\n"
    );
  }

  #[test]
  fn dry_run_skips_torrent_file_creation() -> Result<()> {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--dry-run",
      ],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
    let torrent = env.resolve("foo.torrent")?;
    let err = fs::read(torrent).unwrap_err();
    assert_eq!(err.kind(), io::ErrorKind::NotFound);
    Ok(())
  }

  #[test]
  fn file_ordering_by_default() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "eeee",
          },
        },
      }
    };

    env.assert_ok();

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["a", "b", "c", "d/e"]);
  }

  #[test]
  fn file_ordering_by_path_ascending() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--sort-by",
        "path",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "eeee",
          },
        },
      }
    };

    env.assert_ok();

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["a", "b", "c", "d/e"]);
  }

  #[test]
  fn file_ordering_by_path_descending() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--sort-by",
        "path:descending",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            a: "aaaa",
          },
        },
      }
    };

    env.assert_ok();

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["d/a", "c", "b", "a"]);
  }

  #[test]
  fn file_ordering_by_size_ascending() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--sort-by",
        "size:ascending",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "e",
          },
        },
      }
    };

    env.assert_ok();

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["b", "d/e", "a", "c"]);
  }

  #[test]
  fn file_ordering_by_size_descending() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--sort-by",
        "size:descending",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "e",
          },
        },
      }
    };

    env.assert_ok();

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["c", "a", "b", "d/e"]);
  }

  #[test]
  fn file_ordering_by_size_ascending_break_ties_path_descending() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--sort-by",
        "size:ascending",
        "--sort-by",
        "path:descending",
      ],
      tree: {
        foo: {
          a: "aa",
          b: "b",
          c: "ccc",
          d: {
            e: "e",
          },
        },
      }
    };

    env.assert_ok();

    let torrent = env.load_metainfo("foo.torrent");
    assert_eq!(torrent.file_paths(), &["d/e", "b", "a", "c"]);
  }

  #[test]
  fn name_required_when_input_is_stdin() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "-",
        "--announce",
        "http://bar",
        "--output",
        "foo.torrent",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn foo_required_when_input_is_stdin() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "-",
        "--announce",
        "http://bar",
        "--name",
        "foo",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn create_from_stdin() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "-",
        "--announce",
        "http://bar",
        "--name",
        "foo",
        "--output",
        "foo.torrent",
        "--md5",
      ],
      input: "hello",
      tree: {},
    };

    env.assert_ok();

    let metainfo = env.load_metainfo("foo.torrent");

    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["hello"]));

    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(5),
        md5sum: Some(Md5Digest::from_data("hello")),
      }
    );
  }

  #[test]
  fn create_messages_path() {
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
        foo: "",
      }
    };

    let want = "[1/3] \u{1F9FF} Searching `foo` for files…\n[2/3] \u{1F9EE} Hashing \
                pieces…\n[3/3] \u{1F4BE} Writing metainfo to `foo.torrent`…\n\u{2728}\u{2728} \
                Done! \u{2728}\u{2728}\n";

    env.assert_ok();

    assert_eq!(env.err(), want);
  }

  #[test]
  fn create_messages_subdir() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo/bar",
        "--announce",
        "https://bar",
      ],
      tree: {
        foo: {
          bar: "",
        },
      }
    };

    let want = format!(
      "[1/3] \u{1F9FF} Searching `foo/bar` for files…\n[2/3] \u{1F9EE} Hashing pieces…\n[3/3] \
       \u{1F4BE} Writing metainfo to `{}`…\n\u{2728}\u{2728} Done! \u{2728}\u{2728}\n",
      Path::new("foo").join("bar.torrent").display(),
    );

    env.assert_ok();

    assert_eq!(env.err(), want);
  }

  #[test]
  fn create_messages_dot() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        ".",
        "--announce",
        "https://bar",
      ],
      cwd: "dir",
      tree: {
        dir: {
          foo: "",
        },
      }
    };

    env.assert_ok();
    let metainfo = env.load_metainfo("../dir.torrent");
    assert_eq!(metainfo.info.name, "dir");
    assert_matches!(metainfo.info.mode, Mode::Multiple{files} if files.len() == 1);

    let want = format!(
      "[1/3] \u{1F9FF} Searching `.` for files…\n[2/3] \u{1F9EE} Hashing pieces…\n[3/3] \u{1F4BE} \
       Writing metainfo to `{}`…\n\u{2728}\u{2728} Done! \u{2728}\u{2728}\n",
      Path::new("..").join("dir.torrent").display(),
    );

    assert_eq!(env.err(), want);
  }

  #[test]
  fn create_messages_dot_dot() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "..",
        "--announce",
        "https://bar",
      ],
      cwd: "a/b",
      tree: {
        a: {
          b: {
            foo: "",
          },
        },
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("../../a.torrent");
    assert_eq!(metainfo.info.name, "a");
    assert_matches!(metainfo.info.mode, Mode::Multiple{files} if files.len() == 1);

    let want = format!(
      "[1/3] \u{1F9FF} Searching `..` for files…\n[2/3] \u{1F9EE} Hashing pieces…\n[3/3] \
       \u{1F4BE} Writing metainfo to `{}`…\n\u{2728}\u{2728} Done! \u{2728}\u{2728}\n",
      Path::new("..").join("..").join("a.torrent").display(),
    );

    assert_eq!(env.err(), want);
  }

  #[test]
  fn create_messages_absolute() {
    let dir = TempDir::new().unwrap();

    let input = dir.path().join("foo");

    fs::write(&input, "").unwrap();

    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        &input,
        "--announce",
        "https://bar",
      ],
      tree: {
      }
    };

    let torrent = dir.path().join("foo.torrent");

    env.assert_ok();

    let metainfo = env.load_metainfo(&torrent);
    assert_eq!(metainfo.info.name, "foo");

    let want = format!(
      "[1/3] \u{1F9FF} Searching `{}` for files…\n[2/3] \u{1F9EE} Hashing pieces…\n[3/3] \
       \u{1F4BE} Writing metainfo to `{}`…\n\u{2728}\u{2728} Done! \u{2728}\u{2728}\n",
      input.display(),
      torrent.display(),
    );

    assert_eq!(env.err(), want);
  }

  #[test]
  fn create_messages_stdio() {
    let dir = TempDir::new().unwrap();

    let input = dir.path().join("foo");

    fs::write(input, "").unwrap();

    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "-",
        "--announce",
        "https://bar",
        "--name",
        "foo",
        "--output",
        "-",
        "--md5",
      ],
      input: "hello",
      tree: {
      }
    };

    env.assert_ok();

    let bytes = env.out_bytes();
    let metainfo = Metainfo::from_bytes(&bytes);

    assert_eq!(metainfo.info.pieces, PieceList::from_pieces(["hello"]));

    assert_eq!(
      metainfo.info.mode,
      Mode::Single {
        length: Bytes(5),
        md5sum: Some(Md5Digest::from_data("hello")),
      }
    );

    let want = "[1/3] \u{1F9FF} Creating single-file torrent from standard input…\n[2/3] \
                \u{1F9EE} Hashing pieces…\n[3/3] \u{1F4BE} Writing metainfo to standard \
                output…\n\u{2728}\u{2728} Done! \u{2728}\u{2728}\n";

    assert_eq!(env.err(), want);
  }

  #[test]
  fn no_output_when_quiet() {
    let mut env = test_env! {
      args: [
        "--quiet",
        "torrent",
        "create",
        "--input",
        "foo"
      ],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
    assert_eq!(env.out(), "");
    assert_eq!(env.err(), "");
  }

  #[test]
  fn test_update_url_invalid_url_fails() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--update-url",
        "not-a-URL",
        "--input",
        "foo"
      ],
      tree: {
        foo: "",
      }
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn test_update_url_valid_url_succeeds() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--update-url",
        "https://www.a_real_url.com",
        "--input",
        "foo"
      ],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
  }

  #[test]
  fn test_update_url() {
    let mut env = test_env! {
      args: [
        "torrent",
        "create",
        "--update-url",
        "https://www.a_real_url.com",
        "--input",
        "foo",
      ],
      tree: {
        foo: "",
      }
    };
    env.assert_ok();
    let metainfo = env.load_metainfo("foo.torrent");
    assert_eq!(
      metainfo.info.update_url,
      Some("https://www.a_real_url.com/".parse().unwrap())
    );
  }
}
