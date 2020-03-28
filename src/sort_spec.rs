use crate::common::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct SortSpec {
  key: SortKey,
  order: SortOrder,
}

impl SortSpec {
  pub(crate) fn compare(specs: &[SortSpec], a: &FileInfo, b: &FileInfo) -> Ordering {
    let mut specs = specs.to_vec();

    specs.push(SortSpec::default());

    Self::compare_specs(&specs, a, b)
  }

  fn compare_specs(specs: &[SortSpec], a: &FileInfo, b: &FileInfo) -> Ordering {
    specs.iter().fold(Ordering::Equal, |ordering, spec| {
      ordering.then_with(|| spec.compare_file_info(a, b))
    })
  }

  fn compare_file_info(self, a: &FileInfo, b: &FileInfo) -> Ordering {
    let ordering = match self.key {
      SortKey::Path => a.path.cmp(&b.path),
      SortKey::Size => a.length.cmp(&b.length),
    };

    match self.order {
      SortOrder::Ascending => ordering,
      SortOrder::Descending => ordering.reverse(),
    }
  }
}

impl Default for SortSpec {
  fn default() -> Self {
    Self {
      key: SortKey::Path,
      order: SortOrder::default(),
    }
  }
}

impl FromStr for SortSpec {
  type Err = strum::ParseError;

  fn from_str(text: &str) -> Result<Self, Self::Err> {
    if let Some(index) = text.find(':') {
      Ok(SortSpec {
        key: text[..index].parse()?,
        order: text[index + 1..].parse()?,
      })
    } else {
      Ok(SortSpec {
        key: text.parse()?,
        order: SortOrder::default(),
      })
    }
  }
}

impl Display for SortSpec {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}:{}", self.key.name(), self.order.name())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn default() {
    assert_eq!(
      SortSpec::default(),
      SortSpec {
        key: SortKey::Path,
        order: SortOrder::Ascending
      }
    );
  }

  #[test]
  fn parse() {
    assert_eq!(
      SortSpec {
        key: SortKey::Path,
        order: SortOrder::Ascending
      },
      "path:ascending".parse().unwrap()
    );
  }
}
