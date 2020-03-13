macro_rules! errln {
  ($env:expr) => {
    writeln!($env.err, "").context(crate::error::Stderr)
  };
  ($env:expr, $fmt:expr) => {
    writeln!($env.err, $fmt).context(crate::error::Stderr)
  };
  ($env:expr, $fmt:expr, $($arg:tt)*) => {
    writeln!($env.err, $fmt, $($arg)*).context(crate::error::Stderr)
  };
}
