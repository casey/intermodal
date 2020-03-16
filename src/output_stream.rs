use crate::common::*;

pub(crate) struct OutputStream {
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
      term,
    }
  }

  pub(crate) fn stderr(style: bool) -> OutputStream {
    Self {
      term: style && atty::is(atty::Stream::Stderr),
      stream: Box::new(io::stderr()),
      style,
    }
  }

  #[cfg(test)]
  pub(crate) fn new(stream: Box<dyn Write>, style: bool, term: bool) -> OutputStream {
    Self {
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
}

impl Write for OutputStream {
  fn write(&mut self, data: &[u8]) -> io::Result<usize> {
    self.stream.write(data)
  }

  fn flush(&mut self) -> io::Result<()> {
    self.stream.flush()
  }
}
