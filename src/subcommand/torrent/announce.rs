use crate::common::*;

const INPUT_HELP: &str =
  "Read torrent metainfo from `INPUT`. If `INPUT` is `-`, read metainfo from standard input.";

const INPUT_FLAG: &str = "input-flag";

const INPUT_POSITIONAL: &str = "<INPUT>";

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Announce a .torrent file.")
)]
pub(crate) struct Announce {
  #[structopt(
    name = INPUT_FLAG,
    long = "input",
    short = "i",
    value_name = "INPUT",
    empty_values(false),
    parse(try_from_os_str = InputTarget::try_from_os_str),
    help = INPUT_HELP,
  )]
  input_flag: Option<InputTarget>,
  #[structopt(
    name = INPUT_POSITIONAL,
    value_name = "INPUT",
    empty_values(false),
    parse(try_from_os_str = InputTarget::try_from_os_str),
    required_unless = INPUT_FLAG,
    conflicts_with = INPUT_FLAG,
    help = INPUT_HELP,
  )]
  input_positional: Option<InputTarget>,
}

impl Announce {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let target = xor_args(
      "input_flag",
      &self.input_flag,
      "input_positional",
      &self.input_positional,
    )?;
    let input = env.read(target)?;

    let mut peer_list = Vec::new();
    let mut trackers = Vec::new();
    let infohash = Infohash::from_input(&input)?;
    let metainfo = Metainfo::from_input(&input)?;

    for result in metainfo.trackers() {
      match result {
        Ok(tracker) => trackers.push(tracker),
        Err(err) => errln!(env, "Skipping malformed tracker URL: {}", err)?,
      }
    }

    if trackers.is_empty() {
      return Err(Error::MetainfoMissingTrackers);
    }

    for tracker in trackers {
      if tracker.scheme() != "udp" {
        errln!(
          env,
          "Only UDP trackers are supported; skipping {}.",
          tracker
        )?;
        continue;
      }

      let hostport = if let Some(hostport) = HostPort::from_url(&tracker) {
        hostport
      } else {
        errln!(env, "Tracker URL `{}` is not well formed.", tracker)?;
        continue;
      };

      let client = match tracker::Client::connect(&hostport) {
        Err(err) => {
          errln!(env, "Couldn't connect to tracker: {}", err)?;
          continue;
        }
        Ok(client) => client,
      };

      match client.announce(infohash) {
        Ok(subswarm) => {
          errln!(env, "Successful announce to tracker `{}`.", tracker)?;
          peer_list.extend(subswarm);
        }
        Err(err) => errln!(env, "Announce failed with tracker `{}`: {}", tracker, err)?,
      }
    }

    for peer in &peer_list {
      outln!(env, "{}", peer)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn input_required() {
    test_env! {
      args: [
        "torrent",
        "announce",
      ],
      tree: {
      },
      matches: Err(Error::Clap { .. }),
    };
  }

  #[test]
  fn input_arguments_positional() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "foo",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(error::Error::Filesystem { .. }));
  }

  #[test]
  fn input_arguments_flag() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "--input",
        "foo",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(error::Error::Filesystem { .. }));
  }

  #[test]
  fn input_arguments_conflict() {
    let mut env = test_env! {
      args: [
        "torrent",
        "announce",
        "--input",
        "foo",
        "bar",
      ],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }
}
