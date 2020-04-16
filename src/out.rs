macro_rules! out {
  ($env:expr) => {
    write!($env.out_mut(), "").context(crate::error::Stdout)
  };
  ($env:expr, $fmt:expr) => {
    write!($env.out_mut(), $fmt).context(crate::error::Stdout)
  };
  ($env:expr, $fmt:expr, $($arg:tt)*) => {
    write!($env.out_mut(), $fmt, $($arg)*).context(crate::error::Stdout)
  };
}
