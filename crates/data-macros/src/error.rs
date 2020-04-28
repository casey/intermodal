use crate::common::*;

#[derive(Debug)]
pub(crate) enum Error {
  Parse(darling::Error),
  Syn(syn::Error),
}

impl From<darling::Error> for Error {
  fn from(error: darling::Error) -> Error {
    Error::Parse(error)
  }
}

impl From<syn::Error> for Error {
  fn from(error: syn::Error) -> Error {
    Error::Syn(error)
  }
}

impl Tokens for Error {
  fn tokens(self) -> TokenStream {
    match self {
      Error::Parse(error) => error.write_errors(),
      Error::Syn(error) => error.to_compile_error(),
    }
  }
}
