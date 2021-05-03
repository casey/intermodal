use crate::common::*;

#[derive(StructOpt)]
#[structopt(
  help_message(consts::HELP_MESSAGE),
  version_message(consts::VERSION_MESSAGE),
  about("Display information about automatic piece length selection.")
)]
pub(crate) struct PieceLength {}

#[allow(clippy::unused_self)]
impl PieceLength {
  pub(crate) fn run(self, env: &mut Env) -> Result<(), Error> {
    let mut rows: Vec<(String, String, String, String)> = vec![(
      "Content".into(),
      "Piece Length".into(),
      "Count".into(),
      "Piece List Size".into(),
    )];

    for i in 14..51 {
      let content_size = Bytes::from(1u64 << i);

      let piece_length = PieceLengthPicker::from_content_size(content_size);

      let metainfo_size = PieceLengthPicker::metainfo_size(content_size, piece_length);

      let piece_count = PieceLengthPicker::piece_count(content_size, piece_length);

      rows.push((
        content_size.to_string(),
        piece_length.to_string(),
        piece_count.to_string(),
        metainfo_size.to_string(),
      ));
    }

    let mut w = (0, 0, 0, 0);
    for (c0, c1, c2, c3) in &rows {
      w = (
        w.0.max(c0.len()),
        w.1.max(c1.len()),
        w.2.max(c2.len()),
        w.3.max(c3.len()),
      );
    }

    for (content_size, piece_length, metainfo_size, piece_count) in rows {
      outln!(
        env,
        "{:w0$} -> {:w1$} x {:w2$} = {:w3$}",
        content_size,
        piece_length,
        metainfo_size,
        piece_count,
        w0 = w.0,
        w1 = w.1,
        w2 = w.2,
        w3 = w.3,
      )?;
    }

    Ok(())
  }
}
