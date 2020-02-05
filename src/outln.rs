macro_rules! outln {
  ($env:expr) => {
    writeln!($env.out, "").context(crate::error::Stderr)?;
  };
  ($env:expr, $fmt:expr) => {
    writeln!($env.out, $fmt).context(crate::error::Stderr)?;
  };
  ($env:expr, $fmt:expr, $($arg:tt)*) => {
    writeln!($env.out, $fmt, $($arg)*).context(crate::error::Stderr)?;
  };
}
