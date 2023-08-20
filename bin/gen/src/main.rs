use crate::common::*;

#[macro_use]
mod cmd;

mod arguments;
mod bin;
mod bin_subcommand;
mod command_ext;
mod common;
mod config;
mod error;
mod example;
mod faq;
mod faq_entry;
mod introduction;
mod options;
mod package;
mod project;
mod readme;
mod reference;
mod reference_section;
mod row;
mod slug;
mod subcommand;
mod summary;
mod table;
mod template_ext;

fn main() {
  pretty_env_logger::init();

  if let Err(error) = Arguments::from_args().run() {
    let bold = Style::new().bold();
    let red = Style::new().fg(ansi_term::Color::Red).bold();
    eprintln!("{}: {}", red.paint("error"), bold.paint(error.to_string()));
    process::exit(EXIT_FAILURE);
  }
}
