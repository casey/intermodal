mod bep;
mod common;
mod opt;
mod status;

use crate::common::*;

fn main() -> Result<(), Box<dyn Error>> {
  Opt::from_args().run()
}
