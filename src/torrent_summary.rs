use crate::common::*;

pub(crate) struct TorrentSummary {
  infohash: Infohash,
  metainfo: Metainfo,
  size: Bytes,
}

#[derive(Serialize)]
pub(crate) struct TorrentSummaryJson {
  name: String,
  comment: Option<String>,
  creation_date: Option<u64>,
  created_by: Option<String>,
  source: Option<String>,
  info_hash: String,
  torrent_size: u64,
  content_size: u64,
  private: bool,
  tracker: Option<String>,
  announce_list: Vec<Vec<String>>,
  update_url: Option<String>,
  dht_nodes: Vec<String>,
  piece_size: u64,
  piece_count: usize,
  file_count: usize,
  files: Vec<String>,
}

impl TorrentSummary {
  fn new(metainfo: Metainfo, infohash: Infohash, size: Bytes) -> Self {
    Self {
      infohash,
      metainfo,
      size,
    }
  }

  /// See `Info::infohash_lossy` for details on when this function is lossy.
  pub(crate) fn from_metainfo_lossy(metainfo: Metainfo) -> Result<Self> {
    let bytes = metainfo.serialize()?;
    let size = Bytes(bytes.len().into_u64());
    let infohash = metainfo.infohash_lossy()?;
    Ok(Self::new(metainfo, infohash, size))
  }

  pub(crate) fn from_input(input: &Input) -> Result<Self> {
    let metainfo = Metainfo::from_input(input)?;
    let infohash = Infohash::from_input(input)?;
    let size = Bytes(input.data.len().into_u64());

    Ok(Self::new(metainfo, infohash, size))
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
        Utc
          .timestamp_opt(
            creation_date
              .min(i64::MAX as u64)
              .try_into()
              .invariant_unwrap("min with i64 is always valid i64"),
            0,
          )
          .unwrap(),
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

    if let Some(update_url) = &self.metainfo.info.update_url {
      table.row("Update URL", update_url);
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

  pub(crate) fn write_json(&self, env: &mut Env) -> Result<()> {
    let data = self.torrent_summary_data();
    let json = serde_json::to_string(&data).context(error::JsonSerialize)?;
    outln!(env, "{}", json)?;
    Ok(())
  }

  fn torrent_summary_data(&self) -> TorrentSummaryJson {
    let (file_count, files) = match &self.metainfo.info.mode {
      Mode::Single { .. } => (1, vec![self.metainfo.info.name.clone()]),
      Mode::Multiple { files } => (
        files.len(),
        files
          .iter()
          .map(|file_info| {
            format!(
              "{}",
              file_info
                .path
                .absolute(Path::new(&self.metainfo.info.name))
                .as_path()
                .display()
            )
          })
          .collect(),
      ),
    };

    TorrentSummaryJson {
      name: self.metainfo.info.name.clone(),
      comment: self.metainfo.comment.clone(),
      creation_date: self.metainfo.creation_date,
      created_by: self.metainfo.created_by.clone(),
      source: self.metainfo.info.source.clone(),
      info_hash: self.infohash.to_string(),
      torrent_size: self.size.count(),
      content_size: self.metainfo.content_size().count(),
      private: self.metainfo.info.private.unwrap_or_default(),
      tracker: self.metainfo.announce.clone(),
      announce_list: self.metainfo.announce_list.clone().unwrap_or_default(),
      update_url: self
        .metainfo
        .info
        .update_url
        .as_ref()
        .map(ToString::to_string),
      dht_nodes: self
        .metainfo
        .nodes
        .as_ref()
        .unwrap_or(&Vec::new())
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<String>>(),
      piece_size: self.metainfo.info.piece_length.count(),
      piece_count: self.metainfo.info.pieces.count(),
      file_count,
      files,
    }
  }
}
