use crate::common::*;

pub(crate) trait Invariant<T: Sized>: Sized {
  fn invariant<D: Display>(self, invariant: D) -> Result<T>;

  fn invariant_unwrap<D: Display>(self, invariant: D) -> T {
    #![allow(clippy::unwrap_used)]
    self.invariant(invariant).unwrap()
  }
}

impl<T> Invariant<T> for Option<T> {
  fn invariant<D: Display>(self, invariant: D) -> Result<T> {
    self.ok_or_else(|| Error::internal(format!("Invariant violated: {invariant}")))
  }
}

impl<T, E: std::error::Error> Invariant<T> for Result<T, E> {
  fn invariant<D: Display>(self, invariant: D) -> Result<T> {
    self.map_err(|err| Error::internal(format!("Invariant `{invariant}` violated: {err}")))
  }
}
