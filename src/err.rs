macro_rules! err {
  ($env:expr, $fmt:expr) => {
    write!($env.err_mut(), $fmt).context(crate::error::Stderr)
  };
  ($env:expr, $fmt:expr, $($arg:tt)*) => {
    write!($env.err_mut(), $fmt, $($arg)*).context(crate::error::Stderr)
  };
}
