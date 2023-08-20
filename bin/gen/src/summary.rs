use crate::common::*;

#[derive(Template)]
#[template(path = "SUMMARY.md")]
pub(crate) struct Summary {
  pub(crate) commands: String,
  pub(crate) references: String,
}

impl Summary {
  pub(crate) fn new(project: &Project) -> Summary {
    let mut commands = Index::new("Commands", "./commands.md");

    for subcommand in &project.bin.subcommands {
      commands.entry(
        &format!("`{}`", subcommand.command_line()),
        &format!("./commands/{}.md", subcommand.slug()),
      );
    }

    let mut references = Index::new("References", "./references.md");

    for section in &project.config.references {
      references.entry(&section.title, &section.path());
    }

    Summary {
      commands: commands.text(),
      references: references.text(),
    }
  }
}

struct Index {
  title: String,
  entries: Vec<String>,
}

impl Index {
  fn new(title: &str, path: &str) -> Self {
    Self {
      title: format!("- [{}]({})", title, path),
      entries: Vec::new(),
    }
  }

  fn entry(&mut self, title: &str, path: &str) {
    self.entries.push(format!("  - [{}]({})", title, path));
  }

  fn text(self) -> String {
    let mut text = self.title;

    for entry in self.entries {
      text.push('\n');
      text.push_str(&entry);
    }

    text
  }
}
