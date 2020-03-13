#[derive(Clone, Copy, Debug)]
pub(crate) struct Style {
  active: bool,
}

impl Style {
  pub(crate) fn active() -> Self {
    Self { active: true }
  }

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

  pub(crate) fn dim(self) -> ansi_term::Style {
    if self.active {
      ansi_term::Style::new().dimmed()
    } else {
      ansi_term::Style::new()
    }
  }
}
