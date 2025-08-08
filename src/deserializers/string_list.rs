use serde::{
  de::{value::SeqAccessDeserializer, Visitor},
  Deserialize, Deserializer,
};

pub fn string_list_deserializer<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(StringListVisitor)
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
