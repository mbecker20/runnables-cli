use std::{
  env,
  io::Read,
  path::{Path, PathBuf},
};

use anyhow::Context;
use path_clean::PathClean;
use serde::{
  de::{value::SeqAccessDeserializer, Visitor},
  Deserialize, Deserializer,
};

pub fn absolute_path(path: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
  let path = path.as_ref();
  let absolute_path = if path.is_absolute() {
    path.to_path_buf()
  } else {
    env::current_dir()
      .context("failed to get current directory from env")?
      .join(path)
  }
  .clean();
  Ok(absolute_path)
}

pub fn runnable_path_display(root_path: &str, path: impl AsRef<Path>) -> anyhow::Result<String> {
  let res = absolute_path(path)?
    .display()
    .to_string()
    .replace(root_path, ".");
  Ok(res)
}

pub fn wait_for_enter() -> anyhow::Result<()> {
  println!("\nPress ENTER to close");
  let buffer = &mut [0u8];
  std::io::stdin()
    .read_exact(buffer)
    .context("failed to read ENTER")?;
  Ok(())
}

pub fn split_match_strings(search: &str, target: &str) -> bool {
  search.split(' ').all(|term| target.contains(term))
}

pub fn string_list_deserializer<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
  D: Deserializer<'de>,
{
  deserializer.deserialize_any(StringListVisitor)
}

struct StringListVisitor;

impl<'de> Visitor<'de> for StringListVisitor {
  type Value = Option<Vec<String>>;

  fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(formatter, "empty or null or string or Vec<String>")
  }

  fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
  where
    E: serde::de::Error,
  {
    Ok(parse_string_list(v).into())
  }

  fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
  where
    A: serde::de::SeqAccess<'de>,
  {
    Vec::<String>::deserialize(SeqAccessDeserializer::new(seq)).map(Some)
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
