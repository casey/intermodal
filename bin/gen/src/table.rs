use crate::common::*;

pub(crate) struct Table<R> {
  rows: Vec<R>,
}

impl<R: Row> Table<R> {
  pub(crate) fn new(rows: Vec<R>) -> Table<R> {
    Table { rows }
  }
}

impl<R: Row> Display for Table<R> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let mut rows = vec![R::header().to_vec()];

    for row in &self.rows {
      rows.push(row.entries());
    }

    let mut widths = vec![0; rows[0].len()];

    for row in &rows {
      for (width, entry) in widths.iter_mut().zip(row) {
        *width = entry.len().max(*width);
      }
    }

    for (i, row) in rows.iter().enumerate() {
      if i == 1 {
        for width in &widths {
          write!(f, "|:{:-<width$}:", "", width = width)?;
        }
        writeln!(f, "|")?;
      }

      for (width, entry) in widths.iter().zip(row) {
        write!(f, "| {:<width$} ", entry, width = width)?;
      }

      write!(f, "|")?;

      if i < rows.len() - 1 {
        writeln!(f)?;
      }
    }

    Ok(())
  }
}
