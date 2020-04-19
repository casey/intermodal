use crate::common::*;

pub(crate) trait TemplateExt {
  #[throws]
  fn render_newline(&self) -> String;
}

impl<T: Template> TemplateExt for T {
  #[throws]
  fn render_newline(&self) -> String {
    let mut text = self.render()?.trim().to_owned();
    text.push('\n');
    text
  }
}
