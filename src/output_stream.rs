use crate::common::*;

pub(crate) struct OutputStream {
  active: bool,
  stream: Box<dyn Write>,
  style: bool,
  term: bool,
}

impl OutputStream {
  pub(crate) fn stdout(style: bool) -> OutputStream {
    let term = atty::is(atty::Stream::Stdout);
    Self {
      stream: Box::new(io::stdout()),
      style: style && term,
      active: true,
      term,
    }
  }

  pub(crate) fn stderr(style: bool) -> OutputStream {
    Self {
      term: style && atty::is(atty::Stream::Stderr),
      stream: Box::new(io::stderr()),
      active: true,
      style,
    }
  }

  #[cfg(test)]
  pub(crate) fn new(stream: Box<dyn Write>, style: bool, term: bool, active: bool) -> OutputStream {
    Self {
      active,
      stream,
      style,
      term,
    }
  }

  pub(crate) fn set_use_color(&mut self, use_color: UseColor) {
    match use_color {
      UseColor::Always => self.style = true,
      UseColor::Auto => {}
      UseColor::Never => self.style = false,
    }
  }

  pub(crate) fn set_is_term(&mut self, term: bool) {
    self.term = term;
  }

  pub(crate) fn is_term(&self) -> bool {
    self.term
  }

  pub(crate) fn is_styled(&self) -> bool {
    self.style
  }

  pub(crate) fn is_styled_term(&self) -> bool {
    self.is_styled() && self.is_term()
  }

  pub(crate) fn style(&self) -> Style {
    Style::from_active(self.style)
  }

  pub(crate) fn set_active(&mut self, active: bool) {
    self.active = active;
  }
}

impl Write for OutputStream {
  fn write(&mut self, data: &[u8]) -> io::Result<usize> {
    if self.active {
      self.stream.write(data)
    } else {
      Ok(data.len())
    }
  }

  fn flush(&mut self) -> io::Result<()> {
    self.stream.flush()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn active() {
    let capture = Capture::new();
    let mut stream = OutputStream::new(Box::new(capture.clone()), false, false, true);
    stream.write_all("hello".as_bytes()).unwrap();
    assert_eq!(capture.string(), "hello");
  }

  #[test]
  fn inactive() {
    let capture = Capture::new();
    let mut stream = OutputStream::new(Box::new(capture.clone()), false, false, false);
    stream.write_all("hello".as_bytes()).unwrap();
    assert_eq!(capture.string(), "");
  }
}
