use crate::common::*;

pub(crate) trait Print {
  fn print(&self, stream: &mut OutputStream) -> io::Result<()>;

  fn println(&self, stream: &mut OutputStream) -> io::Result<()> {
    self.print(stream)?;
    writeln!(stream)?;
    Ok(())
  }
}
