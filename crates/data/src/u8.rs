use crate::common::*;

impl<'a> Data<'a> for u8 {
  const ALIGNMENT: usize = 1;
  const FIXED_SIZE: usize = 1;

  fn load(buffer: &'a [u8]) -> Result<Self> {
    Self::check(buffer)?;
    Ok(buffer[0])
  }

  fn store(&self, buffer: &mut [u8]) -> Result<()> {
    Self::check(buffer)?;
    buffer[0] = *self;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn u8_round_trip() -> Result<()> {
    0u8.round_trip(&[0])?;
    1u8.round_trip(&[1])?;
    255u8.round_trip(&[255])?;
    Ok(())
  }

  #[test]
  fn u8_errors() {
    u8::error_test(&mut Buffer::new::<u8>(&[0]));
  }
}
