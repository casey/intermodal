macro_rules! errln {
  ($env:expr) => {
    writeln!($env.err_mut(), "").context(crate::error::Stderr)
  };
  ($env:expr, $fmt:expr) => {
    writeln!($env.err_mut(), $fmt).context(crate::error::Stderr)
  };
  ($env:expr, $fmt:expr, $($arg:tt)*) => {
    writeln!($env.err_mut(), $fmt, $($arg)*).context(crate::error::Stderr)
  };
}
