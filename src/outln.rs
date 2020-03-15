macro_rules! outln {
  ($env:expr) => {
    writeln!($env.out_mut(), "").context(crate::error::Stdout)
  };
  ($env:expr, $fmt:expr) => {
    writeln!($env.out_mut(), $fmt).context(crate::error::Stdout)
  };
  ($env:expr, $fmt:expr, $($arg:tt)*) => {
    writeln!($env.out_mut(), $fmt, $($arg)*).context(crate::error::Stdout)
  };
}
