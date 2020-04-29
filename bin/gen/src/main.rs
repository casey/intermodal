use crate::common::*;

#[macro_use]
mod cmd;

mod bin;
mod changelog;
mod command_ext;
mod common;
mod config;
mod entry;
mod error;
mod example;
mod faq;
mod faq_entry;
mod introduction;
mod kind;
mod metadata;
mod opt;
mod package;
mod project;
mod readme;
mod reference;
mod reference_section;
mod release;
mod row;
mod slug;
mod subcommand;
mod summary;
mod table;
mod template_ext;

fn main() {
  pretty_env_logger::init();

  if let Err(error) = Opt::from_args().run() {
    eprintln!("{}", error);
    process::exit(EXIT_FAILURE);
  }
}
