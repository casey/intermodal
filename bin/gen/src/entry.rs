use crate::common::*;

pub(crate) struct Entry {
  pub(crate) time: DateTime<Utc>,
  pub(crate) version: String,
  pub(crate) metadata: Metadata,
  hash: String,
  author: String,
  summary: String,
}

impl Entry {
  #[throws]
  pub(crate) fn new(commit: &Commit, version: &str, config: &Config) -> Self {
    let time = DateTime::<Utc>::from_utc(
      NaiveDateTime::from_timestamp(commit.time().seconds(), 0),
      Utc,
    );

    let metadata = if let Some(metadata) = config.changelog.get(&commit.id().to_string()) {
      metadata.clone()
    } else {
      Metadata::from_commit(commit)?
    };

    fn bytes_to_option(bytes: &[u8]) -> Option<String> {
      let string = String::from_utf8_lossy(bytes).into_owned();
      if string.is_empty() {
        None
      } else {
        Some(string)
      }
    }

    let name = bytes_to_option(commit.author().name_bytes());
    let email = bytes_to_option(commit.author().email_bytes());

    let author = match (name, email) {
      (Some(name), Some(email)) => format!("[{}](mailto:{})", name, email),
      (Some(name), None) => name,
      (None, Some(email)) => email,
      (None, None) => String::from("Anonymous"),
    };

    Entry {
      hash: commit.id().to_string(),
      summary: commit.summary().unwrap().into(),
      version: version.into(),
      author,
      metadata,
      time,
    }
  }

  fn url(&self) -> String {
    format!("https://github.com/casey/intermodal/commit/{}", self.hash)
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
