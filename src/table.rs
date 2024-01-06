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

  pub(crate) fn size(&mut self, name: &'static str, bytes: Bytes) {
    self.rows.push((name, Value::Size(bytes)));
  }

  pub(crate) fn list(&mut self, name: &'static str, list: Vec<String>) {
    self.rows.push((name, Value::List(list)));
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

  pub(crate) fn directory(&mut self, name: &'static str, root: &str, mut files: Vec<FilePath>) {
    files.sort();
    self.rows.push((
      name,
      Value::Directory {
        root: root.to_owned(),
        files,
      },
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

  pub(crate) fn write_human_readable(&self, out: &mut dyn Write, style: Style) -> io::Result<()> {
    fn padding(out: &mut dyn Write, n: usize) -> io::Result<()> {
      write!(out, "{:width$}", "", width = n)
    }

    let name_width = self.name_width();

    for (name, value) in self.rows() {
      write!(
        out,
        "{:width$}{}",
        "",
        style.blue().paint(*name),
        width = name_width - UnicodeWidthStr::width(*name),
      )?;

      match value {
        Value::List(list) => {
          for (i, value) in list.iter().enumerate() {
            if i == 0 {
              padding(out, 2)?;
            } else {
              padding(out, name_width + 2)?;
            }
            writeln!(out, "{value}")?;
          }
        }
        Value::Directory { root, files } => {
          let mut tree = Tree::new(root);
          for file in files {
            tree.insert(file.components());
          }
          let lines = tree.lines();

          for (i, (last, name)) in lines.iter().enumerate() {
            if i == 0 {
              write!(out, "  ")?;
            } else {
              write!(out, "{:indent$}  ", "", indent = name_width)?;
            }

            if !last.is_empty() {
              for last in &last[..last.len() - 1] {
                if *last {
                  write!(out, "  ")?;
                } else {
                  write!(out, "│ ")?;
                }
              }
              if last[last.len() - 1] {
                write!(out, "└─")?;
              } else {
                write!(out, "├─")?;
              }
            }

            writeln!(out, "{name}")?;
          }
        }
        Value::Scalar(scalar) => writeln!(out, "  {scalar}")?,
        Value::Size(bytes) => writeln!(out, "  {bytes}")?,
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
              format!("{name}:").as_str(),
              width = tier_name_width + 1
            )?;

            for (i, value) in values.iter().enumerate() {
              if i > 0 {
                padding(out, name_width + 2 + tier_name_width + 1)?;
              }

              writeln!(out, " {value}")?;
            }
          }
        }
      }
    }

    Ok(())
  }

  pub(crate) fn write_tab_delimited(&self, out: &mut dyn Write) -> io::Result<()> {
    for (name, value) in self.rows() {
      write!(out, "{}\t", name.to_lowercase())?;
      match value {
        Value::List(list) => {
          for (i, value) in list.iter().enumerate() {
            if i > 0 {
              write!(out, "\t")?;
            }
            write!(out, "{value}")?;
          }
          writeln!(out)?;
        }
        Value::Directory { root, files } => {
          for (i, file) in files.iter().enumerate() {
            if i > 0 {
              write!(out, "\t")?;
            }
            write!(out, "{root}/{file}")?;
          }
          writeln!(out)?;
        }
        Value::Scalar(scalar) => writeln!(out, "{scalar}")?,
        Value::Size(Bytes(value)) => writeln!(out, "{value}")?,
        Value::Tiers(tiers) => {
          for (i, value) in tiers.iter().flat_map(|(_name, values)| values).enumerate() {
            if i > 0 {
              write!(out, "\t")?;
            }
            write!(out, "{value}")?;
          }
          writeln!(out)?;
        }
      }
    }

    Ok(())
  }
}

enum Value {
  Scalar(String),
  Tiers(Vec<(String, Vec<String>)>),
  Size(Bytes),
  List(Vec<String>),
  Directory { root: String, files: Vec<FilePath> },
}

struct Tree<'name> {
  name: &'name str,
  children: Vec<Tree<'name>>,
}

impl<'name> Tree<'name> {
  fn new(name: &'name str) -> Tree<'name> {
    Self {
      name,
      children: Vec::new(),
    }
  }

  fn insert(&mut self, file: &'name [String]) {
    if file.is_empty() {
      return;
    }

    let head = &file[0];

    for child in &mut self.children {
      if child.name == head {
        child.insert(&file[1..]);
        return;
      }
    }

    let mut child = Self::new(head);
    child.insert(&file[1..]);

    self.children.push(child);
  }

  fn lines(&self) -> Vec<(Vec<bool>, &'name str)> {
    let mut lines = Vec::new();
    let mut last = Vec::new();
    self.lines_inner(&mut last, &mut lines);
    lines
  }

  fn lines_inner(&self, last: &mut Vec<bool>, lines: &mut Vec<(Vec<bool>, &'name str)>) {
    lines.push((last.clone(), self.name));
    last.push(false);
    for (i, child) in self.children.iter().enumerate() {
      if i == self.children.len() - 1 {
        last.pop();
        last.push(true);
      }
      child.lines_inner(last, lines);
    }
    last.pop();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn human_readable(table: &Table, want: &str) {
    let mut cursor = Cursor::new(Vec::new());
    table
      .write_human_readable(&mut cursor, Style::inactive())
      .unwrap();
    let have = String::from_utf8(cursor.into_inner()).unwrap();
    assert_eq!(have, want, "have != want:\nHAVE:\n{have}\nWANT:\n{want}",);
  }

  fn tab_delimited(table: &Table, want: &str) {
    let mut cursor = Cursor::new(Vec::new());
    table.write_tab_delimited(&mut cursor).unwrap();
    let have = String::from_utf8(cursor.into_inner()).unwrap();
    assert_eq!(have, want, "have != want:\nHAVE:\n{have}\nWANT:\n{want}",);
  }

  #[test]
  fn color() {
    let mut table = Table::new();
    table.row("Here", "bar");
    table.row("There", "baz");
    let mut cursor = Cursor::new(Vec::new());
    table
      .write_human_readable(&mut cursor, Style::active())
      .unwrap();
    let have = String::from_utf8(cursor.into_inner()).unwrap();
    assert_eq!(
      have,
      " \u{1b}[34mHere\u{1b}[0m  bar\n\u{1b}[34mThere\u{1b}[0m  baz\n"
    );
  }

  #[test]
  fn directory() {
    let mut table = Table::new();
    table.directory(
      "Files",
      "Foo",
      vec![
        FilePath::from_components(&["a", "b"]),
        FilePath::from_components(&["a", "c"]),
        FilePath::from_components(&["d"]),
      ],
    );
    tab_delimited(&table, "files\tFoo/a/b\tFoo/a/c\tFoo/d\n");
    human_readable(
      &table,
      "\
Files  Foo
       ├─a
       │ ├─b
       │ └─c
       └─d
",
    );
  }

  #[test]
  fn single_row() {
    let mut table = Table::new();
    table.row("Foo", "bar");
    human_readable(&table, "Foo  bar\n");
    tab_delimited(&table, "foo\tbar\n");
  }

  #[test]
  fn multiple_rows() {
    let mut table = Table::new();
    table.row("Foo", "bar");
    table.row("X", "y");
    human_readable(&table, "Foo  bar\n  X  y\n");
    tab_delimited(&table, "foo\tbar\nx\ty\n");
  }

  #[test]
  fn list() {
    let mut table = Table::new();
    table.list("Something", vec!["a".into(), "b".into(), "c".into()]);
    table.list("Other", vec!["x".into(), "y".into(), "z".into()]);
    human_readable(
      &table,
      "\
Something  a
           b
           c
    Other  x
           y
           z
",
    );
    tab_delimited(
      &table,
      "\
something\ta\tb\tc
other\tx\ty\tz
",
    );
  }

  #[test]
  fn tiers_aligned() {
    let mut table = Table::new();
    table.tiers("Foo", vec![("Bar", &["a", "b"]), ("Baz", &["x", "y"])]);
    human_readable(
      &table,
      "\
Foo  Bar: a
          b
     Baz: x
          y
",
    );
    tab_delimited(&table, "foo\ta\tb\tx\ty\n");
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
      &table,
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
    tab_delimited(
      &table,
      "first\tthe\tthing\tabout\tthat\nsecond\tthe\tthing\tabout\tthat\n",
    );
  }
}
