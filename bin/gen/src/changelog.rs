use crate::common::*;

pub(crate) struct Changelog {
  releases: Vec<Release>,
}

impl Changelog {
  #[throws]
  pub(crate) fn new(project: &Project) -> Self {
    let mut current = project.repo.head()?.peel_to_commit()?;

    let mut entries = Vec::new();

    let mut head = true;

    loop {
      let summary_bytes = current
        .summary_bytes()
        .ok_or_else(|| Error::CommitSummery { hash: current.id() })?;

      let summary = String::from_utf8_lossy(summary_bytes);

      if !summary.starts_with("fixup!") {
        let manifest_bytes = current
          .tree()?
          .get_path("Cargo.toml".as_ref())?
          .to_object(&project.repo)?
          .as_blob()
          .unwrap()
          .content()
          .to_vec();

        let manifest = Manifest::from_slice(&manifest_bytes)?;

        let entry = Entry::new(
          &current,
          manifest.package.unwrap().version.as_ref(),
          head,
          &project.config,
        )?;

        entries.push(entry);
      }

      head = false;

      match current.parent_count() {
        0 => break,
        1 => current = current.parent(0)?,
        other => throw!(Error::CommitParents {
          hash: current.id(),
          parents: other
        }),
      }
    }

    entries.reverse();

    let mut releases = Vec::new();

    let mut last_version = entries[0].version.clone();

    let mut unreleased = Vec::new();

    for entry in entries {
      if entry.version != last_version {
        let version = entry.version.clone();
        let time = entry.time;

        unreleased.push(entry);

        releases.push(Release {
          version: Some(version.clone()),
          time,
          entries: unreleased,
        });

        last_version = version;

        unreleased = Vec::new();
      } else {
        unreleased.push(entry);
      }
    }

    if !unreleased.is_empty() {
      releases.push(Release {
        version: None,
        time: Utc::now(),
        entries: unreleased,
      });
    }

    releases.reverse();

    for release in &mut releases {
      release.entries.reverse();
    }

    Self { releases }
  }

  #[throws]
  pub(crate) fn render(&self, book: bool) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push("Changelog".into());
    lines.push("=========".into());

    for release in &self.releases {
      lines.push("".into());
      lines.push("".into());
      release.render(&mut lines, book)?;
    }

    let mut text = lines.join("\n");
    text.push('\n');
    text
  }
}
