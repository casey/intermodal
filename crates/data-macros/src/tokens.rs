use crate::common::*;

pub(crate) trait Tokens {
  fn tokens(self) -> TokenStream;
}

impl Tokens for TokenStream {
  fn tokens(self) -> TokenStream {
    self
  }
}

impl<T: Tokens, E: Tokens> Tokens for Result<T, E> {
  fn tokens(self) -> TokenStream {
    match self {
      Ok(t) => t.tokens(),
      Err(e) => e.tokens(),
    }
  }
}
