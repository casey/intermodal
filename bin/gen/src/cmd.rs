macro_rules! cmd {
  {
    $bin:expr,
    $($arg:expr),*
    $(,)?
  } => {
    {
      let mut command = Command::new($bin);

      $(
        command.arg($arg);
      )*

      command
    }
  }
}
