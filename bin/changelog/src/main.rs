// TODO:
// - parse URL in fixed:
// - figure out how to check labels before merge

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
