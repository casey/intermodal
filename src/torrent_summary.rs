use crate::common::*;

pub(crate) struct TorrentSummary {
  metainfo: Metainfo,
  infohash: sha1::Digest,
  size: Bytes,
}

impl TorrentSummary {
  pub(crate) fn load(path: &Path) -> Result<Self, Error> {
    let bytes = fs::read(path).context(error::Filesystem { path })?;

    let metainfo = Metainfo::deserialize(path, &bytes)?;

    let value = bencode::Value::decode(&bytes).unwrap();

    let infohash = if let bencode::Value::Dict(items) = value {
      let info = items
        .iter()
        .find(|(key, _value)| key == b"info")
        .unwrap()
        .1
        .encode();
      Sha1::from(info).digest()
    } else {
      panic!()
    };

    let metadata = path.metadata().context(error::Filesystem { path })?;

    Ok(Self {
      size: Bytes(metadata.len().into()),
      infohash,
      metainfo,
    })
  }

  pub(crate) fn table(&self) -> Table {
    let mut table = Table::new();

    table.row("Name", &self.metainfo.info.name);

    if let Some(comment) = &self.metainfo.comment {
      table.row("Comment", comment);
    }

    if let Some(creation_date) = self.metainfo.creation_date {
      table.row(
        "Created",
        Utc.timestamp(
          creation_date
            .min(i64::max_value() as u64)
            .try_into()
            .unwrap(),
          0,
        ),
      );
    }

    table.row("Info Hash", self.infohash);

    table.row("Torrent Size", self.size);

    table.row("Content Size", self.metainfo.info.mode.total_size());

    table.row(
      "Private",
      if self.metainfo.info.private.unwrap_or(0) == 1 {
        "yes"
      } else {
        "no"
      },
    );

    match &self.metainfo.announce_list {
      Some(tiers) => {
        let mut value = Vec::new();
        value.push(("Main".to_owned(), vec![self.metainfo.announce.clone()]));

        for (i, tier) in tiers.iter().enumerate() {
          value.push((format!("Tier {}", i + 1), tier.clone()));
        }

        table.tiers("Trackers", value);
      }
      None => table.row("Tracker", &self.metainfo.announce),
    }

    table.row("Piece Size", Bytes::from(self.metainfo.info.piece_length));

    table.row("Piece Count", self.metainfo.info.pieces.len() / 20);

    table.row(
      "File Count",
      match &self.metainfo.info.mode {
        Mode::Single { .. } => 1,
        Mode::Multiple { files } => files.len(),
      },
    );

    table
  }
}
