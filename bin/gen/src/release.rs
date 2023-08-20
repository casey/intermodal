use crate::common::*;

pub(crate) struct Release {
  pub(crate) version: Option<String>,
  pub(crate) time: DateTime<Utc>,
  pub(crate) entries: Vec<Entry>,
}

impl Release {
  #[throws]
  pub(crate) fn render(&self, lines: &mut Vec<String>) {
    let time = self.time.format("%Y-%m-%d");

    let header = match &self.version {
      Some(version) => format!(
        "[v{}](https://github.com/casey/intermodal/releases/tag/v{}) - {}",
        version, version, time,
      ),
      None => format!("UNRELEASED - {}", time),
    };

    lines.push(header.clone());
    lines.push("-".repeat(header.len()));

    for entry in &self.entries {
      entry.render(lines)?;
    }
  }
}
