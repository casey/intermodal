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

  fn shorthash(&self) -> String {
    self.hash[..12].into()
  }

  #[throws]
  pub(crate) fn render(&self, lines: &mut Vec<String>, book: bool) {
    let mut line = "- ".to_string();

    let url = self.url();

    line.push_str(&format!(
      "{} [`{}`]({}) {}",
      if book {
        self.metadata.kind.emoji_character()
      } else {
        self.metadata.kind.emoji_name()
      },
      self.shorthash(),
      url,
      self.summary
    ));

    if let Some(pr) = &self.metadata.pr {
      let n = pr.path_segments().unwrap().last().unwrap();
      line.push_str(&format!(" ([#{}]({}))", n, pr));
    }

    if !self.metadata.fixes.is_empty() {
      line.push_str(" - Fixes ");

      for (i, issue) in self.metadata.fixes.iter().enumerate() {
        if i > 0 {
          line.push_str(", ");
        }
        let n = issue.path_segments().unwrap().last().unwrap();
        line.push_str(&format!("[#{}]({})", n, issue));
      }
    }

    line.push_str(&format!(" - _{}_", self.author));

    lines.push(line);
  }
}
