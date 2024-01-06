use crate::common::*;

const METAINFO_HELP: &str = "Dump arbitrary bencode in `<INPUT>`. If  `<INPUT>` is `-`, \
                             read metainfo from standard input.";

const INPUT_POSITIONAL: &str = "<INPUT>";

const INPUT_FLAG: &str = "input-flag";

#[derive(StructOpt)]
pub(crate) struct Dump {
  #[structopt(
    name = INPUT_FLAG,
    long = "input",
    short = "i",
    value_name = "INPUT",
    empty_values = false,
    parse(try_from_os_str = InputTarget::try_from_os_str),
    help = METAINFO_HELP,
  )]
  input_flag: Option<InputTarget>,
  #[structopt(
    name = INPUT_POSITIONAL,
    value_name = "INPUT",
    empty_values = false,
    required_unless = INPUT_FLAG,
    conflicts_with = INPUT_FLAG,
    parse(try_from_os_str = InputTarget::try_from_os_str),
    help = METAINFO_HELP,
  )]
  input_positional: Option<InputTarget>,
}

struct Fmt<'a>(&'a Value<'a>);

fn fmt_string(f: &mut Formatter, string: &[u8]) -> fmt::Result {
  if let Ok(string) = str::from_utf8(string) {
    write!(f, "\"{string}\"")?;
  } else {
    write!(f, "0x")?;
    for byte in string {
      write!(f, "{byte:02x}")?;
    }
  }

  Ok(())
}

impl<'a> Display for Fmt<'a> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match &self.0 {
      Value::Integer(integer) => write!(f, "{integer}")?,
      Value::Dict(dict) => {
        write!(f, "{{")?;

        for (i, (key, value)) in dict.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          fmt_string(f, key)?;
          write!(f, ": ")?;
          write!(f, "{}", Fmt(value))?;
        }

        write!(f, "}}")?;
      }
      Value::List(list) => {
        write!(f, "[")?;
        for (i, element) in list.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", Fmt(element))?;
        }
        write!(f, "]")?;
      }
      Value::Bytes(bytes) => fmt_string(f, bytes)?,
    }

    Ok(())
  }
}

impl Dump {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let target = xor_args(
      "input_positional",
      &self.input_positional,
      "input_flag",
      &self.input_flag,
    )?;

    let input = env.read(target.clone())?;

    let value = Value::from_bencode(&input.data).unwrap();

    outln!(env, "{}", Fmt(&value))?;

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn require_input() {
    let mut env = test_env! {
      args: ["torrent", "dump"],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));

    // flag is ok
    let mut env = test_env! {
      args: ["torrent", "dump", "--input", "foo"],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Filesystem { .. }));

    // positional is ok
    let mut env = test_env! {
      args: ["torrent", "dump", "foo"],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Filesystem { .. }));

    // both are fail
    let mut env = test_env! {
      args: ["torrent", "dump", "--input", "foo", "foo"],
      tree: {},
    };
    assert_matches!(env.run(), Err(Error::Clap { .. }));
  }

  #[test]
  fn hex_string() {
    assert_eq!(
      Fmt(&Value::Bytes(b"\x80\x81".to_vec().into())).to_string(),
      "0x8081",
    );
  }

  #[test]
  fn output() {
    fn case(input: &'static str, output: &str) {
      let mut env = test_env! {
        args: ["torrent", "dump", "input.torrent"],
        tree: {
          "input.torrent": input,
        },
      };

      env.assert_ok();

      assert_eq!(env.out(), output);
    }

    case("0:", "\"\"\n");
    case("1:x", "\"x\"\n");

    case("i-123e", "-123\n");
    case("i-1e", "-1\n");
    case("i0e", "0\n");
    case("i123e", "123\n");

    case("le", "[]\n");
    case("li0ei1ei2ee", "[0, 1, 2]\n");

    case("de", "{}\n");
    case("d1:xi0ee", "{\"x\": 0}\n");
    case("d1:xi0e1:yi1ee", "{\"x\": 0, \"y\": 1}\n");
    case("d3:xyzd3:abci0eee", "{\"xyz\": {\"abc\": 0}}\n");
  }
}
