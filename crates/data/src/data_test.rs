use crate::common::*;

pub trait DataTest<'a>: Data<'a> + PartialEq + Debug + Default {
  fn round_trip(self, encoded: &'a [u8]) -> Result<()> {
    let mut buffer = Vec::<u8>::with_capacity(Self::FIXED_SIZE);
    buffer.resize(Self::FIXED_SIZE, 0);
    self.store(&mut buffer)?;
    assert_eq!(buffer, encoded);
    let decoded = Self::load(&encoded)?;
    assert_eq!(self, decoded);
    Ok(())
  }

  fn error_test(buffer: &'a mut [u8]) {
    let want = Self::FIXED_SIZE;
    let have = Self::FIXED_SIZE - 1;
    assert_eq!(buffer.len(), want);
    let short_buffer = &mut buffer[..have];
    assert_eq!(
      Self::default().store(short_buffer),
      Err(Error::Size { have, want })
    );
    assert_eq!(Self::load(short_buffer), Err(Error::Size { have, want }));
  }
}

impl<'a, T: Data<'a> + PartialEq + Debug + Default> DataTest<'a> for T {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn round_trip_test() {
    0u8.round_trip(&[1]).ok();
  }
}
