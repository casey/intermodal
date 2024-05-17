pub(crate) use std::{
  cmp::{Ord, PartialOrd},
  collections::BTreeSet,
  env,
  fmt::{self, Display, Formatter},
  fs::{self, File},
  io,
  ops::Deref,
  path::{Path, PathBuf},
  process::{self, Command, ExitStatus, Stdio},
  str,
};

pub(crate) use ansi_term::Style;
pub(crate) use askama::Template;
pub(crate) use fehler::{throw, throws};
pub(crate) use git2::Repository;
pub(crate) use libc::EXIT_FAILURE;
pub(crate) use log::info;
pub(crate) use regex::Regex;
pub(crate) use serde::Deserialize;
pub(crate) use snafu::{ResultExt, Snafu};
pub(crate) use std::string::FromUtf8Error;
pub(crate) use structopt::StructOpt;
pub(crate) use url::Url;

// modules
pub(crate) use crate::error;

// traits
pub(crate) use crate::{command_ext::CommandExt, row::Row, slug::Slug, template_ext::TemplateExt};

// structs and enums
pub(crate) use crate::{
  arguments::Arguments, bin::Bin, bin_subcommand::BinSubcommand, config::Config, error::Error,
  example::Example, faq::Faq, faq_entry::FaqEntry, introduction::Introduction, options::Options,
  package::Package, project::Project, readme::Readme, reference::Reference,
  reference_section::ReferenceSection, subcommand::Subcommand, summary::Summary, table::Table,
};
