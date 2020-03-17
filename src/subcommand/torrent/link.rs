use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Generate a magnet link from a `.torrent` file.")
)]
pub(crate) struct Link {
  #[structopt(
    long = "input",
    short = "i",
    value_name = "METAINFO",
    help = "Generate magnet link from metainfo at `PATH`.",
    parse(from_os_str)
  )]
  input: PathBuf,
}

struct Infohash {
  inner: Sha1Digest,
}

impl Infohash {
  pub(crate) fn load(path: &Path) -> Result<Infohash, Error> {
    use bendy::decoding::{Decoder, Object};

    let bytes = fs::read(path).context(error::Filesystem { path })?;

    let mut decoder = Decoder::new(&bytes);

    let object = decoder.next_object().unwrap().unwrap();

    if let Object::Dict(mut decoder) = object {
      loop {
        let (key, val) = decoder.next_pair().unwrap().unwrap();

        if key == b"info" {
          if let Object::Dict(infodict) = val {
            let raw = infodict.into_raw().unwrap();
            return Ok(Infohash {
              inner: Sha1Digest::from_data(raw),
            });
          }
        }
      }
    }

    panic!()
  }

  // pub(crate) fn infohash(&self) -> Sha1Digest {
  // }
}

impl Into<Sha1Digest> for Infohash {
  fn into(self) -> Sha1Digest {
    self.inner
  }
}

impl Link {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let input = env.resolve(&self.input);
    let infohash = Infohash::load(&input)?;

    // TODO: make ::new take an infohash
    let link = MagnetLink::new(infohash.into());

    outln!(env, "{}", link.to_url())?;

    // let mut trackers = HashSet::new();
    // for result in metainfo.trackers() {
    //   let tracker = result?;
    //   if !trackers.contains(&tracker) {
    //     trackers.insert(tracker.clone());
    //     link.add_tracker(tracker);
    //   }
    // }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use claim::assert_ok;
  use pretty_assertions::assert_eq;

  #[test]
  fn simple() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "d4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:eeee",
      }
    };

    assert_ok!(env.run());

    // TODO: deduplicate this
    const INFO: &str = "d6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:eee";

    let infohash = Sha1Digest::from_data(INFO.as_bytes());

    assert_eq!(env.out(), format!("magnet:?xt=urn:btih:{}\n", infohash),);
  }

  #[test]
  #[ignore]
  fn announce_url_not_required() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "d4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:eeee",
      }
    };

    assert_ok!(env.run());
  }

  #[test]
  #[ignore]
  fn infohash_correct_with_nonstandard_info_dict() {
    let mut env = test_env! {
      args: [
        "torrent",
        "link",
        "--input",
        "foo.torrent",
      ],
      tree: {
        "foo.torrent": "d8:announce24:https://foo.com/announce4:infod6:lengthi0e4:name3:foo12:piece lengthi1e6:pieces0:eeee",
      }
    };

    assert_ok!(env.run());
  }

  #[test]
  #[ignore]
  fn bad_metainfo_error() {
    todo!()
  }
}
