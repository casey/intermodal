use crate::common::*;

pub(crate) trait Attribute: FromMeta {
  type Item: syn::parse::Parse;

  fn attribute(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
  ) -> proc_macro::TokenStream {
    Self::inner(attr.into(), item.into()).tokens().into()
  }

  fn inner(attr: TokenStream, item: TokenStream) -> Result<TokenStream, Error> {
    let args = Punctuated::<NestedMeta, token::Comma>::parse_terminated
      .parse2(attr)?
      .into_iter()
      .collect();

    Self::innermost(args, item)
  }

  fn innermost(args: Vec<NestedMeta>, item: TokenStream) -> Result<TokenStream, Error> {
    let item = syn::parse2::<Self::Item>(item)?;
    Self::from_list(&args)?.expand(item)
  }

  fn expand(&self, item: Self::Item) -> Result<TokenStream, Error>;
}
