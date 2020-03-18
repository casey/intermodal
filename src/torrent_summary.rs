use crate::common::*;

pub(crate) struct TorrentSummary {
  infohash: Infohash,
  metainfo: Metainfo,
  size: Bytes,
}

impl TorrentSummary {
  fn new(metainfo: Metainfo, infohash: Infohash, size: Bytes) -> Self {
    Self {
      infohash,
      metainfo,
      size,
    }
  }

  pub(crate) fn from_metainfo(metainfo: Metainfo) -> Result<Self> {
    let bytes = metainfo.serialize()?;
    let size = Bytes(bytes.len().into_u64());
    let infohash = metainfo.infohash()?;
    Ok(Self::new(metainfo, infohash, size))
  }

  pub(crate) fn from_input(input: &Input) -> Result<Self> {
    let metainfo = Metainfo::from_input(input)?;

    Ok(Self::from_metainfo(metainfo)?)
  }

  pub(crate) fn write(&self, env: &mut Env) -> Result<()> {
    let table = self.table();

    if env.out().is_term() {
      let style = env.out().style();
      table
        .write_human_readable(env.out_mut(), style)
        .context(error::Stdout)?;
    } else {
      table
        .write_tab_delimited(env.out_mut())
        .context(error::Stdout)?;
    }

    Ok(())
  }

  fn table(&self) -> Table {
    let mut table = Table::new();

    table.row("Name", &self.metainfo.info.name);

    if let Some(comment) = &self.metainfo.comment {
      table.row("Comment", comment);
    }

    if let Some(creation_date) = self.metainfo.creation_date {
      #[allow(clippy::as_conversions)]
      table.row(
        "Creation Date",
        Utc.timestamp(
          creation_date
            .min(i64::max_value() as u64)
            .try_into()
            .unwrap(),
          0,
        ),
      );
    }

    if let Some(created_by) = &self.metainfo.created_by {
      table.row("Created By", created_by);
    }

    if let Some(source) = &self.metainfo.info.source {
      table.row("Source", source);
    }

    table.row("Info Hash", self.infohash);

    table.size("Torrent Size", self.size);

    table.size("Content Size", self.metainfo.content_size());

    table.row(
      "Private",
      if self.metainfo.info.private.unwrap_or(false) {
        "yes"
      } else {
        "no"
      },
    );

    if let Some(announce) = &self.metainfo.announce {
      table.row("Tracker", announce);
    }

    if let Some(tiers) = &self.metainfo.announce_list {
      let mut value = Vec::new();

      for (i, tier) in tiers.iter().enumerate() {
        value.push((format!("Tier {}", i + 1), tier.clone()));
      }

      table.tiers("Announce List", value);
    }

    if let Some(nodes) = &self.metainfo.nodes {
      table.list(
        "DHT Nodes",
        nodes
          .iter()
          .map(ToString::to_string)
          .collect::<Vec<String>>(),
      );
    }

    table.size("Piece Size", self.metainfo.info.piece_length);

    table.row("Piece Count", self.metainfo.info.pieces.count());

    match &self.metainfo.info.mode {
      Mode::Single { .. } => {
        table.row("File Count", 1);
        table.row("Files", &self.metainfo.info.name);
      }
      Mode::Multiple { files } => {
        table.row("File Count", files.len());
        table.directory(
          "Files",
          &self.metainfo.info.name,
          files
            .iter()
            .map(|file_info| file_info.path.clone())
            .collect(),
        );
      }
    };

    table
  }
}
