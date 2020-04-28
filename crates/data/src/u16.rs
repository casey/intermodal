use crate::common::*;

impl<'a> Data<'a> for u16 {
  const ALIGNMENT: usize = 2;
  const FIXED_SIZE: usize = 2;

  fn load(buffer: &'a [u8]) -> Result<Self> {
    Self::check(buffer)?;
    Ok(u16::from_le_bytes([buffer[0], buffer[1]]))
  }

  fn store(&self, buffer: &mut [u8]) -> Result<()> {
    Self::check(buffer)?;
    let bytes = self.to_le_bytes();
    buffer.copy_from_slice(&bytes);
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn u16_round_trip() -> Result<()> {
    0u16.round_trip(&Buffer::new::<u16>(&[0, 0]))?;
    1u16.round_trip(&Buffer::new::<u16>(&[1, 0]))?;
    255u16.round_trip(&Buffer::new::<u16>(&[255, 0]))?;
    256u16.round_trip(&Buffer::new::<u16>(&[0, 1]))?;
    u16::max_value().round_trip(&Buffer::new::<u16>(&[0xFF, 0xFF]))?;
    Ok(())
  }

  #[test]
  fn u16_errors() {
    u16::error_test(&mut Buffer::new::<u16>(&[0, 0]));
  }
}
