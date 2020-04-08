use crate::common::*;

pub(crate) struct Changelog {
  releases: Vec<Release>,
}

impl Changelog {
  #[throws]
  pub(crate) fn new(repo: &Repository) -> Self {
    let mut current = repo.head()?.peel_to_commit()?;

    let mut entries = Vec::new();

    let mut head = true;

    loop {
      let summary_bytes = current
        .summary_bytes()
        .ok_or_else(|| anyhow!("Commit had no summary"))?;

      let summary = String::from_utf8_lossy(summary_bytes);

      if !summary.starts_with("fixup!") {
        let manifest_bytes = current
          .tree()?
          .get_path("Cargo.toml".as_ref())?
          .to_object(&repo)?
          .as_blob()
          .unwrap()
          .content()
          .to_vec();

        let manifest = Manifest::from_slice(&manifest_bytes)?;

        let entry = Entry::new(&current, manifest.package.unwrap().version.as_ref(), head)?;

        entries.push(entry);
      }

      head = false;

      match current.parent_count() {
        0 => break,
        1 => current = current.parent(0)?,
        _ => throw!(anyhow!("Commit had multiple parents!")),
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
}

impl Display for Changelog {
  #[throws(fmt::Error)]
  fn fmt(&self, f: &mut Formatter) {
    writeln!(f, "Changelog")?;
    writeln!(f, "=========")?;

    for release in &self.releases {
      writeln!(f)?;
      writeln!(f)?;
      write!(f, "{}", release)?;
    }
  }
}
