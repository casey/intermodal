#[derive(Clone, Copy, Debug)]
pub(crate) struct Style {
  active: bool,
}

impl Style {
  pub(crate) fn from_active(active: bool) -> Self {
    Self { active }
  }

  #[cfg(test)]
  pub(crate) fn active() -> Self {
    Self { active: true }
  }

  #[cfg(test)]
  pub(crate) fn inactive() -> Self {
    Self { active: false }
  }

  pub(crate) fn message(self) -> ansi_term::Style {
    if self.active {
      ansi_term::Style::new().bold()
    } else {
      ansi_term::Style::new()
    }
  }

  pub(crate) fn error(self) -> ansi_term::Style {
    if self.active {
      ansi_term::Style::new().fg(ansi_term::Color::Red).bold()
    } else {
      ansi_term::Style::new()
    }
  }

  pub(crate) fn blue(self) -> ansi_term::Style {
    if self.active {
      ansi_term::Style::new().fg(ansi_term::Color::Blue)
    } else {
      ansi_term::Style::new()
    }
  }

  pub(crate) fn good(self) -> ansi_term::Style {
    if self.active {
      ansi_term::Style::new().fg(ansi_term::Color::Green).bold()
    } else {
      ansi_term::Style::new()
    }
  }

  pub(crate) fn dim(self) -> ansi_term::Style {
    if self.active {
      ansi_term::Style::new().dimmed()
    } else {
      ansi_term::Style::new()
    }
  }
}
