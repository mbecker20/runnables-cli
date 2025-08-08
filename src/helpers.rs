use std::{
  env,
  io::Read,
  path::{Path, PathBuf},
};

use anyhow::Context;
use path_clean::PathClean;

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

pub fn split_match_strings(search: &str, target: &str, extras: &[&str]) -> bool {
  if search.split(' ').all(|term| target.contains(term)) {
    return true;
  }
  for extra in extras {
    if search.split(' ').all(|term| extra.contains(term)) {
      return true;
    }
  }
  false
}
