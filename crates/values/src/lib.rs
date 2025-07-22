use serde::{Deserialize, Serialize};

/// An enum of all the ways we can interpret values.
#[derive(Clone, Serialize, Deserialize)]
pub struct Value {
  contents: ValueContents,
  size:     u64,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ValueContents {
  MessagePack(serde_json::Value),
  Json(serde_json::Value),
  String(String),
  Bytes(Vec<u8>),
}

impl Value {
  pub fn pretty(&self) -> String {
    match &self.contents {
      ValueContents::MessagePack(v) => serde_json::to_string(v).unwrap(),
      ValueContents::Json(v) => serde_json::to_string(v).unwrap(),
      ValueContents::String(s) => format!("\"{}\"", s),
      ValueContents::Bytes(b) => format!("{:#x}", hex_fmt::HexFmt(b)),
    }
  }

  pub fn pretty_long(&self) -> String {
    match &self.contents {
      ValueContents::MessagePack(v) => serde_json::to_string_pretty(v).unwrap(),
      ValueContents::Json(v) => serde_json::to_string_pretty(v).unwrap(),
      ValueContents::String(s) => format!("\"{}\"", s),
      ValueContents::Bytes(b) => format!("{:#x}", hex_fmt::HexFmt(b)),
    }
  }

  pub fn size(&self) -> u64 { self.size }

  pub fn size_pretty(&self) -> String {
    const KB: u64 = 1000;
    const MB: u64 = 1000 * KB;
    const GB: u64 = 1000 * MB;

    match self.size {
      size if size < KB => format!("{size} B"),
      size if size < MB => format!("{:.2} KB", size as f64 / KB as f64),
      size if size < GB => {
        format!("{:.2} MB", size as f64 / MB as f64)
      }
      size => format!("{:.2} GB", size as f64 / GB as f64),
    }
  }

  pub fn contents(&self) -> &ValueContents { &self.contents }
}

impl From<Vec<u8>> for Value {
  fn from(bytes: Vec<u8>) -> Self {
    let size = bytes.len() as _;

    // check all the possible representations
    if let Ok(value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
      return Value {
        contents: ValueContents::Json(value),
        size,
      };
    }

    // try to convert to a string
    if let Ok(string) = String::from_utf8(bytes.clone()) {
      return Value {
        contents: ValueContents::String(string),
        size,
      };
    }

    if let Ok(value) = rmp_serde::from_slice::<serde_json::Value>(&bytes) {
      return Value {
        contents: ValueContents::MessagePack(value),
        size,
      };
    }

    Value {
      contents: ValueContents::Bytes(bytes),
      size,
    }
  }
}
