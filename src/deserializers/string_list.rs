use serde::{
  de::{value::SeqAccessDeserializer, SeqAccess, Visitor},
  Deserialize, Deserializer,
};

pub fn string_list_deserializer<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(StringListVisitor)
}

pub fn option_string_list_deserializer<'de, D>(
  deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(OptionStringListVisitor)
}

struct StringListVisitor;

impl<'de> Visitor<'de> for StringListVisitor {
  type Value = Vec<String>;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(formatter, "string or Vec<String>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(parse_string_list(v))
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    Vec::<String>::deserialize(SeqAccessDeserializer::new(seq))
  }
}

struct OptionStringListVisitor;

impl<'de> Visitor<'de> for OptionStringListVisitor {
  type Value = Option<Vec<String>>;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(formatter, "null or string or Vec<String>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    StringListVisitor.visit_str(v).map(Some)
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: SeqAccess<'de>,
  {
    StringListVisitor.visit_seq(seq).map(Some)
  }

  fn visit_none<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }

  fn visit_unit<E>(self) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(None)
  }
}

/// Parses a list of strings from a comment seperated and multiline string
///
/// Example source:
/// ```text
/// # supports comments
/// path/to/file1 # comment1
/// path/to/file2
///
/// # also supports comma seperated values
/// path/to/file3,path/to/file4
/// ```
///
/// Returns:
/// ```text
/// ["path/to/file1", "path/to/file2", "path/to/file3", "path/to/file4"]
/// ```
fn parse_string_list(source: impl AsRef<str>) -> Vec<String> {
  source
    .as_ref()
    .split('\n')
    .map(str::trim)
    .filter(|line| !line.is_empty() && !line.starts_with('#'))
    .filter_map(|line| line.split(" #").next())
    .flat_map(|line| line.split(','))
    .map(str::trim)
    .filter(|entry| !entry.is_empty())
    .map(str::to_string)
    .collect()
}
