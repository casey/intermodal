pub(crate) use std::{
  cmp::{Ord, PartialOrd},
  collections::{BTreeMap, BTreeSet},
  env,
  fmt::{self, Display, Formatter},
  fs::{self, File},
  path::{Path, PathBuf},
  process::{Command, ExitStatus, Stdio},
  str,
};

pub(crate) use anyhow::{anyhow, Error};
pub(crate) use askama::Template;
pub(crate) use cargo_toml::Manifest;
pub(crate) use chrono::{DateTime, NaiveDateTime, Utc};
pub(crate) use fehler::{throw, throws};
pub(crate) use git2::{Commit, Repository};
pub(crate) use log::info;
pub(crate) use regex::Regex;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use structopt::StructOpt;
pub(crate) use strum::VariantNames;
pub(crate) use strum_macros::{EnumVariantNames, IntoStaticStr};
pub(crate) use url::Url;

// traits
pub(crate) use crate::{
  command_ext::CommandExt, exit_status_ext::ExitStatusExt, row::Row, slug::Slug,
  template_ext::TemplateExt,
};

// structs and enums
pub(crate) use crate::{
  bin::Bin, changelog::Changelog, config::Config, entry::Entry, example::Example, faq::Faq,
  faq_entry::FaqEntry, introduction::Introduction, kind::Kind, metadata::Metadata, opt::Opt,
  package::Package, project::Project, readme::Readme, reference::Reference,
  reference_section::ReferenceSection, release::Release, subcommand::Subcommand, summary::Summary,
  table::Table,
};
