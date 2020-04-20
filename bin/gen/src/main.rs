use crate::common::*;

#[macro_use]
mod cmd;

mod bin;
mod changelog;
mod command_ext;
mod common;
mod config;
mod entry;
mod example;
mod exit_status_ext;
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

#[throws]
fn main() {
  pretty_env_logger::init();

  let project = Project::load()?;

  Opt::from_args().run(&project)?;
}
