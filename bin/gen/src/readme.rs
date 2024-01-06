use crate::common::*;

#[derive(Template)]
#[template(path = "README.md")]
pub(crate) struct Readme {
  pub(crate) table_of_contents: String,
  pub(crate) packages: Table<Package>,
}

const HEADING_PATTERN: &str = "(?m)^(?P<MARKER>#+) (?P<TEXT>.*)$";

impl Readme {
  #[throws]
  pub(crate) fn load(config: &Config, template: &Path) -> Readme {
    let text = fs::read_to_string(template).context(error::Filesystem { path: template })?;

    let header_re = Regex::new(HEADING_PATTERN)?;

    let mut lines = Vec::new();

    for captures in header_re.captures_iter(&text).skip(2) {
      let marker = captures.name("MARKER").unwrap().as_str();
      let text = captures.name("TEXT").unwrap().as_str();
      let level = marker.len();
      let indentation = " ".repeat((level - 2) * 2);
      let slug = text
        .to_lowercase()
        .replace(' ', "-")
        .replace(['.', '&'], "");
      lines.push(format!("{}- [{}](#{})", indentation, text, slug));
    }

    Readme {
      table_of_contents: lines.join("\n"),
      packages: Table::new(config.packages.clone()),
    }
  }
}
