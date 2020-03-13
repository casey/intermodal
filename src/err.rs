macro_rules! err {
  ($env:expr, $fmt:expr) => {
    write!($env.err, $fmt).context(crate::error::Stderr)
  };
  ($env:expr, $fmt:expr, $($arg:tt)*) => {
    write!($env.err, $fmt, $($arg)*).context(crate::error::Stderr)
  };
}
