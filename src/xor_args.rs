use crate::common::*;

pub(crate) fn xor_args<T: Clone>(
  a_name: &str,
  a: &Option<T>,
  b_name: &str,
  b: &Option<T>,
) -> Result<T> {
  let target = a.as_ref().xor(b.as_ref()).ok_or_else(|| {
    Error::internal(format!(
      "Expected exactly one of the arguments `{a_name}` or `{b_name}` to be set",
    ))
  })?;

  Ok(target.clone())
}
