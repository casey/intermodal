use crate::common::*;

use syn::{Meta, MetaNameValue, NestedMeta};

macro_rules! assert_attribute_expansion_eq {
  {
    #[$meta:meta]
    $item:item,
    $($expansion:tt)*
  } => {
    {
      let have = expand_attribute!(#[$meta] $item).unwrap().to_string();
      let want = quote::quote!($($expansion)*).to_string();
      assert_eq!(have, want);
    }
  }
}

macro_rules! expand_attribute {
  {
    #[$meta:meta]
    $item:item
  } => {
    {
      let meta = quote::quote!($meta);
      let item = quote::quote!($item);
      $crate::test::expand_attribute(meta, item)
    }
  }
}

pub(crate) fn expand_attribute(meta: TokenStream, item: TokenStream) -> Result<TokenStream, Error> {
  let meta = syn::parse2::<Meta>(meta).unwrap();

  let (path, args) = split_meta(meta);

  match path.as_str() {
    "data::table" => Table::innermost(args, item),
    _ => panic!("attribute `{}` unknown", path),
  }
}

fn split_meta(meta: Meta) -> (String, Vec<NestedMeta>) {
  fn text(path: syn::Path) -> String {
    let mut text = String::new();

    if let Some(_) = path.leading_colon {
      text.push_str("::");
    }

    for (i, segment) in path.segments.iter().enumerate() {
      if i > 0 {
        text.push_str("::");
      }
      text.push_str(&segment.ident.to_string());
    }

    text
  }

  match meta {
    Meta::Path(path) => (text(path), Vec::new()),
    Meta::List(syn::MetaList { path, nested, .. }) => (text(path), nested.into_iter().collect()),
    Meta::NameValue(MetaNameValue { path, lit, .. }) => (text(path), vec![NestedMeta::Lit(lit)]),
  }
}
