use crate::common::*;

use self::Error::*;

#[derive(Clone)]
pub(crate) enum Value<'buffer> {
  Int(&'buffer str),
  List(Vec<Value<'buffer>>),
  Dict(Vec<(&'buffer [u8], Value<'buffer>)>),
  Str(&'buffer [u8]),
}

impl<'buffer> Value<'buffer> {
  pub(crate) fn decode(buffer: &'buffer [u8]) -> Result<Value<'buffer>, Error> {
    Parser::parse(buffer)
  }

  pub(crate) fn encode(&self) -> Vec<u8> {
    let mut buffer = Vec::new();
    self.encode_into(&mut buffer);
    buffer
  }

  pub(crate) fn encode_into(&self, buffer: &mut Vec<u8>) {
    match self {
      Self::Int(value) => {
        buffer.push(b'i');
        buffer.extend_from_slice(value.as_bytes());
        buffer.push(b'e');
      }
      Self::List(values) => {
        buffer.push(b'l');
        for value in values {
          value.encode_into(buffer);
        }
        buffer.push(b'e');
      }
      Self::Dict(items) => {
        buffer.push(b'd');
        for (key, value) in items {
          Self::encode_str(buffer, key);
          value.encode_into(buffer);
        }
        buffer.push(b'e');
      }
      Self::Str(contents) => Self::encode_str(buffer, contents),
    }
  }

  fn encode_str(buffer: &mut Vec<u8>, contents: &[u8]) {
    buffer.extend_from_slice(contents.len().to_string().as_bytes());
    buffer.push(b':');
    buffer.extend_from_slice(contents);
  }

  fn fmt_str(f: &mut Formatter, contents: &[u8]) -> fmt::Result {
    if let Ok(text) = str::from_utf8(contents) {
      write!(f, "\"{}\"", text)
    } else {
      write!(f, "<")?;
      for byte in contents {
        write!(f, "{:X}", byte)?;
      }
      write!(f, ">")
    }
  }
}

impl<'buffer> Display for Value<'buffer> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    match self {
      Self::Int(value) => write!(f, "{}", value),
      Self::List(values) => {
        write!(f, "[")?;
        for (i, value) in values.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          write!(f, "{}", value)?;
        }
        write!(f, "]")
      }
      Self::Dict(items) => {
        write!(f, "{{")?;
        for (i, (key, value)) in items.iter().enumerate() {
          if i > 0 {
            write!(f, ", ")?;
          }
          Value::fmt_str(f, key)?;
          write!(f, ": {}", value)?;
        }
        write!(f, "}}")
      }
      Self::Str(contents) => Value::fmt_str(f, contents),
    }
  }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
  TrailingData { start: usize },
  UnexpectedEndOfBuffer,
  UnexpectedByte { found: u8 },
  UnsortedKey,
  DuplicateKey,
  EmptyInteger,
  NegativeZero,
  LeadingZero,
}

pub(crate) struct Parser<'buffer> {
  index: usize,
  buffer: &'buffer [u8],
}

impl<'buffer> Parser<'buffer> {
  pub(crate) fn parse(buffer: &'buffer [u8]) -> Result<Value<'buffer>, Error> {
    let parser = Parser { index: 0, buffer };

    Ok(parser.root()?)
  }

  fn root(mut self) -> Result<Value<'buffer>, Error> {
    let root = self.value()?;

    if self.index != self.buffer.len() {
      return Err(TrailingData { start: self.index });
    }

    Ok(root)
  }

  fn value(&mut self) -> Result<Value<'buffer>, Error> {
    match self.next()? {
      b'i' => self.int(),
      b'l' => self.list(),
      b'd' => self.dict(),
      b'0'..=b'9' => self.string(),
      found => Err(UnexpectedByte { found }),
    }
  }

  fn extract_digits(&mut self) -> Result<&'buffer [u8], Error> {
    let start = self.index;

    while let b'0'..=b'9' = self.next()? {
      self.advance()?;
    }

    Ok(&self.buffer[start..self.index])
  }

  fn parse_digits(digits: &[u8]) -> Result<u64, Error> {
    if digits.is_empty() {
      return Err(EmptyInteger);
    }

    if digits == b"0" {
      return Ok(0);
    }

    let mut i = 0;

    for digit in digits {
      let value: u64 = (digit - b'0').into();

      if value == 0 && i == 0 {
        return Err(LeadingZero);
      }

      i = i * 10 + value;
    }

    Ok(i)
  }

  fn int(&mut self) -> Result<Value<'buffer>, Error> {
    self.expect(b'i')?;

    let start = self.index;

    let negative = self.accept(b'-')?;

    let digits = self.extract_digits()?;

    let end = self.index;

    self.expect(b'e')?;

    let value = Self::parse_digits(digits)?;

    if value == 0 && negative {
      return Err(NegativeZero);
    }

    Ok(Value::Int(
      str::from_utf8(&self.buffer[start..end]).unwrap(),
    ))
  }

  fn list(&mut self) -> Result<Value<'buffer>, Error> {
    self.expect(b'l')?;

    let mut values = Vec::new();

    while self.next()? != b'e' {
      values.push(self.value()?);
    }

    self.expect(b'e')?;

    Ok(Value::List(values))
  }

  fn dict(&mut self) -> Result<Value<'buffer>, Error> {
    self.expect(b'd')?;

    let mut values: Vec<(&[u8], Value)> = Vec::new();

    while self.next()? != b'e' {
      let key = self.key()?;

      if let Some((last_key, _)) = values.last() {
        match key.cmp(last_key) {
          Ordering::Equal => return Err(DuplicateKey),
          Ordering::Less => return Err(UnsortedKey),
          Ordering::Greater => {}
        }
      }

      let value = self.value()?;

      values.push((key, value));
    }

    self.expect(b'e')?;

    Ok(Value::Dict(values))
  }

  fn string(&mut self) -> Result<Value<'buffer>, Error> {
    Ok(Value::Str(self.key()?))
  }

  fn key(&mut self) -> Result<&'buffer [u8], Error> {
    let digits = self.extract_digits()?;

    self.expect(b':')?;

    let len = Self::parse_digits(digits)?;

    let start = self.index;
    for _ in 0..len {
      self.advance()?;
    }

    Ok(&self.buffer[start..self.index])
  }

  fn next(&self) -> Result<u8, Error> {
    self
      .buffer
      .get(self.index)
      .cloned()
      .ok_or(UnexpectedEndOfBuffer)
  }

  fn advance(&mut self) -> Result<(), Error> {
    if self.index == self.buffer.len() {
      Err(UnexpectedEndOfBuffer)
    } else {
      self.index += 1;
      Ok(())
    }
  }

  fn expect(&mut self, expected: u8) -> Result<(), Error> {
    let found = self.next()?;

    if found != expected {
      return Err(UnexpectedByte { found });
    }

    self.advance()?;

    Ok(())
  }

  fn accept(&mut self, acceptable: u8) -> Result<bool, Error> {
    if self.next()? == acceptable {
      self.advance()?;
      Ok(true)
    } else {
      Ok(false)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn err(input: impl AsRef<[u8]>, expected: Error) {
    let buffer = input.as_ref();
    let text = String::from_utf8_lossy(buffer);
    match Parser::parse(buffer) {
      Ok(_) => panic!(
        "Input `{}` passed validation, expected: {:?}",
        text, expected,
      ),
      Err(error) => assert_eq!(error, expected, "Unexpected error for input `{}`", text),
    }
  }

  fn ok(input: impl AsRef<[u8]>) {
    let buffer = input.as_ref();
    match Value::decode(buffer) {
      Err(_) => {
        panic!(
          "Input failed to validate: `{}`",
          String::from_utf8_lossy(buffer)
        );
      }
      Ok(value) => {
        let round_trip = value.encode();
        assert_eq!(round_trip, buffer);
      }
    }
  }

  #[test]
  fn misc() {
    err("", UnexpectedEndOfBuffer);
    err("i20efoo", TrailingData { start: 4 });
    err("defoo", TrailingData { start: 2 });
    err("lefoo", TrailingData { start: 2 });
    err("1:afoo", TrailingData { start: 3 });
  }

  #[test]
  fn int() {
    err("ie", EmptyInteger);
    err("i-0e", NegativeZero);
    err("i00e", LeadingZero);
    err("iae", UnexpectedByte { found: b'a' });
    ok("i0e");
    ok("i-100e");
  }

  #[test]
  fn list() {
    ok("le");
    ok("llelelee");
    ok("li20ee");
    ok("li20edelee");
  }

  #[test]
  fn dict() {
    ok("de");
    ok("d0:0:e");
    err("di0elee", UnexpectedByte { found: b'i' });
    err("d0:i0ei0ei0ee", UnexpectedByte { found: b'i' });
    err("d0:e", UnexpectedByte { found: b'e' });
    err("d0:de0:dee", DuplicateKey);
    err("d1:ade0:dee", UnsortedKey);
    err("d1:ade0:dee", UnsortedKey);
    ok("d1:ade1:bde1:cdee");
  }

  #[test]
  fn string() {
    ok("0:");
    ok("5:hello");
    err("1:", UnexpectedEndOfBuffer);
    err("2:a", UnexpectedEndOfBuffer);
  }
}
