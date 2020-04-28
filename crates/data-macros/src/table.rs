use crate::common::*;

#[derive(FromMeta)]
pub(crate) struct Table {}

impl Attribute for Table {
  type Item = ItemTrait;

  fn expand(&self, item: Self::Item) -> Result<TokenStream, Error> {
    Ok(item.into_token_stream())
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn empty() {
    assert_attribute_expansion_eq!(
      #[data::table]
      trait Foo {},
      trait Foo {}
    );
  }
}
