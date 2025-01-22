use crate::common::*;

use super::Create;

pub(crate) struct CreateContent {
  pub(crate) files: Option<Files>,
  pub(crate) piece_length: Bytes,
  pub(crate) progress_bar: ProgressBar,
  pub(crate) name: String,
  pub(crate) output: OutputTarget,
}

impl CreateContent {
  pub(crate) fn from_create(create: &Create, input: &InputTarget, env: &mut Env) -> Result<Self> {
    match input {
      InputTarget::Path(path) => {
        let spinner = if env.err().is_styled_term() {
          let style = ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg:.bold}…")
            .tick_chars(consts::TICK_CHARS);

          Some(ProgressBar::new_spinner().with_style(style))
        } else {
          None
        };

        let files = Walker::new(&env.resolve(path)?)
          .include_junk(create.include_junk)
          .include_hidden(create.include_hidden)
          .ignore(create.ignore)
          .follow_symlinks(create.follow_symlinks)
          .sort_by(create.sort_by.clone())
          .globs(&create.globs)?
          .spinner(spinner)
          .files()?;

        let piece_length = create
          .piece_length
          .unwrap_or_else(|| PieceLengthPicker::from_content_size(files.total_size()));

        let style = ProgressStyle::default_bar()
          .template(consts::PROGRESS_STYLE)
          .tick_chars(consts::TICK_CHARS)
          .progress_chars(consts::PROGRESS_CHARS);

        let progress_bar = ProgressBar::new(files.total_size().count()).with_style(style);

        let resolved = env.resolve(path)?;

        let filename = resolved
          .file_name()
          .ok_or_else(|| Error::FilenameExtract { path: path.clone() })?;

        let name = match &create.name {
          Some(name) => name.clone(),
          None => filename
            .to_str()
            .ok_or_else(|| Error::FilenameDecode {
              filename: PathBuf::from(filename),
            })?
            .to_owned(),
        };

        let output = create
          .output
          .clone()
          .unwrap_or_else(|| OutputTarget::Path(Self::torrent_path(path, &name)));

        Ok(Self {
          files: Some(files),
          piece_length,
          progress_bar,
          name,
          output,
        })
      }

      InputTarget::Stdin => {
        let files = None;
        let piece_length = create.piece_length.unwrap_or(Bytes::kib() * 256);

        let style = ProgressStyle::default_bar()
          .template("{spinner:.green} ⟪{elapsed_precise}⟫ {binary_bytes} ⟨{binary_bytes_per_sec}⟩")
          .tick_chars(consts::TICK_CHARS);

        let progress_bar = ProgressBar::new_spinner().with_style(style);

        let name = create
          .name
          .clone()
          .ok_or_else(|| Error::internal("Expected `--name` to be set when `--input -`."))?;

        let output = create
          .output
          .clone()
          .ok_or_else(|| Error::internal("Expected `--output` to be set when `--input -`."))?;

        Ok(Self {
          files,
          piece_length,
          progress_bar,
          name,
          output,
        })
      }
    }
  }

  fn torrent_path(input: &Path, name: &str) -> PathBuf {
    input.join("..").lexiclean().join(format!("{name}.torrent"))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use pretty_assertions::assert_eq;

  #[test]
  fn torrent_path() {
    fn case(path: &str, name: &str, expected: impl AsRef<Path>) {
      let expected = expected.as_ref();
      assert_eq!(
        CreateContent::torrent_path(Path::new(path), name),
        expected,
        "{} + {} != {}",
        path,
        name,
        expected.display(),
      );
    }

    use path::Component;

    case("foo", "foo", "foo.torrent");
    case("foo", "foo", "foo.torrent");
    case("foo", "bar", "bar.torrent");
    case("foo/bar", "foo", Path::new("foo").join("foo.torrent"));
    case("foo/bar", "bar", Path::new("foo").join("bar.torrent"));
    case(
      "/foo/bar",
      "bar",
      Path::new(&Component::RootDir)
        .join("foo")
        .join("bar.torrent"),
    );
    case(".", "foo", Path::new("..").join("foo.torrent"));
    case("..", "foo", Path::new("..").join("..").join("foo.torrent"));
  }
}
