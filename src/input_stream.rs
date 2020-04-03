use crate::common::*;

pub(crate) trait InputStream {
  fn buf_read<'a>(&'a mut self) -> Box<dyn BufRead + 'a>;
}

impl InputStream for io::Stdin {
  fn buf_read<'a>(&'a mut self) -> Box<dyn BufRead + 'a> {
    Box::new(self.lock())
  }
}

impl InputStream for io::Empty {
  fn buf_read<'a>(&'a mut self) -> Box<dyn BufRead + 'a> {
    Box::new(BufReader::new(self))
  }
}

impl InputStream for Cursor<Vec<u8>> {
  fn buf_read<'a>(&'a mut self) -> Box<dyn BufRead + 'a> {
    Box::new(BufReader::new(self))
  }
}
