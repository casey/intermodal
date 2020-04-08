use crate::common::*;

mod changelog;
mod common;
mod entry;
mod kind;
mod metadata;
mod opt;
mod release;

#[throws]
fn main() {
  Opt::from_args().run()?;
}
