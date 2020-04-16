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
pub(crate) use regex::Regex;
pub(crate) use serde::{Deserialize, Serialize};
pub(crate) use structopt::StructOpt;
pub(crate) use strum::VariantNames;
pub(crate) use strum_macros::{EnumVariantNames, IntoStaticStr};
pub(crate) use url::Url;

pub(crate) use crate::{
  bin::Bin, changelog::Changelog, command_ext::CommandExt, config::Config, entry::Entry,
  example::Example, exit_status_ext::ExitStatusExt, introduction::Introduction, kind::Kind,
  metadata::Metadata, opt::Opt, project::Project, readme::Readme, release::Release,
  subcommand::Subcommand, summary::Summary,
};
