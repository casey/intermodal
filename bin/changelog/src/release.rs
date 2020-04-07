use crate::common::*;

pub(crate) struct Release {
  pub(crate) version: Option<String>,
  pub(crate) time: DateTime<Utc>,
  pub(crate) entries: Vec<Entry>,
}

impl Display for Release {
  #[throws(fmt::Error)]
  fn fmt(&self, f: &mut Formatter) {
    let time = self.time.format("%Y-%m-%d");

    let header = match &self.version {
      Some(version) => format!(
        "[v{}](https://github.com/casey/intermodal/releases/tag/v{}) - {}",
        version, version, time,
      ),
      None => format!("UNRELEASED - {}", time),
    };

    writeln!(f, "{}", header)?;
    writeln!(f, "{}", "-".repeat(header.len()))?;

    for entry in &self.entries {
      writeln!(f, "- {}", entry)?;
    }
  }
}
