use crate::common::*;

pub(crate) struct Entry {
  pub(crate) time: DateTime<Utc>,
  pub(crate) version: String,
  pub(crate) metadata: Metadata,
  hash: String,
  author: String,
  summary: String,
  head: bool,
}

impl Entry {
  #[throws]
  pub(crate) fn new(commit: &Commit, version: &str, head: bool, config: &Config) -> Self {
    let time = DateTime::<Utc>::from_utc(
      NaiveDateTime::from_timestamp(commit.time().seconds(), 0),
      Utc,
    );

    let metadata = if let Some(metadata) = config.changelog.get(&commit.id().to_string()) {
      metadata.clone()
    } else {
      Metadata::from_commit(commit)?
    };

    Entry {
      version: version.into(),
      summary: commit.summary().unwrap().into(),
      author: commit.author().to_string(),
      hash: commit.id().to_string(),
      metadata,
      head,
      time,
    }
  }

  fn url(&self) -> String {
    if self.head {
      "https://github.com/casey/intermodal/commits/master".into()
    } else {
      format!("https://github.com/casey/intermodal/commit/{}", self.hash)
    }
  }

  fn permanent_shorthash(&self) -> String {
    if self.head {
      "x".repeat(12)
    } else {
      self.hash[..12].into()
    }
  }
}

impl Display for Entry {
  #[throws(fmt::Error)]
  fn fmt(&self, f: &mut Formatter) {
    let url = self.url();

    let shorthash = self.permanent_shorthash();

    write!(
      f,
      "{} [`{}`]({}) {}",
      self.metadata.kind.emoji(),
      shorthash,
      url,
      self.summary
    )?;

    if let Some(pr) = &self.metadata.pr {
      let n = pr.path_segments().unwrap().last().unwrap();
      write!(f, " ([#{}]({}))", n, pr)?;
    }

    if !self.metadata.fixes.is_empty() {
      write!(f, " - Fixes ")?;

      for (i, issue) in self.metadata.fixes.iter().enumerate() {
        if i > 0 {
          write!(f, ", ")?;
        }
        let n = issue.path_segments().unwrap().last().unwrap();
        write!(f, "[#{}]({})", n, issue)?;
      }
    }

    write!(f, " - _{}_", self.author)?;
  }
}
