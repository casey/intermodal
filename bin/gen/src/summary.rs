use crate::common::*;

#[derive(Template)]
#[template(path = "SUMMARY.md")]
pub(crate) struct Summary {
  pub(crate) commands: String,
}

impl Summary {
  pub(crate) fn new(bin: &Bin) -> Summary {
    let mut lines = Vec::new();

    lines.push("- [Commands](./commands.md)".to_string());

    for subcommand in &bin.subcommands {
      let slug = subcommand.slug();

      lines.push(format!(
        "  - [`{}`](./commands/{}.md)",
        subcommand.command_line(),
        slug
      ))
    }

    Summary {
      commands: lines.join("\n"),
    }
  }
}
