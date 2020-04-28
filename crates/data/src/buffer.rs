use crate::common::*;

pub(crate) struct Buffer {
  layout: Layout,
  alloc: *mut u8,
}

impl Buffer {
  pub(crate) fn new<'a, T: Data<'a>>(contents: &[u8]) -> Buffer {
    let layout = Layout::from_size_align(T::FIXED_SIZE, T::ALIGNMENT).unwrap();
    let alloc = unsafe { alloc::alloc::alloc_zeroed(layout) };
    let mut buffer = Buffer { layout, alloc };
    buffer.deref_mut().copy_from_slice(contents);
    buffer
  }
}

impl Drop for Buffer {
  fn drop(&mut self) {
    unsafe { alloc::alloc::dealloc(self.alloc, self.layout) };
  }
}

impl Deref for Buffer {
  type Target = [u8];

  fn deref(&self) -> &Self::Target {
    unsafe { core::slice::from_raw_parts(self.alloc, self.layout.size()) }
  }
}

impl DerefMut for Buffer {
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { core::slice::from_raw_parts_mut(self.alloc, self.layout.size()) }
  }
}
