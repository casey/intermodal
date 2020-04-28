use crate::common::*;

#[cfg(test)]
#[macro_use]
mod test;

mod attribute;
mod common;
mod error;
mod table;
mod tokens;

#[proc_macro_attribute]
pub fn table(
  attr: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  Table::attribute(attr, item)
}
