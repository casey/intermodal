use crate::common::*;

pub trait Data<'a>: Sized + 'a {
  const FIXED_SIZE: usize;

  const ALIGNMENT: usize;

  fn load(buffer: &'a [u8]) -> Result<Self>;

  fn store(&self, buffer: &mut [u8]) -> Result<()>;

  fn check(buffer: &'a [u8]) -> Result<()> {
    let have = buffer.len();
    let want = Self::FIXED_SIZE;
    if have < want {
      return Err(Error::Size { have, want });
    }

    let alignment = Self::ALIGNMENT;
    let offset = buffer.as_ptr().align_offset(alignment);
    if offset != 0 {
      return Err(Error::Alignment { alignment, offset });
    }

    Ok(())
  }
}
