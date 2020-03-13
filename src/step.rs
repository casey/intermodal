use crate::common::*;

pub(crate) trait Step {
  fn print(&self, env: &mut Env) -> Result<(), Error> {
    let style = env.err_style();
    let dim = style.dim();
    let message = style.message();

    err!(
      env,
      "{}[{}/{}]{} ",
      dim.prefix(),
      self.n(),
      Self::total(),
      dim.suffix()
    )?;

    err!(env, "{}{} ", message.prefix(), self.symbol())?;

    self.write_message(&mut env.err).context(error::Stderr)?;

    errln!(env, "{}", message.suffix())?;

    Ok(())
  }

  fn n(&self) -> usize;

  fn total() -> usize;

  fn write_message(&self, write: &mut dyn Write) -> io::Result<()>;

  fn symbol(&self) -> &str;
}
