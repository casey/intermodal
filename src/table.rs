use crate::common::*;

pub(crate) struct Table {
  rows: Vec<(&'static str, Value)>,
}

impl Table {
  pub(crate) fn new() -> Self {
    Self { rows: Vec::new() }
  }

  pub(crate) fn row(&mut self, name: &'static str, value: impl ToString) {
    self.rows.push((name, Value::Scalar(value.to_string())));
  }

  pub(crate) fn tiers(
    &mut self,
    name: &'static str,
    tiers: impl IntoIterator<Item = (impl ToString, impl IntoIterator<Item = impl ToString>)>,
  ) {
    self.rows.push((
      name,
      Value::Tiers(
        tiers
          .into_iter()
          .map(|(name, values)| {
            (
              name.to_string(),
              values.into_iter().map(|value| value.to_string()).collect(),
            )
          })
          .collect(),
      ),
    ));
  }

  fn rows(&self) -> &[(&'static str, Value)] {
    &self.rows
  }

  pub(crate) fn name_width(&self) -> usize {
    self
      .rows()
      .iter()
      .map(|row| UnicodeWidthStr::width(row.0))
      .max()
      .unwrap_or(0)
  }

  pub(crate) fn write_human_readable(&self, out: &mut dyn Write) -> io::Result<()> {
    fn padding(out: &mut dyn Write, n: usize) -> io::Result<()> {
      write!(out, "{:width$}", "", width = n)
    }

    let name_width = self.name_width();

    for (name, value) in self.rows() {
      write!(out, "{:>width$}", name, width = name_width)?;

      match value {
        Value::Scalar(value) => writeln!(out, "  {}", value)?,
        Value::Tiers(tiers) => {
          let tier_name_width = tiers
            .iter()
            .map(|(name, _values)| UnicodeWidthStr::width(name.as_str()))
            .max()
            .unwrap_or(0);

          for (i, (name, values)) in tiers.iter().enumerate() {
            if i > 0 {
              padding(out, name_width)?;
            }

            write!(
              out,
              "  {:width$}",
              format!("{}:", name).as_str(),
              width = tier_name_width + 1
            )?;

            for (i, value) in values.iter().enumerate() {
              if i > 0 {
                padding(out, name_width + 2 + tier_name_width + 1)?;
              }

              writeln!(out, " {}", value)?;
            }
          }
        }
      }
    }

    Ok(())
  }
}

enum Value {
  Scalar(String),
  Tiers(Vec<(String, Vec<String>)>),
}

#[cfg(test)]
mod tests {
  use super::*;

  fn human_readable(table: Table, want: &str) {
    let mut cursor = Cursor::new(Vec::new());
    table.write_human_readable(&mut cursor).unwrap();
    let have = String::from_utf8(cursor.into_inner()).unwrap();
    if have != want {
      panic!("have != want:\nHAVE:\n{}\nWANT:\n{}", have, want);
    }
  }

  #[test]
  fn single_row() {
    let mut table = Table::new();
    table.row("Foo", "bar");
    human_readable(table, "Foo  bar\n");
  }

  #[test]
  fn multiple_rows() {
    let mut table = Table::new();
    table.row("Foo", "bar");
    table.row("X", "y");
    human_readable(table, "Foo  bar\n  X  y\n");
  }

  #[test]
  fn tiers_aligned() {
    let mut table = Table::new();
    table.tiers("Foo", vec![("Bar", &["a", "b"]), ("Baz", &["x", "y"])]);
    human_readable(
      table,
      "\
Foo  Bar: a
          b
     Baz: x
          y
",
    );
  }

  #[test]
  fn tiers_unaligned() {
    let mut table = Table::new();
    table.tiers(
      "First",
      vec![("Some", &["the", "thing"]), ("Other", &["about", "that"])],
    );
    table.tiers(
      "Second",
      vec![
        ("Row", &["the", "thing"]),
        ("More Stuff", &["about", "that"]),
      ],
    );
    human_readable(
      table,
      " First  Some:  the
               thing
        Other: about
               that
Second  Row:        the
                    thing
        More Stuff: about
                    that
",
    );
  }
}
