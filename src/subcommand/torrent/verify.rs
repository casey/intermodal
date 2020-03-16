use crate::common::*;
use verify_step::VerifyStep;

mod verify_step;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Verify files against a `.torrent` file.")
)]
pub(crate) struct Verify {
  #[structopt(
    long = "input",
    short = "i",
    value_name = "METAINFO",
    help = "Verify torrent contents against torrent metainfo in `FILE`.",
    parse(from_os_str)
  )]
  metainfo: PathBuf,
  #[structopt(
    long = "content",
    short = "c",
    value_name = "PATH",
    help = "Verify torrent content at `PATH` against torrent metainfo. Defaults to `name` field \
            of torrent info dictionary.",
    parse(from_os_str)
  )]
  content: Option<PathBuf>,
}

impl Verify {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let metainfo_path = env.resolve(&self.metainfo);
    let metainfo = Metainfo::load(&metainfo_path)?;

    VerifyStep::Loading {
      metainfo: &metainfo_path,
    }
    .print(env)?;

    let base = if let Some(content) = &self.content {
      env.resolve(content)
    } else {
      metainfo_path.parent().unwrap().join(&metainfo.info.name)
    };

    let progress_bar = if env.err().is_styled_term() {
      let style = ProgressStyle::default_bar()
        .template(
          "{spinner:.green} ⟪{elapsed_precise}⟫ ⟦{bar:40.cyan}⟧ \
           {binary_bytes}/{binary_total_bytes} ⟨{binary_bytes_per_sec}, {eta}⟩",
        )
        .tick_chars(consts::TICK_CHARS)
        .progress_chars(consts::PROGRESS_CHARS);

      Some(ProgressBar::new(metainfo.content_size().count()).with_style(style))
    } else {
      None
    };

    VerifyStep::Verifying { content: &base }.print(env)?;

    let status = metainfo.verify(&base, progress_bar)?;

    status.print(env)?;

    if status.good() {
      errln!(
        env,
        "\u{2728}\u{2728} Verification succeeded! \u{2728}\u{2728}"
      )?;
      Ok(())
    } else {
      Err(Error::Verify)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn require_metainfo_argument() {
    let mut env = test_env! {
      args: [],
      tree: {},
    };
    assert!(matches!(env.run(), Err(Error::Clap { .. })));
  }

  #[test]
  fn pass() -> Result<()> {
    let mut create_env = test_env! {
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

    create_env.run()?;

    let torrent = create_env.resolve("foo.torrent");

    let mut verify_env = test_env! {
      args: [
        "torrent",
        "verify",
        "--input",
        &torrent,
      ],
      tree: {},
    };

    assert_matches!(verify_env.run(), Ok(()));

    let want = format!(
      "[1/2] \u{1F4BE} Loading metainfo from `{}`…\n[2/2] \u{1F9EE} Verifying pieces from \
       `{}`…\n\u{2728}\u{2728} Verification succeeded! \u{2728}\u{2728}\n",
      torrent.display(),
      create_env.resolve("foo").display()
    );

    assert_eq!(verify_env.err(), want);
    assert_eq!(verify_env.out(), "");

    Ok(())
  }

  #[test]
  fn fail() -> Result<()> {
    let mut create_env = test_env! {
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

    create_env.run()?;

    create_env.write("foo/a", "xyz");

    let torrent = create_env.resolve("foo.torrent");

    let mut verify_env = test_env! {
      args: [
        "torrent",
        "verify",
        "--input",
        &torrent,
      ],
      tree: {},
    };

    assert_matches!(verify_env.status(), Err(EXIT_FAILURE));

    let want = [
      &format!(
        "[1/2] \u{1F4BE} Loading metainfo from `{}`…",
        torrent.display()
      ),
      &format!(
        "[2/2] \u{1F9EE} Verifying pieces from `{}`…",
        create_env.resolve("foo").display()
      ),
      "Pieces corrupted.",
      "error: Torrent verification failed.",
      "",
    ]
    .join("\n");

    assert_eq!(verify_env.err(), want);
    assert_eq!(verify_env.out(), "");

    Ok(())
  }

  #[test]
  fn alternate_path() -> Result<()> {
    let mut create_env = test_env! {
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

    create_env.run()?;

    let torrent = create_env.resolve("foo.torrent");

    let foo = create_env.resolve("foo");

    let bar = create_env.resolve("bar");

    fs::rename(&foo, &bar).unwrap();

    let mut verify_env = test_env! {
      args: [
        "torrent",
        "verify",
        "--input",
        &torrent,
        "--content",
        &bar,
      ],
      tree: {},
    };

    assert_matches!(verify_env.run(), Ok(()));

    let want = format!(
      "[1/2] \u{1F4BE} Loading metainfo from `{}`…\n[2/2] \u{1F9EE} Verifying pieces from \
       `{}`…\n\u{2728}\u{2728} Verification succeeded! \u{2728}\u{2728}\n",
      torrent.display(),
      bar.display(),
    );

    assert_eq!(verify_env.err(), want);
    assert_eq!(verify_env.out(), "");

    Ok(())
  }

  #[test]
  fn output_multiple() -> Result<()> {
    let mut create_env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "https://bar",
        "--md5",
      ],
      tree: {
        foo: {
          a: "abc",
          d: "efg",
          h: "ijk",
          l: "mno",
          p: "qrs",
          t: "uvw",
        },
      },
    };

    create_env.run()?;

    let torrent = create_env.resolve("foo.torrent");

    create_env.write("foo/a", "xyz");
    create_env.write("foo/d", "efgg");
    create_env.write("foo/h", "ik");
    create_env.remove_file("foo/l");
    create_env.remove_file("foo/p");
    create_env.create_dir("foo/p");

    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;
      let metadata = create_env.metadata("foo/t");
      let mut permissions = metadata.permissions();
      permissions.set_mode(0);
      create_env.set_permissions("foo/t", permissions);
    }

    let mut verify_env = test_env! {
      args: [
        "torrent",
        "verify",
        "--input",
        &torrent,
      ],
      tree: {},
    };

    assert_matches!(verify_env.status(), Err(EXIT_FAILURE));

    let want = [
      &format!(
        "[1/2] \u{1F4BE} Loading metainfo from `{}`…",
        torrent.display()
      ),
      &format!(
        "[2/2] \u{1F9EE} Verifying pieces from `{}`…",
        create_env.resolve("foo").display()
      ),
      "a: MD5 checksum mismatch: d16fb36f911f878998c136191af705e (expected \
       90150983cd24fb0d6963f7d28e17f72)",
      "d: 1 byte too long",
      "h: 1 byte too short",
      "l: File missing",
      "p: Expected file but found directory",
      #[cfg(unix)]
      "t: Permission denied (os error 13)",
      "Pieces corrupted.",
      "error: Torrent verification failed.",
      "",
    ]
    .join("\n");

    assert_eq!(verify_env.err(), want);
    assert_eq!(verify_env.out(), "");

    Ok(())
  }

  #[test]
  fn output_color() -> Result<()> {
    let mut create_env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "https://bar",
        "--md5",
      ],
      tree: {
        foo: {
          a: "abc",
          d: "efg",
          h: "ijk",
          l: "mno",
          p: "qrs",
          t: "uvw",
        },
      },
    };

    create_env.run()?;

    let torrent = create_env.resolve("foo.torrent");

    create_env.write("foo/a", "xyz");
    create_env.write("foo/d", "efgg");
    create_env.write("foo/h", "ik");
    create_env.remove_file("foo/l");
    create_env.remove_file("foo/p");
    create_env.create_dir("foo/p");

    #[cfg(unix)]
    {
      use std::os::unix::fs::PermissionsExt;
      let metadata = create_env.metadata("foo/t");
      let mut permissions = metadata.permissions();
      permissions.set_mode(0);
      create_env.set_permissions("foo/t", permissions);
    }

    let mut verify_env = test_env! {
      args: [
        "torrent",
        "verify",
        "--input",
        &torrent,
      ],
      err_style: true,
      tree: {},
    };

    assert_matches!(verify_env.status(), Err(EXIT_FAILURE));

    let style = Style::active();

    fn error(path: &str, message: &str) -> String {
      let style = Style::active();
      format!(
        "{}{}:{} {}",
        style.message().prefix(),
        path,
        style.message().suffix(),
        style.error().paint(message)
      )
    }

    let want = [
      &format!(
        "{} \u{1F4BE} {}",
        style.dim().paint("[1/2]"),
        style
          .message()
          .paint(format!("Loading metainfo from `{}`…", torrent.display()))
      ),
      &format!(
        "{} \u{1F9EE} {}",
        style.dim().paint("[2/2]"),
        style.message().paint(format!(
          "Verifying pieces from `{}`…",
          create_env.resolve("foo").display()
        ))
      ),
      &error(
        "a",
        "MD5 checksum mismatch: d16fb36f911f878998c136191af705e (expected \
         90150983cd24fb0d6963f7d28e17f72)",
      ),
      &error("d", "1 byte too long"),
      &error("h", "1 byte too short"),
      &error("l", "File missing"),
      &error("p", "Expected file but found directory"),
      #[cfg(unix)]
      &error("t", "Permission denied (os error 13)"),
      "Pieces corrupted.",
      &format!(
        "{}{}",
        style.error().paint("error"),
        style.message().paint(": Torrent verification failed.")
      ),
      "",
    ]
    .join("\n");

    assert_eq!(verify_env.err(), want);
    assert_eq!(verify_env.out(), "");

    Ok(())
  }

  // MD5 checksum mismatch: BAD (expected  GOOD)

  #[test]
  fn output_single() -> Result<()> {
    let mut create_env = test_env! {
      args: [
        "torrent",
        "create",
        "--input",
        "foo",
        "--announce",
        "https://bar",
      ],
      tree: {
        foo: "abc",
      },
    };

    create_env.run()?;

    let torrent = create_env.resolve("foo.torrent");

    create_env.write("foo", "abcxyz");

    let mut verify_env = test_env! {
      args: [
        "torrent",
        "verify",
        "--input",
        &torrent,
      ],
      tree: {},
    };

    assert_matches!(verify_env.status(), Err(EXIT_FAILURE));

    let want = [
      &format!(
        "[1/2] \u{1F4BE} Loading metainfo from `{}`…",
        torrent.display()
      ),
      &format!(
        "[2/2] \u{1F9EE} Verifying pieces from `{}`…",
        create_env.resolve("foo").display()
      ),
      "3 bytes too long",
      "Pieces corrupted.",
      "error: Torrent verification failed.",
      "",
    ]
    .join("\n");

    assert_eq!(verify_env.err(), want);
    assert_eq!(verify_env.out(), "");

    Ok(())
  }
}
