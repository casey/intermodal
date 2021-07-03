use crate::common::*;

pub(crate) struct Linter {
  allowed: BTreeSet<Lint>,
}

impl Linter {
  pub(crate) fn new() -> Linter {
    Linter {
      allowed: BTreeSet::new(),
    }
  }

  pub(crate) fn allow(&mut self, allowed: impl IntoIterator<Item = Lint>) {
    self.allowed.extend(allowed);
  }

  pub(crate) fn is_allowed(&self, lint: Lint) -> bool {
    self.allowed.contains(&lint)
  }

  pub(crate) fn is_denied(&self, lint: Lint) -> bool {
    !self.is_allowed(lint)
  }
}
