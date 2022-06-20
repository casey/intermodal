use crate::common::*;

#[derive(Clone)]
pub(crate) struct Capture {
  cursor: Rc<RefCell<Cursor<Vec<u8>>>>,
}

impl Capture {
  pub(crate) fn new() -> Self {
    Self {
      cursor: Rc::new(RefCell::new(Cursor::new(Vec::new()))),
    }
  }

  pub(crate) fn string(&self) -> String {
    str::from_utf8(self.cursor.borrow().get_ref())
      .unwrap()
      .to_owned()
  }

  pub(crate) fn bytes(&self) -> Vec<u8> {
    self.cursor.borrow().get_ref().clone()
  }
}

impl Write for Capture {
  fn write(&mut self, buffer: &[u8]) -> std::result::Result<usize, std::io::Error> {
    self.cursor.borrow_mut().write(buffer)
  }

  fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
    self.cursor.borrow_mut().flush()
  }
}
