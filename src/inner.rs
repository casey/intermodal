use crate::common::*;

pub(crate) fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
  S: Serializer,
  T: Serialize,
{
  value.as_ref().unwrap().serialize(serializer)
}

pub(crate) fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
  D: Deserializer<'de>,
  T: Deserialize<'de>,
{
  Ok(Some(T::deserialize(deserializer)?))
}

#[cfg(test)]
mod tests {
  use super::*;

  use serde::{de::DeserializeOwned, Deserialize, Serialize};

  use std::fmt::Debug;

  fn case<T>(value: T, expected: impl AsRef<[u8]>)
  where
    T: Serialize + DeserializeOwned + PartialEq + Debug,
  {
    let serialized = bendy::serde::ser::to_bytes(&value).unwrap();
    assert_eq!(serialized, expected.as_ref());

    let deserialized = bendy::serde::de::from_bytes(&serialized).unwrap();
    assert_eq!(value, deserialized);
  }

  #[test]
  fn serialize() {
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Foo {
      #[serde(skip_serializing_if = "Option::is_none", default, with = "super")]
      pub(crate) bar: Option<u8>,
    }

    let none = Foo { bar: None };
    case(none, b"de");

    let some = Foo { bar: Some(1) };
    case(some, b"d3:bari1ee");
  }
}
