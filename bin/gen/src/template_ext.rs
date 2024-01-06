use crate::common::*;

pub(crate) trait TemplateExt {
  #[throws]
  fn render_newline(&self) -> String;

  #[throws]
  fn render_to(&self, path: impl AsRef<Path>) {
    let path = path.as_ref();
    let text = self.render_newline()?;
    fs::write(path, text).context(error::Filesystem { path })?;
  }
}

impl<T: Template> TemplateExt for T {
  #[throws]
  fn render_newline(&self) -> String {
    let mut text = self
      .render()
      .context(error::TemplateRender)?
      .trim()
      .to_owned();
    text.push('\n');
    text
  }
}
